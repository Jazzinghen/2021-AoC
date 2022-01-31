use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, alphanumeric1};
use nom::multi::{separated_list1};
use nom::sequence::separated_pair;
use nom::combinator::{map};
use nom::IResult;

fn display(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let parser = separated_pair(
        separated_list1(multispace1, alphanumeric1),
        tag(" | "),
        separated_list1(multispace1, alphanumeric1)
    );
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        (s.0, s.1)
    })
    (input)
}

fn lcd_digit_count(input: String, _check_all_digits: bool) -> u64 {
    let input_lines = input.lines();

    let mut count = 0u64;

    for line in input_lines {
        let (_, (_unique, digits)) = display(line.trim()).expect("Something went super wrong!");
        for digit_data in digits {
            let activation_count = digit_data.len();
            if activation_count == 2 || activation_count == 3 || activation_count == 4 || activation_count == 7 {
                count += 1;
            }
        }
    }

    return count;
}

pub fn part1(input: String) {
    let simple_digits_count = lcd_digit_count(input, false);
    println!("Amount of simple digits: {}", simple_digits_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_digits_count() {
        let input_string = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let simple_count = lcd_digit_count(input_string.to_string(), false);

        assert_eq!(simple_count, 26u64);
    }
}