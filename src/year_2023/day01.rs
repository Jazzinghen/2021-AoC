use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct CalibrationDigit {
    value: u8,
    position: usize,
}

impl Ord for CalibrationDigit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for CalibrationDigit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_calibration(data: &str, spelled_support: bool) -> u64 {
    let char_numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let spelled_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut final_calibration = 0u64;
    for entry in data.lines() {
        let mut line_data = BinaryHeap::new();

        for (value, character) in char_numbers.iter().enumerate() {
            if let Some(pos) = entry.find(character) {
                line_data.push(CalibrationDigit {
                    value: u8::try_from(value + 1).unwrap(),
                    position: pos,
                });
            }
            if let Some(pos) = entry.rfind(character) {
                line_data.push(CalibrationDigit {
                    value: u8::try_from(value + 1).unwrap(),
                    position: pos,
                });
            }
        }

        if spelled_support {
            for (value, spelling) in spelled_numbers.iter().enumerate() {
                if let Some(pos) = entry.find(spelling) {
                    line_data.push(CalibrationDigit {
                        value: u8::try_from(value + 1).unwrap(),
                        position: pos,
                    });
                }
                if let Some(pos) = entry.rfind(spelling) {
                    line_data.push(CalibrationDigit {
                        value: u8::try_from(value + 1).unwrap(),
                        position: pos,
                    });
                }
            }
        }

        let final_data = line_data.into_sorted_vec();

        final_calibration += u64::from(
            final_data
                .first()
                .and_then(|entry_data| Some(entry_data.value))
                .unwrap()
                * 10
                + final_data
                    .last()
                    .and_then(|entry_data| Some(entry_data.value))
                    .unwrap(),
        );
    }

    return final_calibration;
}

pub fn part1(input: &str) {
    let calibration_value = find_calibration(input, false);
    println!("Final calibration value: {}", calibration_value);
}

pub fn part2(input: &str) {
    let calibration_value = find_calibration(input, true);
    println!("Final calibration value: {}", calibration_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_calibration() {
        let input_string = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let calibration = find_calibration(input_string, false);

        assert_eq!(calibration, 142u64);
    }

    #[test]
    fn silly_calibration() {
        let input_string = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let calibration = find_calibration(input_string, true);

        assert_eq!(calibration, 281u64);
    }
}
