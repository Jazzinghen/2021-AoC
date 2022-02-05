// Days
use crate::days;

// Expose parts of the library
pub mod jazz_parser;

type DayFn = fn(&str);

pub fn get_day(day: u32) -> (Option<DayFn>, Option<DayFn>) {
    return match day {
        1 => (Some(days::day01::part1), Some(days::day01::part2)),
        2 => (Some(days::day02::part1), Some(days::day02::part2)),
        3 => (Some(days::day03::part1), Some(days::day03::part2)),
        4 => (Some(days::day04::part1), None),
        5 => (Some(days::day05::part1), Some(days::day05::part2)),
        6 => (Some(days::day06::part1), Some(days::day06::part2)),
        7 => (Some(days::day07::part1), Some(days::day07::part2)),
        8 => (Some(days::day08::part1), Some(days::day08::part2)),
        9 => (Some(days::day09::part1), Some(days::day09::part2)),
        _ => {
            println!("Unknown day: {}", day);
            return (None, None);
        }
    };
}
