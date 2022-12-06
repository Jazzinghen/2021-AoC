mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use crate::aoc_lib::DayFn;

pub fn get_day(day: u8) -> (Option<DayFn>, Option<DayFn>) {
    match day {
        1 => (Some(day01::part1), Some(day01::part2)),
        2 => (Some(day02::part1), Some(day02::part2)),
        3 => (Some(day03::part1), Some(day03::part2)),
        4 => (Some(day04::part1), Some(day04::part2)),
        5 => (Some(day05::part1), Some(day05::part2)),
        6 => (Some(day06::part1), Some(day06::part2)),
        7 => (None, None),
        8 => (None, None),
        9 => (None, None),
        10 => (None, None),
        11 => (None, None),
        12 => (None, None),
        13 => (None, None),
        14 => (None, None),
        15 => (None, None),
        16 => (None, None),
        17 => (None, None),
        18 => (None, None),
        19 => (None, None),
        20 => (None, None),
        21 => (None, None),
        22 => (None, None),
        23 => (None, None),
        24 => (None, None),
        25 => (None, None),
        _ => {
            println!("Unknown day: {}", day);
            (None, None)
        }
    }
}
