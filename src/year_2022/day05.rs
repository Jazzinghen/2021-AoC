use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

// Box parser
fn box_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (rem_input, boxes) = separated_list1(
        char(' '),
        alt((delimited(char('['), take(1u8), char(']')), tag("   "))),
    )(input)?;

    let parsed_boxes = boxes
        .into_iter()
        .map(|val| {
            if val == "   " {
                None
            } else {
                Some(val.chars().next().unwrap())
            }
        })
        .collect_vec();

    Ok((rem_input, parsed_boxes))
}

// Instructions parser
fn move_instruction(input: &str) -> IResult<&str, (usize, (usize, usize))> {
    let (rem_input, (qty, (from, to))) = preceded(
        tag("move "),
        separated_pair(
            digit1,
            tag(" from "),
            separated_pair(digit1, tag(" to "), digit1),
        ),
    )(input)?;

    Ok((
        rem_input,
        (
            qty.parse::<usize>().unwrap(),
            (
                from.parse::<usize>().unwrap() - 1,
                to.parse::<usize>().unwrap() - 1,
            ),
        ),
    ))
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct ShipCargo {
    box_stacks: Vec<Vec<char>>,
}

impl ShipCargo {
    fn new(input: &str) -> Self {
        let mut box_stacks: Vec<Vec<char>> = Vec::new();

        for (_, box_row) in input.lines().filter_map(|l| box_line(l).ok()) {
            if box_row.len() > box_stacks.len() {
                box_stacks.resize_with(box_row.len(), Default::default);
            }
            for (idx, box_content) in box_row.into_iter().enumerate() {
                if let Some(content) = box_content {
                    box_stacks[idx].push(content);
                }
            }
        }

        for stack in box_stacks.iter_mut() {
            stack.reverse();
        }

        Self { box_stacks }
    }

    pub fn apply_instructions(&mut self, input: &str, is_cratemover9001: bool) -> String {
        for (_, (qty, (from, to))) in input
            .lines()
            .filter_map(|l| move_instruction(l.trim()).ok())
        {
            let starting_id = self.box_stacks[from].len() - qty;
            let boxes_to_move = self.box_stacks[from].drain(starting_id..).collect_vec();
            if is_cratemover9001 {
                self.box_stacks[to].extend(boxes_to_move.into_iter());
            } else {
                self.box_stacks[to].extend(boxes_to_move.into_iter().rev());
            }
        }

        self.box_stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
}

pub fn part1(input: &str) {
    let mut ship_stacks = ShipCargo::new(input);
    let final_top = ship_stacks.apply_instructions(input, false);
    println!("Final cargo state: {}", final_top);
}

pub fn part2(input: &str) {
    let mut ship_stacks = ShipCargo::new(input);
    let final_top = ship_stacks.apply_instructions(input, true);
    println!(
        "Final cargo state using the motherfucking CrateMover 9001: {}",
        final_top
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn simple_moves() {
        let mut ship_stacks = ShipCargo::new(INPUT_STRING);
        let final_top = ship_stacks.apply_instructions(INPUT_STRING, false);

        assert_eq!(final_top, "CMZ".to_string());
    }

    #[test]
    fn simple_cratemover9001() {
        let mut ship_stacks = ShipCargo::new(INPUT_STRING);
        let final_top = ship_stacks.apply_instructions(INPUT_STRING, true);

        assert_eq!(final_top, "MCD".to_string());
    }
}
