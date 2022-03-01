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

    pub fn get_version(&self) -> u8 {
        match self {
            Packet::Literal(lit) => lit.version,
            Packet::Operator(op) => op.version,
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
    sub_packets: Vec<Packet>,
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
    let mut version = 0u8;
    for input_char in input.chars().take(3) {
        version <<= 1u8;
        if input_char == '1' {
            version += 1u8;
        }
    }
    let mut type_id = 0u8;
    for input_char in input.chars().skip(3).take(3) {
        type_id <<= 1u8;
        if input_char == '1' {
            type_id += 1u8;
        }
    }

    if type_id == 4u8 {
        Packet::Literal(parse_literal(&input[6..], version))
    } else {
        Packet::Operator(parse_operator(&input[6..], version))
    }
}

fn parse_literal(input: &str, version: u8) -> LiteralPayload {
    let mut final_value = 0u64;
    let mut last_chunk = 0usize;
    for (chunk_idx, chunk) in input
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

fn parse_operator(input: &str, version: u8) -> OperatorPayload {
    let size_in_chars = input.starts_with('0');
    let size_displacement = if size_in_chars { 15usize } else { 11usize };
    let mut remaining_data = usize::from_str_radix(&input[1..size_displacement + 1], 2).unwrap();

    let mut sub_packets: Vec<Packet> = Vec::new();

    while remaining_data > 0usize {
        let sub_size: usize = sub_packets.iter().map(Packet::get_size).sum();
        let sub_start = sub_size + size_displacement;

        let sub_package = parse_packet(&input[sub_start + 1..]);

        remaining_data -= if size_in_chars {
            sub_package.get_size()
        } else {
            1
        };

        sub_packets.push(sub_package);
    }

    let size: usize =
        sub_packets.iter().map(Packet::get_size).sum::<usize>() + size_displacement + 7usize;

    OperatorPayload {
        version,
        sub_packets,
        size,
    }
}

fn sum_all_versions(root: &Packet) -> u64 {
    let mut exploration_stack: Vec<&Packet> = vec![root];

    let mut version_sum = 0u64;

    while let Some(next_operator) = exploration_stack.pop() {
        version_sum += u64::from(next_operator.get_version());
        if let Packet::Operator(op) = next_operator {
            exploration_stack.extend(op.sub_packets.iter());
        };
    }

    version_sum
}

pub fn part1(input: &str) {
    let input_binary = hex_payload_to_binary(input);
    let parsed_package = parse_packet(&input_binary);
    let version_sum = sum_all_versions(&parsed_package);

    println!("Sum of all the version numbers: {}", version_sum);
}

pub fn _part2(_input: &str) {
    // let risk_grid = RiskGrid::new(input);
    // let start: Point = (0, 0);
    // let end: Point = (risk_grid.rows - 1, risk_grid.columns - 1);
    // let start_grid_coord: GridCoord = (0u8, 0u8);
    // let end_grid_coord: GridCoord = (4u8, 4u8);
    // println!("Lowest risk path sum: {}", risk_grid.find_lowest_risk_path(&(start, start_grid_coord), &(end, end_grid_coord)));
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

        let sub_packets = vec![
            Packet::Literal(LiteralPayload {
                version: 6u8,
                value: 10u64,
                size: 11usize,
            }),
            Packet::Literal(LiteralPayload {
                version: 2u8,
                value: 20u64,
                size: 16usize,
            }),
        ];

        let comparison_operator = Packet::Operator(OperatorPayload {
            version: 1u8,
            sub_packets,
            size: 49usize,
        });

        assert_eq!(parse_packet(&input_binary), comparison_operator);
    }

    #[test]
    fn basic_operator_sub_size() {
        let input_binary = hex_payload_to_binary("EE00D40C823060");
        // 111 011 1 00000000011 010 100 00001 100 100 00010 001 100 00011 00000
        // VVV TTT I LLLLLLLLLLL AAA AAA AAAAA BBB BBB BBBBB CCC CCC CCCCC XXXXX

        let sub_packets = vec![
            Packet::Literal(LiteralPayload {
                version: 2u8,
                value: 1u64,
                size: 11usize,
            }),
            Packet::Literal(LiteralPayload {
                version: 4u8,
                value: 2u64,
                size: 11usize,
            }),
            Packet::Literal(LiteralPayload {
                version: 1u8,
                value: 3u64,
                size: 11usize,
            }),
        ];

        let comparison_operator = Packet::Operator(OperatorPayload {
            version: 7u8,
            sub_packets,
            size: 51usize,
        });

        assert_eq!(parse_packet(&input_binary), comparison_operator);
    }

    #[test]
    fn version_sum() {
        let input_binary = hex_payload_to_binary("8A004A801A8002F478");
        let operator = parse_packet(&input_binary);

        assert_eq!(sum_all_versions(&operator), 16u64);

        let input_binary = hex_payload_to_binary("620080001611562C8802118E34");
        let operator = parse_packet(&input_binary);

        assert_eq!(sum_all_versions(&operator), 12u64);

        let input_binary = hex_payload_to_binary("C0015000016115A2E0802F182340");
        let operator = parse_packet(&input_binary);

        assert_eq!(sum_all_versions(&operator), 23u64);

        let input_binary = hex_payload_to_binary("A0016C880162017C3686B18A3D4780");
        let operator = parse_packet(&input_binary);

        assert_eq!(sum_all_versions(&operator), 31u64);
    }
}
