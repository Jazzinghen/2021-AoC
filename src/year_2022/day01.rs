use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_max_cals(input: &str) -> u64 {
    let mut max_calories: u64 = 0;
    let mut curr_elf_calories: u64 = 0;

    for calories_raw in input.lines() {
        if let Ok(calories) = calories_raw.trim().parse::<u64>() {
            curr_elf_calories += calories;
        } else {
            max_calories = max_calories.max(curr_elf_calories);
            curr_elf_calories = 0;
        }
    }

    max_calories
}

fn find_top_cals(input: &str, top_n: usize) -> u64 {
    let mut curr_elf_calories: u64 = 0;
    let mut calories_heap: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    for calories_raw in input.lines() {
        if let Ok(calories) = calories_raw.trim().parse::<u64>() {
            curr_elf_calories += calories;
        } else {
            if calories_heap.len() >= top_n {
                if calories_heap.peek().unwrap().0 < curr_elf_calories {
                    let _ = calories_heap.pop();
                    calories_heap.push(Reverse(curr_elf_calories));
                }
            } else {
                calories_heap.push(Reverse(curr_elf_calories));
            }

            curr_elf_calories = 0;
        }
    }

    if calories_heap.len() >= top_n {
        if calories_heap.peek().unwrap().0 < curr_elf_calories {
            let _ = calories_heap.pop();
            calories_heap.push(Reverse(curr_elf_calories));
        }
    } else {
        calories_heap.push(Reverse(curr_elf_calories));
    }

    calories_heap.into_iter().map(|r| r.0).sum()
}

pub fn part1(input: &str) {
    let max_calories = find_max_cals(input);
    println!("Largest amount of carried calories: {}", max_calories);
}

pub fn part2(input: &str) {
    let max_calories = find_top_cals(input, 3);
    println!(
        "Sum of the calories carried by the top three elves: {}",
        max_calories
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_calories_count() {
        let input_string = "1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000";

        let max_calories = find_max_cals(input_string);

        assert_eq!(max_calories, 24000u64);
    }

    #[test]
    fn top_three_sum() {
        let input_string = "1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000";

        let max_calories = find_top_cals(input_string, 3);

        assert_eq!(max_calories, 45000u64);
    }
}
