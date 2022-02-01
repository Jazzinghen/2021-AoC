// Days
use crate::days;

// Expose parts of the library
pub mod jazz_parser;

pub fn noop(_inp: String) {}

type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (days::day01::part1, days::day01::part2),
        2 => (days::day02::part1, days::day02::part2),
        3 => (days::day03::part1, days::day03::part2),
        4 => (days::day04::part1, noop),
        5 => (days::day05::part1, days::day05::part2),
        6 => (days::day06::part1, days::day06::part2),
        7 => (days::day07::part1, days::day07::part2),
        8 => (days::day08::part1, days::day08::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
