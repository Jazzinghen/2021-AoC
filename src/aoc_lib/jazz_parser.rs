use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit0, one_of};
use nom::combinator::{map, opt, recognize};
use nom::sequence::pair;
use nom::IResult;

fn digit1to9(input: &str) -> IResult<&str, char> {
    one_of("123456789")(input)
}

fn uint(input: &str) -> IResult<&str, &str> {
    alt((tag("0"), recognize(pair(digit1to9, digit0))))(input)
}

pub fn _i64(input: &str) -> IResult<&str, i64> {
    let parser = recognize(pair(opt(tag("-")), uint));
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        s.parse::<i64>().unwrap()
    })(input)
}

pub fn usize(input: &str) -> IResult<&str, usize> {
    let parser = recognize(uint);
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        s.parse::<usize>().unwrap()
    })(input)
}
