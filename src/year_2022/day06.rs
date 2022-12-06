use itertools::Itertools;

fn find_comms_start(stream: &str, length: usize) -> Result<usize, String> {
    for start in (length - 1)..stream.len() {
        let unique_chars = stream[start - (length - 1)..=start]
            .chars()
            .unique()
            .count();
        if unique_chars == length {
            return Ok(start + 1);
        }
    }

    Err(format!(
        "Didn't find a starting pattern of length {}!",
        length
    ))
}

pub fn part1(input: &str) {
    match find_comms_start(input, 4) {
        Ok(stream_start) => println!("Stream starts at: {}", stream_start),
        Err(why) => println!("{}", why),
    }
}

pub fn part2(input: &str) {
    match find_comms_start(input, 14) {
        Ok(stream_start) => println!("Message starts at: {}", stream_start),
        Err(why) => println!("{}", why),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_start() {
        let input_start: [(&str, usize); 4] = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, start) in input_start.into_iter() {
            assert_eq!(find_comms_start(input, 4).unwrap(), start);
        }
    }

    #[test]
    fn message_start() {
        let input_start: [(&str, usize); 5] = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, start) in input_start.into_iter() {
            assert_eq!(find_comms_start(input, 14).unwrap(), start);
        }
    }
}
