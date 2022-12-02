use std::time::{Duration, Instant};
use std::{env, fs};

use clap::Parser;

mod aoc_lib;
use aoc_lib::DayFn;

mod year_2021;
mod year_2022;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLIConfig {
    /// Day to run
    #[clap(value_parser)]
    day: u8,

    /// Year to get the day from
    #[clap(short, long, value_parser, default_value_t = 2021)]
    year: u16,
}

fn get_day(year: u16, day: u8) -> (Option<DayFn>, Option<DayFn>) {
    return match year {
        2021 => year_2021::get_day(day),
        2022 => year_2022::get_day(day),
        _ => {
            println!("Unknown year: {}", year);
            return (None, None);
        }
    };
}

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return format!("{}µs", micro_sec.round());
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return format!("{}ms ", whole_ms) + &fmt_time(rem_ms);
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0)
}

fn fmt_dur(dur: Duration) -> String {
    fmt_time(dur.as_secs_f64() * 1000.0)
}

fn main() {
    // Get day string
    let user_config = CLIConfig::parse();

    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("inputs")
        .join(format!("{}", user_config.year))
        .join(format!("day{:02}.txt", user_config.day));
    println!("Reading {}", filename.display());
    println!();
    let input = fs::read_to_string(filename).expect("Error while reading");

    // Get corresponding function
    let to_run = get_day(user_config.year, user_config.day);

    // Time it
    if let Some(part_one) = to_run.0 {
        println!("Running Part 1 =============================================");
        let part1_start = Instant::now();
        part_one(&input);
        let part1_dur = part1_start.elapsed();
        println!("Took {}", fmt_dur(part1_dur));
        println!();
    }

    if let Some(part_two) = to_run.1 {
        println!("Running Part 2 =============================================");
        let part2_start = Instant::now();
        part_two(&input);
        let part2_dur = part2_start.elapsed();
        println!("Took {}", fmt_dur(part2_dur));
    }
}
