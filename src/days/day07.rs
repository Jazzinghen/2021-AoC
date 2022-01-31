use std::convert::TryInto;

use itertools::{sorted};

fn midpoint_binary_search(sorted_positions: &Vec<u32>, consumption_function: fn(&u32, &u32)->u32) -> u32 {
    let mut left_location: u32 = *sorted_positions.first().expect("Expecting this vector to be larger than 0");
    let mut right_location: u32 = *sorted_positions.last().expect("Expecting this vector to be larger than 0");

    let mut left_consumption: u64 = sorted_positions.iter().fold(0u64, |mut sum, val| {sum += u64::from(consumption_function(val, &left_location)); sum});
    let mut right_consumption: u64 = sorted_positions.iter().fold(0u64, |mut sum, val| {sum += u64::from(consumption_function(val, &right_location)); sum});

    while left_consumption != right_consumption {
        let mid_location = left_location + ((right_location - left_location) / 2);

        if left_consumption <= right_consumption {
            right_location = mid_location;
            right_consumption = sorted_positions.iter().fold(0u64, |mut sum, val| {sum += u64::from(consumption_function(val, &right_location)); sum});
        } else {
            left_location = if right_location - left_location == 1 {
                mid_location + 1
            } else {
                mid_location
            };
            left_consumption = sorted_positions.iter().fold(0u64, |mut sum, val| {sum += u64::from(consumption_function(val, &left_location)); sum});
        }
    }

    return left_location;
}

fn simple_delta(start: &u32, target: &u32) -> u32 {
    if start > target {
        start - target
    } else {
        target - start
    }
}

fn linear_delta(start: &u32, target: &u32) -> u32 {
    let delta = simple_delta(start, target);
    let upped: u64 = u64::from(delta) * u64::from(delta + 1);
    return (upped / 2).try_into().unwrap();
}

fn min_crab_fuel(input: String, consumption_function: fn(&u32, &u32)->u32) -> u64 {
    let initial_horizontal = input.split(',').map(|hor| hor.trim().parse::<u32>().expect("Given a non-number as horizontal position!"));
    let sorted_horizontal: Vec<u32> = sorted(initial_horizontal).collect();

    let mid_point = midpoint_binary_search(&sorted_horizontal, consumption_function);
    let fuel_cost = sorted_horizontal.iter().fold(0u64, |mut sum, val| {sum += u64::from(consumption_function(val, &mid_point)); sum});

    return fuel_cost;
}

pub fn part1(input: String) {
    let min_consumption = min_crab_fuel(input, simple_delta);
    println!("Estimated minimum cost: {}", min_consumption);
}

pub fn part2(input: String) {
    let min_consumption = min_crab_fuel(input, linear_delta);
    println!("Estimated minimum geometric cost: {}", min_consumption);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let input_string = "16,1,2,0,4,2,7,1,2,14";

        let min_fuel_needed = min_crab_fuel(input_string.to_string(), simple_delta);

        assert_eq!(min_fuel_needed, 37u64);
    }

    #[test]
    fn linear_rate() {
        let input_string = "16,1,2,0,4,2,7,1,2,14";

        let min_fuel_needed = min_crab_fuel(input_string.to_string(), linear_delta);

        assert_eq!(min_fuel_needed, 168u64);
    }
}