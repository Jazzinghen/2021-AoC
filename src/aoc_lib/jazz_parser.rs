use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::sequence::pair;
use nom::IResult;

pub fn _i64(input: &str) -> IResult<&str, i64> {
    map(
        pair(opt(char('-')), digit1),
        |(sign, value): (Option<char>, &str)| {
            let sign_mul = if sign.is_some() { -1 } else { 1 };
            value.parse::<i64>().unwrap() * sign_mul
        },
    )(input)
}

pub fn i32(input: &str) -> IResult<&str, i32> {
    map(
        pair(opt(char('-')), digit1),
        |(sign, value): (Option<char>, &str)| {
            let sign_mul = if sign.is_some() { -1 } else { 1 };
            value.parse::<i32>().unwrap() * sign_mul
        },
    )(input)
}

pub fn usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse().unwrap())(input)
}
