use std::iter::FromIterator;

use itertools::Itertools;

#[derive(Eq, PartialEq, Debug)]
enum Packet {
    Literal(LiteralPayload),
    Operator(OperatorPayload),
}

impl Packet {
    pub fn get_size(&self) -> usize {
        match self {
            Packet::Literal(lit) => lit.size,
            Packet::Operator(op) => op.size,
        }
    }

    pub fn get_total_version(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.version.into(),
            Packet::Operator(op) => op.total_version,
        }
    }

    pub fn get_value(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.value,
            Packet::Operator(op) => op.value,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct LiteralPayload {
    version: u8,
    value: u64,
    size: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct OperatorPayload {
    version: u8,
    value: u64,
    total_version: u64,
    size: usize,
}

fn hex_payload_to_binary(input: &str) -> String {
    let mut binary_payload: String = String::new();
    for c in input
        .to_uppercase()
        .chars()
        .take_while(|c| !c.is_whitespace())
    {
        let binary = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => {
                panic!("Provided a value that's not an hexadecimal digit")
            }
        };
        binary_payload.push_str(binary);
    }
    binary_payload
}

fn parse_packet(input: &str) -> Packet {
    let type_string: String = input.chars().skip(3).take(3).collect();
    let type_id = u8::from_str_radix(&type_string, 2).unwrap();

    if type_id == 4u8 {
        Packet::Literal(parse_literal(input))
    } else {
        Packet::Operator(parse_operator(input))
    }
}

fn parse_literal(input: &str) -> LiteralPayload {
    let version_string: String = input.chars().take(3).collect();
    let version = u8::from_str_radix(&version_string, 2).unwrap();
    let mut final_value = 0u64;
    let mut last_chunk = 0usize;
    for (chunk_idx, chunk) in input[6..]
        .chars()
        .chunks(5)
        .into_iter()
        .map(String::from_iter)
        .enumerate()
    {
        let chunk_value = u64::from_str_radix(&chunk, 2).unwrap();
        let value: u64 = chunk_value & !0b10000;
        final_value <<= 4;
        final_value += value;
        if chunk_value & 0b10000 == 0 {
            last_chunk = chunk_idx + 1;
            break;
        }
    }

    LiteralPayload {
        version,
        value: final_value,
        size: 6usize + last_chunk * 5usize,
    }
}

fn parse_operator(input: &str) -> OperatorPayload {
    let version_string: String = input.chars().take(3).collect();
    let version = u8::from_str_radix(&version_string, 2).unwrap();

    let actual_input = &input[6..];
    let size_in_chars = actual_input.starts_with('0');
    let size_displacement = if size_in_chars { 15usize } else { 11usize };
    let mut remaining_data =
        usize::from_str_radix(&actual_input[1..size_displacement + 1], 2).unwrap();

    let mut sub_packets: Vec<Packet> = Vec::new();

    while remaining_data > 0usize {
        let sub_size: usize = sub_packets.iter().map(Packet::get_size).sum();
        let sub_start = sub_size + size_displacement;

        let sub_package = parse_packet(&actual_input[sub_start + 1..]);

        remaining_data -= if size_in_chars {
            sub_package.get_size()
        } else {
            1
        };

        sub_packets.push(sub_package);
    }

    let total_version: u64 = sub_packets
        .iter()
        .map(Packet::get_total_version)
        .sum::<u64>()
        + u64::from(version);

    let size: usize =
        sub_packets.iter().map(Packet::get_size).sum::<usize>() + size_displacement + 7usize;

    let op_string: String = input.chars().skip(3).take(3).collect();
    let op_id = u8::from_str_radix(&op_string, 2).unwrap();
    let value: u64 = match op_id {
        0u8 => sub_packets.iter().map(Packet::get_value).sum(),
        1u8 => sub_packets.iter().map(Packet::get_value).product(),
        2u8 => sub_packets.iter().map(Packet::get_value).min().unwrap(),
        3u8 => sub_packets.iter().map(Packet::get_value).max().unwrap(),
        5u8 => (sub_packets[0].get_value() > sub_packets[1].get_value()) as u64,
        6u8 => (sub_packets[0].get_value() < sub_packets[1].get_value()) as u64,
        7u8 => (sub_packets[0].get_value() == sub_packets[1].get_value()) as u64,
        _ => {
            panic!("Invalid operation id {}", op_id)
        }
    };

    OperatorPayload {
        version,
        value,
        total_version,
        size,
    }
}

pub fn part1(input: &str) {
    let input_binary = hex_payload_to_binary(input);
    let parsed_package = parse_packet(&input_binary);

    println!(
        "Sum of all the version numbers: {}",
        parsed_package.get_total_version()
    );
}

pub fn part2(input: &str) {
    let input_binary = hex_payload_to_binary(input);
    let parsed_package = parse_packet(&input_binary);

    println!("Computed value: {}", parsed_package.get_value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_conversion() {
        let input_string = "D2FE28";

        assert_eq!(
            hex_payload_to_binary(input_string),
            "110100101111111000101000"
        )
    }

    #[test]
    fn literal_parse() {
        let input_string = "D2FE28";
        let input_binary = hex_payload_to_binary(input_string);

        let comparison_packet = Packet::Literal(LiteralPayload {
            version: 6u8,
            value: 2021u64,
            size: 21usize,
        });

        assert_eq!(comparison_packet, parse_packet(&input_binary));
    }

    #[test]
    fn basic_operator_char_size() {
        let input_binary = hex_payload_to_binary("38006F45291200");
        // 001 110 0 000000000011011 110 100 01010 010 100 10001 00100 0000000
        // VVV TTT I LLLLLLLLLLLLLLL AAA AAA AAAAA BBB BBB BBBBB BBBBB XXXXXXX

        let comparison_operator = Packet::Operator(OperatorPayload {
            version: 1u8,
            total_version: 9u64,
            value: 1u64,
            size: 49usize,
        });

        assert_eq!(parse_packet(&input_binary), comparison_operator);
    }

    #[test]
    fn basic_operator_sub_size() {
        let input_binary = hex_payload_to_binary("EE00D40C823060");
        // 111 011 1 00000000011 010 100 00001 100 100 00010 001 100 00011 00000
        // VVV TTT I LLLLLLLLLLL AAA AAA AAAAA BBB BBB BBBBB CCC CCC CCCCC XXXXX

        let comparison_operator = Packet::Operator(OperatorPayload {
            version: 7u8,
            total_version: 14u64,
            value: 3u64,
            size: 51usize,
        });

        assert_eq!(parse_packet(&input_binary), comparison_operator);
    }

    #[test]
    fn version_sum() {
        let input_binary = hex_payload_to_binary("8A004A801A8002F478");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_total_version(), 16u64);

        let input_binary = hex_payload_to_binary("620080001611562C8802118E34");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_total_version(), 12u64);

        let input_binary = hex_payload_to_binary("C0015000016115A2E0802F182340");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_total_version(), 23u64);

        let input_binary = hex_payload_to_binary("A0016C880162017C3686B18A3D4780");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_total_version(), 31u64);
    }

    #[test]
    fn compute_operators_results() {
        let input_binary = hex_payload_to_binary("C200B40A82");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 3u64);

        let input_binary = hex_payload_to_binary("04005AC33890");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 54u64);

        let input_binary = hex_payload_to_binary("880086C3E88112");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 7u64);

        let input_binary = hex_payload_to_binary("CE00C43D881120");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 9u64);

        let input_binary = hex_payload_to_binary("D8005AC2A8F0");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 1u64);

        let input_binary = hex_payload_to_binary("F600BC2D8F");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 0u64);

        let input_binary = hex_payload_to_binary("9C005AC2F8F0");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 0u64);

        let input_binary = hex_payload_to_binary("9C0141080250320F1802104A08");
        let operator = parse_packet(&input_binary);

        assert_eq!(operator.get_value(), 1u64);
    }
}
