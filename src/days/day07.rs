use std::{collections::VecDeque, vec};

use itertools::{sorted, Itertools};

fn fuel_consuption_binary_search(sorted_positions: &Vec<u32>) -> usize {
    let mut sorted_slice = sorted_positions.as_slice();
    let mut left_idx: usize = 0;
    let mut right_idx = sorted_slice.len();



    while left_idx <= right_idx {
        let mid_point = left_idx + ((right_idx - right_idx) / 2);
        let mid_value = sorted_slice[mid_point];
        let (low_slice, high_slice) = sorted_slice.split_at(mid_point);
        let low_sum = low_slice.iter().fold(0u32, |mut sum, val| {sum += mid_value - val; sum});
        let high_sum = high_slice.iter().fold(0u32, |mut sum, val| {sum += val - mid_value; sum});
        if low_sum <= high_sum {
            sorted_slice = low_slice
            right_idx = mid_point;
        } else {
            left_idx = right_idx;
        }
    }



    return 0;
}

fn min_crab_drift(input: String) -> u64 {
    let initial_horizontal = input.split(',').map(|hor| hor.trim().parse::<u32>().expect("Given a non-number as horizontal position!"));
    let sorted_horizontal: Vec<u32> = sorted(initial_horizontal).collect();

    return lives_count.into_iter().sum();
}

pub fn part1(input: String) {
    let min_fuel_needed = min_crab_drift(input);
    println!("Final population: {}", min_fuel_needed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let input_string = "16,1,2,0,4,2,7,1,2,14";

        let min_fuel_needed = min_crab_drift(input_string.to_string());

        assert_eq!(min_fuel_needed, 37u64);
    }
}