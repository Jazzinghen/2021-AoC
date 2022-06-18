use std::convert::{TryFrom, TryInto};

use hashbrown::HashMap;

const FIRST_PLAYER_CYCLE: [u8; 5] = [6, 4, 2, 0, 8];
const SECOND_PLAYER_CYCLE: [u8; 5] = [5, 3, 1, 9, 7];

const ALL_ROLLS: [u8; 27] = [
    3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Player {
    position: u8,
    score: u8,
}

impl Player {
    fn new(position: u8) -> Self {
        Self { position, score: 0 }
    }

    pub fn advance(&self, squares: u8) -> Self {
        let new_position = (self.position + squares - 1) % 10 + 1;
        Self {
            position: new_position,
            score: self.score + new_position,
        }
    }
}

fn get_positions(input: &str) -> (u8, u8) {
    let starting_positions: Vec<u8> = input
        .lines()
        .map(|line| {
            line.chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap()
        })
        .collect();
    (starting_positions[0], starting_positions[1])
}

fn generate_score_loop(start_locations: (u8, u8)) -> (Vec<u8>, Vec<u8>) {
    let (mut first_location, mut second_location) = (
        usize::from(start_locations.0 + FIRST_PLAYER_CYCLE[0]),
        usize::from(start_locations.1 + SECOND_PLAYER_CYCLE[0]),
    );

    let mut scores: (Vec<u8>, Vec<u8>) = (
        vec![u8::try_from((first_location - 1) % 10).unwrap() + 1],
        vec![u8::try_from((second_location - 1) % 10).unwrap() + 1],
    );

    let mut step_idx: usize = 1;

    while (u8::try_from(first_location % 10).unwrap() != start_locations.0) || step_idx != 0 {
        first_location += usize::from(FIRST_PLAYER_CYCLE[step_idx]);
        let score: u8 = (((first_location - 1) % 10) + 1).try_into().unwrap();
        scores.0.push(score);
        step_idx = (step_idx + 1) % 5;
    }

    step_idx = 1;

    while (u8::try_from(second_location % 10).unwrap() != start_locations.1) || step_idx != 0 {
        second_location += usize::from(SECOND_PLAYER_CYCLE[step_idx]);
        let score: u8 = (((second_location - 1) % 10) + 1).try_into().unwrap();
        scores.1.push(score);
        step_idx = (step_idx + 1) % 5;
    }

    scores
}

fn compute_final_scores(starting_positions: (u8, u8)) -> ((u16, u16), u32, u32) {
    let (first_score_loop, second_score_loop) = generate_score_loop(starting_positions);

    let first_loop_total: u16 = first_score_loop.iter().cloned().map(u16::from).sum();
    let second_loop_total: u16 = second_score_loop.iter().cloned().map(u16::from).sum();

    let loops_to_win_first = 1000 / first_loop_total;

    let loops_to_win_second = 1000 / second_loop_total;

    let mut first_score = loops_to_win_first * first_loop_total;
    let mut extra_rolls_first: u8 = 0;
    while first_score < 1000 {
        first_score += u16::from(
            first_score_loop
                [usize::from(extra_rolls_first % u8::try_from(first_score_loop.len()).unwrap())],
        );
        extra_rolls_first += 1;
    }

    let mut second_score = loops_to_win_second * second_loop_total;
    let mut extra_rolls_second: u8 = 0;
    while second_score < 1000 {
        second_score += u16::from(
            second_score_loop
                [usize::from(extra_rolls_second % u8::try_from(second_score_loop.len()).unwrap())],
        );
        extra_rolls_second += 1;
    }

    let first_rolls = loops_to_win_first * u16::try_from(first_score_loop.len()).unwrap()
        + u16::from(extra_rolls_first);
    let second_rolls = loops_to_win_second * u16::try_from(second_score_loop.len()).unwrap()
        + u16::from(extra_rolls_second);

    let mut total_rolls: (u16, u16) = (first_rolls, second_rolls);

    if first_rolls <= second_rolls {
        let loops = first_rolls / u16::try_from(second_score_loop.len()).unwrap();
        let missing_rolls =
            first_rolls - (loops * u16::try_from(second_score_loop.len()).unwrap()) - 1;

        let missing_score = (0..missing_rolls).into_iter().fold(0u16, |total, idx| {
            total
                + u16::from(
                    second_score_loop
                        [usize::from(idx % u16::try_from(second_score_loop.len()).unwrap())],
                )
        });
        second_score = loops * second_loop_total + missing_score;
        total_rolls.1 = (loops * u16::try_from(second_score_loop.len()).unwrap()) + missing_rolls;
    } else {
        let loops = second_rolls / u16::try_from(first_score_loop.len()).unwrap();
        let missing_rolls = second_rolls - (loops * u16::try_from(first_score_loop.len()).unwrap());

        let missing_score = (0..missing_rolls).into_iter().fold(0u16, |total, idx| {
            total
                + u16::from(
                    first_score_loop
                        [usize::from(idx % u16::try_from(first_score_loop.len()).unwrap())],
                )
        });
        first_score = loops * first_loop_total + missing_score;
        total_rolls.0 = (loops * u16::try_from(first_score_loop.len()).unwrap()) + missing_rolls;
    }

    let total_rolls = (u32::from(total_rolls.0) + u32::from(total_rolls.1)) * 3;
    let loser_score = if first_score >= second_score {
        u32::from(second_score) * total_rolls
    } else {
        u32::from(first_score) * total_rolls
    };

    ((first_score, second_score), total_rolls, loser_score)
}

fn rec_dirac_match(
    results_lut: &mut HashMap<(Player, Player), (u64, u64)>,
    in_turn: &Player,
    next_turn: &Player,
    target_score: u8,
) -> (u64, u64) {
    if let Some(results) = results_lut.get(&(*in_turn, *next_turn)) {
        return *results;
    }

    if in_turn.score >= target_score {
        return (1, 0);
    };
    if next_turn.score >= target_score {
        return (0, 1);
    };

    let mut in_turn_score: u64 = 0;
    let mut next_turn_score: u64 = 0;

    for roll in ALL_ROLLS {
        let in_turn_evolution = in_turn.advance(roll);

        let (next_turn_wins, in_turn_wins) =
            rec_dirac_match(results_lut, next_turn, &in_turn_evolution, target_score);

        in_turn_score += in_turn_wins;
        next_turn_score += next_turn_wins;
    }

    results_lut.insert((*in_turn, *next_turn), (in_turn_score, next_turn_score));

    (in_turn_score, next_turn_score)
}

pub fn part1(input: &str) {
    let starting_positions = get_positions(input);

    let (_, _, loser_score) = compute_final_scores(starting_positions);

    println!("The loser scored: {}", loser_score);
}

pub fn part2(input: &str) {
    let starting_positions = get_positions(input);

    let player_one = Player::new(starting_positions.0);
    let player_two = Player::new(starting_positions.1);

    let mut acceleration_structure: HashMap<(Player, Player), (u64, u64)> = HashMap::new();

    let (first_universes, second_universes) =
        rec_dirac_match(&mut acceleration_structure, &player_one, &player_two, 21);

    println!(
        "The player that won in most universes won in {} universes",
        first_universes.max(second_universes)
    );
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn _generate_steps_triplets(dice_sides: u8) -> Vec<([u8; 3], [u8; 3])> {
        let rolls = if (dice_sides % 6) == 0 {
            dice_sides
        } else {
            (dice_sides / 6 + 1) * 6
        };
        (0..rolls)
            .map(|roll| (roll % dice_sides) + 1)
            .chunks(3)
            .into_iter()
            .tuples()
            .map(|(first, second)| {
                (
                    first.collect::<Vec<u8>>().try_into().unwrap(),
                    second.collect::<Vec<u8>>().try_into().unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn input_parsing() {
        let input_str = "Player 1 starting position: 4
        Player 2 starting position: 8";

        let starting_positions = get_positions(input_str);

        assert_eq!(starting_positions, (4, 8))
    }

    #[test]
    fn deterministic_match() {
        let input_str = "Player 1 starting position: 4
        Player 2 starting position: 8";

        let starting_positions = get_positions(input_str);

        let (results, total_rolls, loser_score) = compute_final_scores(starting_positions);

        assert_eq!(results.0, 1000);
        assert_eq!(results.1, 745);
        assert_eq!(total_rolls, 993);
        assert_eq!(loser_score, 739785);
    }

    #[test]
    fn begin_of_the_abyss() {
        let input_str = "Player 1 starting position: 4
        Player 2 starting position: 8";

        let starting_positions = get_positions(input_str);

        let first_player = Player::new(starting_positions.0);
        let second_player = Player::new(starting_positions.1);

        let mut acceleration_structure: HashMap<(Player, Player), (u64, u64)> = HashMap::new();

        let (first_universes, second_universes) = rec_dirac_match(
            &mut acceleration_structure,
            &first_player,
            &second_player,
            21,
        );

        assert_eq!(first_universes.max(second_universes), 444356092776315u64);
        assert_eq!(first_universes.min(second_universes), 341960390180808u64);
    }
}
