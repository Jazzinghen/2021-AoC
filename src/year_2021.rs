mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::aoc_lib::DayFn;

pub fn get_day(day: u8) -> (Option<DayFn>, Option<DayFn>) {
    match day {
        1 => (Some(day01::part1), Some(day01::part2)),
        2 => (Some(day02::part1), Some(day02::part2)),
        3 => (Some(day03::part1), Some(day03::part2)),
        4 => (Some(day04::part1), None),
        5 => (Some(day05::part1), Some(day05::part2)),
        6 => (Some(day06::part1), Some(day06::part2)),
        7 => (Some(day07::part1), Some(day07::part2)),
        8 => (Some(day08::part1), Some(day08::part2)),
        9 => (Some(day09::part1), Some(day09::part2)),
        10 => (Some(day10::part1), None),
        11 => (Some(day11::part1), Some(day11::part2)),
        12 => (Some(day12::part1), Some(day12::part2)),
        13 => (Some(day13::part1), Some(day13::part2)),
        14 => (Some(day14::part1), Some(day14::part2)),
        15 => (Some(day15::part1), Some(day15::part2)),
        16 => (Some(day16::part1), Some(day16::part2)),
        17 => (Some(day17::part1), Some(day17::part2)),
        18 => (Some(day18::part1), Some(day18::part2)),
        19 => (Some(day19::both_parts), None),
        20 => (Some(day20::part1), Some(day20::part2)),
        21 => (Some(day21::part1), Some(day21::part2)),
        22 => (Some(day22::part1), Some(day22::part2)),
        23 => (Some(day23::basic::part1), Some(day23::complex::part2)),
        24 => (Some(day24::only_part), None),
        25 => (Some(day25::part1), None),
        _ => {
            println!("Unknown day: {}", day);
            (None, None)
        }
    }
}
