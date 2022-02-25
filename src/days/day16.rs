use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::iter::FromIterator;

use itertools::Itertools;

enum Packet {
    Literal(LiteralPayload),
    Operator(OperatorPayload),
}

struct LiteralPayload {
    version: u8,
    value: u64,
}

struct OperatorPayload {
    version: u8,
    sub_packets: Vec<Packet>,
}

fn hex_payload_to_binary(input: &str) -> String {
    let mut binary_payload: String = String::new();
    for c in input.to_uppercase().chars() {
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
    return binary_payload;
}

fn parse_packet(input: &str) -> Packet {
    let mut input_chars = input.char_indices();
    let mut version = 0u8;
    for _ in 0..3 {
        version <<= 1u8;
        if input_chars.next().unwrap().1 == '1' {
            version += 1u8;
        }
    }
    let mut type_id = 0u8;
    for _ in 0..3 {
        type_id <<= 1u8;
        if input_chars.next().unwrap().1 == '1' {
            type_id += 1u8;
        }
    }

    let (current_byte, _) = input_chars.next().unwrap();

    println!(
        "Current packet version: {}; Type ID: {}; Remaining Payload: {}",
        version,
        type_id,
        &input[current_byte..]
    );

    if type_id == 4u8 {
        Packet::Literal(parse_literal(&input[current_byte..], version))
    } else {
        Packet::Operator(parse_operator(&input[current_byte..], version))
    }
}

fn parse_literal(input: &str, version: u8) -> LiteralPayload {
    let mut final_value = 0u64;
    for chunk in input.chars().chunks(5).into_iter().map(String::from_iter) {
        if chunk.len() == 5 {
            let chunk_value = u64::from_str_radix(&chunk, 2).unwrap();
            let value: u64 = chunk_value & !0b10000;
            println!("Parsing {}, which is {}", chunk, value);
            final_value <<= 4;
            final_value += value;
            println!("Current final value: {}", final_value);
        }
    }

    LiteralPayload {
        version,
        value: final_value,
    }
}

fn parse_operator(input: &str, version: u8) -> OperatorPayload {
    OperatorPayload {
        version: 0u8,
        sub_packets: Vec::new(),
    }
}

pub fn part1(input: &str) {
    // let risk_grid = RiskGrid::new(input);
    // let start: Point = (0, 0);
    // let end: Point = (risk_grid.rows - 1, risk_grid.columns - 1);
    // let grid_coord: GridCoord = (0u8, 0u8);
    // println!("Lowest risk path sum: {}", risk_grid.find_lowest_risk_path(&(start, grid_coord), &(end, grid_coord)));
}

pub fn part2(input: &str) {
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

        if let Packet::Literal(parsed_literal) = parse_packet(&input_binary) {
            assert_eq!(parsed_literal.version, 6u8);
            assert_eq!(parsed_literal.value, 2021u64);
        } else {
            panic!("The provided string wasn't parsed to a Literal Payload!")
        }
    }
}
