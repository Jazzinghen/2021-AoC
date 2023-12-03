use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

enum GemAmount {
    Red(u8),
    Green(u8),
    Blue(u8),
}

struct GameScore {
    red: u8,
    green: u8,
    blue: u8,
}

fn gem_entry(input: &str) -> IResult<&str, GemAmount> {
    let (rem_input, (amount, color)) = separated_pair(
        map_res(digit1, |s: &str| s.parse::<u8>()),
        space0,
        alt((tag("red"), tag("green"), tag("blue"))),
    )(input)?;

    let amount_out = match color {
        "red" => GemAmount::Red(amount),
        "green" => GemAmount::Green(amount),
        "blue" => GemAmount::Blue(amount),
        _ => panic!("We should not have gotten this strange color: {}", color),
    };

    Ok((rem_input, amount_out))
}

fn round_result(input: &str) -> IResult<&str, GameScore> {
    let (rem_input, gem_amount) = separated_list1(tag(", "), gem_entry)(input)?;

    let mut round_scores = GameScore {
        red: 0,
        green: 0,
        blue: 0,
    };
    for amount in gem_amount {
        match amount {
            GemAmount::Red(qty) => round_scores.red += qty,
            GemAmount::Green(qty) => round_scores.green += qty,
            GemAmount::Blue(qty) => round_scores.blue += qty,
        }
    }

    Ok((rem_input, round_scores))
}

fn game_max(input: &str) -> IResult<&str, (u32, GameScore)> {
    let (rem_input, (game_id, scores)) = separated_pair(
        preceded(tag("Game "), map_res(digit1, |s: &str| s.parse::<u32>())),
        tag(":"),
        separated_list1(tag(";"), preceded(space0, round_result)),
    )(input)?;

    let final_score = scores
        .into_iter()
        .reduce(|acc, game| GameScore {
            red: acc.red.max(game.red),
            green: acc.green.max(game.green),
            blue: acc.blue.max(game.blue),
        })
        .unwrap();

    Ok((rem_input, (game_id, final_score)))
}

fn check_max(input: &str) -> u32 {
    input
        .lines()
        .map(|game| game_max(game.trim()).unwrap().1)
        .fold(0u32, |acc, (game_id, result)| {
            if result.red > 12 || result.green > 13 || result.blue > 14 {
                acc
            } else {
                acc + game_id
            }
        })
}

fn check_power(input: &str) -> u64 {
    input
        .lines()
        .map(|game| game_max(game.trim()).unwrap().1 .1)
        .map(|maximum| u64::from(maximum.red) * u64::from(maximum.green) * u64::from(maximum.blue))
        .sum()
}

pub fn part1(input: &str) {
    let id_sum = check_max(input);
    println!("Final id sum: {}", id_sum);
}

pub fn part2(input: &str) {
    let id_sum = check_power(input);
    println!("Final id sum: {}", id_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_max() {
        let input_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let id_sum = check_max(input_string);

        assert_eq!(id_sum, 8u32);
    }

    #[test]
    fn simple_power() {
        let input_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let power = check_power(input_string);

        assert_eq!(power, 2286u64);
    }
}
