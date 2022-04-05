use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

use crate::aoc_lib::jazz_data::{ArenaTree, Node};

fn sailfish_component(input: &str) -> IResult<&str, ArenaTree<Option<u8>>> {
    let val_parse: IResult<&str, &str> = digit1(input);
    match val_parse {
        Ok((rem_str, value)) => {
            let new_node = Node::<Option<u8>>::new(0, Some(value.parse::<u8>().unwrap()));
            Ok((rem_str, ArenaTree::<Option<u8>> {arena: vec![new_node]}))
        }
        Err(_) => {
            let (rem_str, subtree) = sailfish_tree(input)?;
            Ok((rem_str, subtree))
        }
    }
}

pub fn sailfish_component<T, Input, Error: ParseError<Input>>(
    base_idx: usize,
    ) -> impl Fn(Input) -> IResult<Input, Input, Error>
    where
    Input: InputTake,
{
    move |input: Input| {
        input
    }
}

pub fn sailfish_tree<Input, Error: ParseError<Input>>(
    base_idx: usize,
    ) -> impl Fn(Input) -> IResult<Input, Input, Error>
    where
    Input: InputTake,
{
    move |input: Input| {
        input
    }
}


fn sailfish_tree(input: &str) -> IResult<&str, ArenaTree<Option<u8>>> {
    let (remain_str, (left, right)) = delimited(
        tag("["),
        separated_pair(sailfish_component, tag(","), sailfish_component),
        tag("]"),
    )(input)?;

    let mut new_tree

    Ok((remain_str, SailfishNumber { left, right })))
}

pub fn part1(input: &str) {
    // let (_, target_trench) = target(input).unwrap();
    // let start_v = target_trench.coolest_speed();
    // let max_height: i32 = start_v.1 * (start_v.1 + 1i32) / 2i32;
    //  println!("Maximum height for provided trench: {}", max_height);
}

pub fn part2(input: &str) {
    // let (_, target_trench) = target(input).unwrap();
    // let initial_velocities: HashSet<Point> = target_trench.compute_initial_velocities();
    // println!("Amount of initial velocities: {}", initial_velocities.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        let input_string = "[1,2]
        [[1,2],3]
        [9,[8,7]]
        [[1,9],[8,5]]
        [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
        [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
        [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

        let numbers: Vec<SailfishNumber> = Vec::new();
        for line in input_string.lines() {
            let (_, parsed_value) = sailfish_number(line).unwrap();
            numbers.push(*parsed_value);
        }

        assert_eq!(traj, reference_trajectory);
    }
}
