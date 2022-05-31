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
        10 => (Some(days::day10::part1), None),
        11 => (Some(days::day11::part1), Some(days::day11::part2)),
        12 => (Some(days::day12::part1), Some(days::day12::part2)),
        13 => (Some(days::day13::part1), Some(days::day13::part2)),
        14 => (Some(days::day14::part1), Some(days::day14::part2)),
        15 => (Some(days::day15::part1), Some(days::day15::part2)),
        16 => (Some(days::day16::part1), Some(days::day16::part2)),
        17 => (Some(days::day17::part1), Some(days::day17::part2)),
        18 => (Some(days::day18::part1), Some(days::day18::part2)),
        19 => (Some(days::day19::both_parts), None),
        20 => (Some(days::day20::part1), Some(days::day20::part2)),
        21 => (Some(days::day21::part1), Some(days::day21::part2)),
        22 => (None, None),
        _ => {
            println!("Unknown day: {}", day);
            return (None, None);
        }
    };
}
