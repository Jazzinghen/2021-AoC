use std::{collections::VecDeque, vec};

fn lanternfish_evolution(input: &str, time_horizon: u16) -> u64 {
    let start_lives = input.split(',');

    let mut starting_lifetimes_count = vec![0u64;9];
    for lifetime in start_lives {
        let life_num: usize = lifetime.trim().parse().expect("We got a string in the CSV that's not a number");
        starting_lifetimes_count[life_num] += 1;
    }

    let mut lives_count: VecDeque<u64> = starting_lifetimes_count.into_iter().collect();

    for _ in 0..time_horizon {
        lives_count.rotate_left(1);
        lives_count[6] += lives_count[8];
    }

    return lives_count.into_iter().sum();
}

pub fn part1(input: &str) {
    let final_population = lanternfish_evolution(input, 80);
    println!("Final population: {}", final_population);
}

pub fn part2(input: &str) {
    let final_population = lanternfish_evolution(input, 256);
    println!("Final population: {}", final_population);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_conditions() {
        let input_string = "3,4,3,1,2";

        let population_count = lanternfish_evolution(input_string, 80);

        assert_eq!(population_count, 5934u64);
    }

    #[test]
    fn infinite_resources() {
        let input_string = "3,4,3,1,2";

        let population_count = lanternfish_evolution(input_string, 256);

        assert_eq!(population_count, 26984457539u64);
    }
}