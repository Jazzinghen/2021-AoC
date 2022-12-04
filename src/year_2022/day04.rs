use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::sequence::{preceded, separated_pair};
use nom::{IResult, ParseTo};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn is_contained_into(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn is_overlapping(&self, other: &Self) -> bool {
        self.start >= other.start && self.start <= other.end
            || self.end >= other.start && self.end <= other.end
    }
}

// Activation instructions parsers
fn assignment_range(input: &str) -> IResult<&str, Range> {
    let (rem_input, (start, end)) = separated_pair(digit1, tag("-"), digit1)(input)?;

    Ok((
        rem_input,
        Range {
            start: start.parse_to().unwrap(),
            end: end.parse_to().unwrap(),
        },
    ))
}

fn assignments_line(input: &str) -> IResult<&str, (Range, Range)> {
    preceded(
        space0,
        separated_pair(assignment_range, tag(","), assignment_range),
    )(input)
}

fn find_fully_contained(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (_, ranges) = assignments_line(l).unwrap();
            ranges
        })
        .map(|(first, second)| first.is_contained_into(&second) || second.is_contained_into(&first))
        .fold(0u64, |acc, contained| acc + (contained as u64))
}

fn find_overlaps(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (_, ranges) = assignments_line(l).unwrap();
            ranges
        })
        .map(|(first, second)| first.is_overlapping(&second) || second.is_overlapping(&first))
        .fold(0u64, |acc, contained| acc + (contained as u64))
}

pub fn part1(input: &str) {
    let fully_contained_count = find_fully_contained(input);
    println!("Fully contained pairs: {}", fully_contained_count);
}

pub fn part2(input: &str) {
    let overlaps_count = find_overlaps(input);
    println!("Overlapping pairs: {}", overlaps_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn full_contained() {
        let fully_contained_count = find_fully_contained(INPUT_STRING);

        assert_eq!(fully_contained_count, 2u64);
    }

    #[test]
    fn simple_overlaps() {
        let overlaps_count = find_overlaps(INPUT_STRING);

        assert_eq!(overlaps_count, 4u64);
    }
}
