use std::cmp::Ordering;
use std::convert::From;

use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum JanKen {
    Rock,
    Paper,
    Scissors,
}

impl Ord for JanKen {
    fn cmp(&self, other: &Self) -> Ordering {
        if *self == Self::Scissors && *other == Self::Rock {
            return Ordering::Less;
        }

        if *self == Self::Rock && *other == Self::Scissors {
            return Ordering::Greater;
        }

        let our_score = u8::from(self);
        let their_score = u8::from(other);

        our_score.cmp(&their_score)
    }
}

impl PartialOrd for JanKen {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<JanKen> for u8 {
    fn from(choice: JanKen) -> Self {
        match choice {
            JanKen::Rock => 1,
            JanKen::Paper => 2,
            JanKen::Scissors => 3,
        }
    }
}

impl From<&JanKen> for u8 {
    fn from(choice: &JanKen) -> Self {
        match choice {
            JanKen::Rock => 1,
            JanKen::Paper => 2,
            JanKen::Scissors => 3,
        }
    }
}

fn compute_score(opponent: &JanKen, ours: &JanKen) -> u8 {
    let score: u8 = u8::from(ours);
    score
        + match ours.cmp(opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
}

fn parse_line_straigth(line: &str) -> (JanKen, JanKen) {
    let choices: Vec<char> = line
        .split_whitespace()
        .map(|c| c.chars().next().unwrap())
        .collect_vec();
    let theirs = match choices[0] {
        'A' => JanKen::Rock,
        'B' => JanKen::Paper,
        'C' => JanKen::Scissors,
        _ => panic!("What's this choice?!?"),
    };
    let ours = match choices[1] {
        'X' => JanKen::Rock,
        'Y' => JanKen::Paper,
        'Z' => JanKen::Scissors,
        _ => panic!("What's this choice?!?"),
    };

    (theirs, ours)
}

fn parse_line_strategy(line: &str) -> (JanKen, JanKen) {
    let choices: Vec<char> = line
        .split_whitespace()
        .map(|c| c.chars().next().unwrap())
        .collect_vec();
    let theirs = match choices[0] {
        'A' => JanKen::Rock,
        'B' => JanKen::Paper,
        'C' => JanKen::Scissors,
        _ => panic!("What's this choice?!?"),
    };
    let ours = match choices[1] {
        'X' => match theirs {
            JanKen::Rock => JanKen::Scissors,
            JanKen::Paper => JanKen::Rock,
            JanKen::Scissors => JanKen::Paper,
        },
        'Y' => theirs,
        'Z' => match theirs {
            JanKen::Rock => JanKen::Paper,
            JanKen::Paper => JanKen::Scissors,
            JanKen::Scissors => JanKen::Rock,
        },
        _ => panic!("What's this choice?!?"),
    };

    (theirs, ours)
}

fn compute_straight_choices(input: &str) -> u64 {
    input
        .lines()
        .map(|entry| parse_line_straigth(entry.trim()))
        .fold(0u64, |acc, (theirs, ours)| {
            acc + u64::from(compute_score(&theirs, &ours))
        })
}

fn compute_strategic_choices(input: &str) -> u64 {
    input
        .lines()
        .map(|entry| parse_line_strategy(entry.trim()))
        .fold(0u64, |acc, (theirs, ours)| {
            acc + u64::from(compute_score(&theirs, &ours))
        })
}

pub fn part1(input: &str) {
    let straight_score = compute_straight_choices(input);
    println!(
        "Score if we follow the manual as if it reported the direct choice: {}",
        straight_score
    );
}

pub fn part2(input: &str) {
    let strategy_score = compute_strategic_choices(input);
    println!(
        "Score if we follow the manual as if it reported the outcome: {}",
        strategy_score
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_score() {
        let input_string = "A Y
        B X
        C Z";

        let straight_choices = compute_straight_choices(input_string);

        assert_eq!(straight_choices, 15u64);
    }

    #[test]
    fn strategic_score() {
        let input_string = "A Y
        B X
        C Z";

        let strategic_scores = compute_strategic_choices(input_string);

        assert_eq!(strategic_scores, 12u64);
    }
}
