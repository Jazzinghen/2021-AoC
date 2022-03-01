use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn display(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let parser = separated_pair(
        separated_list1(multispace1, alphanumeric1),
        tag(" | "),
        separated_list1(multispace1, alphanumeric1),
    );
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        (s.0, s.1)
    })(input)
}

fn cypher_crack(cypher: Vec<&str>, digits: Vec<&str>) -> u32 {
    let mut mappings: Vec<HashSet<char>> = vec![Default::default(); 10];

    let mut potential_six = Vec::<HashSet<char>>::new();
    let mut potential_five = Vec::<HashSet<char>>::new();

    // This problem requires us to go through a lot of pain. We need to extract all the known combinations, then use set
    // arithmetic to get the important bits to create the other values
    for digit_code in cypher {
        match digit_code.len() {
            2 => mappings[1].extend(digit_code.chars()),
            3 => mappings[7].extend(digit_code.chars()),
            4 => mappings[4].extend(digit_code.chars()),
            5 => potential_five.push(HashSet::from_iter(digit_code.chars())),
            6 => potential_six.push(HashSet::from_iter(digit_code.chars())),
            7 => mappings[8].extend(digit_code.chars()),
            _ => {}
        }
    }

    let mut nine_idx = 0usize;
    let mut lower_left_code = HashSet::new();
    for (idx, six_lines) in potential_six.iter().enumerate() {
        if six_lines.is_superset(&mappings[4]) {
            mappings[9].extend(six_lines);
            nine_idx = idx;
            let lower_left_set: Vec<_> = mappings[8].difference(&mappings[9]).collect();
            assert_eq!(
                lower_left_set.len(),
                1,
                "We should have a single code in the difference between 8 and 9!"
            );
            lower_left_code.insert(*lower_left_set[0]);
        }
    }
    potential_six.swap_remove(nine_idx);

    let (zero_idx, six_idx) = if potential_six[0].is_superset(&mappings[1]) {
        (0, 1)
    } else {
        (1, 0)
    };
    mappings[0].extend(&potential_six[zero_idx]);
    mappings[6].extend(&potential_six[six_idx]);

    let mut five_idx = 0usize;
    for (idx, five_lines) in potential_five.iter().enumerate() {
        let potential_six_encode = five_lines.union(&lower_left_code);
        let mut potential_six_set: HashSet<char> = HashSet::new();
        potential_six_set.extend(potential_six_encode);
        if mappings[6].eq(&potential_six_set) {
            mappings[5].extend(five_lines);
            five_idx = idx;
        }
    }
    potential_five.swap_remove(five_idx);

    let centre_code_data: Vec<_> = mappings[8].difference(&mappings[0]).collect();
    assert_eq!(
        centre_code_data.len(),
        1,
        "We should have a single code in the difference between 8 and 0!"
    );
    let upper_left_data: Vec<_> = mappings[4].difference(&mappings[1]).collect();
    let upper_left_code = if upper_left_data[0] == centre_code_data[0] {
        *upper_left_data[1]
    } else {
        *upper_left_data[0]
    };
    let mut opposite_one_set = HashSet::new();
    opposite_one_set.insert(upper_left_code);
    opposite_one_set.extend(lower_left_code.iter());

    let potential_three_diff = mappings[8].difference(&opposite_one_set);
    let mut potential_three_set: HashSet<char> = HashSet::new();
    potential_three_set.extend(potential_three_diff);
    let (three_idx, two_idx) = if potential_five[0].eq(&potential_three_set) {
        (0, 1)
    } else {
        (1, 0)
    };
    mappings[3].extend(&potential_five[three_idx]);
    mappings[2].extend(&potential_five[two_idx]);

    let mut final_value = 0u32;

    for obf_digit in digits {
        final_value *= 10;
        let mut obf_digit_set: HashSet<char> = HashSet::new();
        obf_digit_set.extend(obf_digit.chars());

        for (open_digit, code) in mappings.iter().enumerate() {
            if obf_digit_set.eq(code) {
                final_value += u32::try_from(open_digit).unwrap();
            }
        }
    }

    final_value
}

fn lcd_simple_digit_count(input: &str) -> u64 {
    let input_lines = input.lines();

    let mut count = 0u64;

    for line in input_lines {
        let (_, (_, digits)) = display(line.trim()).expect("Something went super wrong!");
        for digit_data in digits {
            let activation_count = digit_data.len();
            if activation_count == 2
                || activation_count == 3
                || activation_count == 4
                || activation_count == 7
            {
                count += 1;
            }
        }
    }

    count
}

fn output_decrypt_sum(input: &str) -> u64 {
    let input_lines = input.lines();

    let mut final_sum = 0u64;

    for line in input_lines {
        let (_, (cypher, digits)) = display(line.trim()).expect("Something went super wrong!");
        let encoded_value = cypher_crack(cypher, digits);
        final_sum += u64::from(encoded_value);
    }

    final_sum
}

pub fn part1(input: &str) {
    let simple_digits_count = lcd_simple_digit_count(input);
    println!("Amount of simple digits: {}", simple_digits_count);
}

pub fn part2(input: &str) {
    let decrypted_sum = output_decrypt_sum(input);
    println!("Sum of all the encrypted values: {}", decrypted_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_digits_count() {
        let input_string =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let simple_count = lcd_simple_digit_count(input_string);

        assert_eq!(simple_count, 26u64);
    }

    #[test]
    fn encrypted_values_sum() {
        let input_string =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let simple_count = output_decrypt_sum(input_string);

        assert_eq!(simple_count, 61229u64);
    }
}
