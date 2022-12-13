use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone)]
enum DataType {
    Val(u32),
    List(Vec<DataType>),
}

fn parse_value(input: &str) -> IResult<&str, DataType> {
    map(digit1, |value: &str| DataType::Val(value.parse().unwrap()))(input)
}

fn parse_list(input: &str) -> IResult<&str, DataType> {
    map(
        delimited(
            char('['),
            separated_list0(char(','), alt((parse_value, parse_list))),
            char(']'),
        ),
        DataType::List,
    )(input)
}

impl Ord for DataType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self {
            DataType::Val(self_val) => match other {
                DataType::Val(other_val) => self_val.cmp(other_val),
                DataType::List(_) => {
                    let encapsulated_self = DataType::List(vec![DataType::Val(*self_val)]);
                    encapsulated_self.cmp(other)
                }
            },
            DataType::List(self_list) => match other {
                DataType::Val(other_val) => {
                    let encapsulated_other = DataType::List(vec![DataType::Val(*other_val)]);
                    self.cmp(&encapsulated_other)
                }
                DataType::List(other_list) => {
                    for (id, token) in self_list.iter().enumerate() {
                        if id >= other_list.len() {
                            return cmp::Ordering::Greater;
                        }
                        let token_cmp = token.cmp(&other_list[id]);
                        if token_cmp != cmp::Ordering::Equal {
                            return token_cmp;
                        }
                    }
                    self_list.len().cmp(&other_list.len())
                }
            },
        }
    }
}

impl PartialOrd for DataType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packets(input: &str) -> Vec<DataType> {
    let mut result: Vec<DataType> = Vec::new();
    for line in input
        .lines()
        .filter_map(|l| if l.is_empty() { None } else { Some(l.trim()) })
    {
        result.push(parse_list(line).unwrap().1);
    }

    result
}

pub fn part1(input: &str) {
    let packets = parse_packets(input);

    let mut idx_sum = 0;

    for (id, (first, second)) in packets.iter().tuples().enumerate() {
        if first <= second {
            idx_sum += id + 1;
        }
    }

    println!("Sum of right packets: {}", idx_sum);
}

pub fn part2(input: &str) {
    let mut packets = parse_packets(input);
    let (_, first_divider) = parse_list("[[2]]").unwrap();
    let (_, second_divider) = parse_list("[[6]]").unwrap();

    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    packets.sort();

    let first_id = packets
        .iter()
        .position(|pack| *pack == first_divider)
        .unwrap()
        + 1;

    let second_id = packets
        .iter()
        .position(|pack| *pack == second_divider)
        .unwrap()
        + 1;

    let decoder_key = first_id * second_id;

    println!("Decoder key: {}", decoder_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "[1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn simple_packets() {
        let packets = parse_packets(INPUT_STRING);

        let mut idx_sum = 0;

        for (id, (first, second)) in packets.iter().tuples().enumerate() {
            if first <= second {
                idx_sum += id + 1;
            }
        }

        assert_eq!(idx_sum, 13);
    }

    #[test]
    fn simple_dividers() {
        let mut packets = parse_packets(INPUT_STRING);
        let (_, first_divider) = parse_list("[[2]]").unwrap();
        let (_, second_divider) = parse_list("[[6]]").unwrap();

        packets.push(first_divider.clone());
        packets.push(second_divider.clone());

        packets.sort();

        let first_id = packets
            .iter()
            .position(|pack| *pack == first_divider)
            .unwrap()
            + 1;

        let second_id = packets
            .iter()
            .position(|pack| *pack == second_divider)
            .unwrap()
            + 1;

        let decoder_key = first_id * second_id;

        assert_eq!(decoder_key, 140);
    }
}
