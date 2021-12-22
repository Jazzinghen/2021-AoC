use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Default)]
struct BingoBoard {
    value_to_location: HashMap<usize, (usize, usize)>,
    column_hits: [u8; 5],
    row_hits: [u8; 5],
}

impl BingoBoard {
    fn final_score(&self, last_value: &usize) -> usize {
        let mut board_score: usize = 0;
        for (value, _) in &self.value_to_location {
            board_score += value;
        }

        board_score * last_value
    }

    fn mark_value(&mut self, value: &usize) -> Option<usize> {
        let (row, col) = self.value_to_location.remove(value).expect("Requested value is not present in this board!");

        self.column_hits[col] += 1;
        self.row_hits[row] += 1;

        for freq in self.column_hits.iter() {
            if *freq >= 5 {
                return Some(self.final_score(value))
            }
        }

        for freq in self.row_hits.iter() {
            if *freq >= 5 {
                return Some(self.final_score(value))
            }
        }

        None
    }
}

pub fn part1(input: String) {
    let mut line_input = input.lines();
    let numbers_called: Vec<usize> = line_input.next().expect("Please give at least one line!").split(',').map(|val| val.parse::<usize>().expect("Didn't manage to parse the value!")).collect();

    let mut bingo_boards = Vec::<BingoBoard>::new();
    let mut val_to_board = HashMap::<usize, HashSet<usize>>::new();

    for line_chunk in line_input.chunks(6).into_iter() {
        let mut next_board: BingoBoard = BingoBoard::default();
        let board_idx = bingo_boards.len();

        for (line, data) in line_chunk.enumerate() {
            if line > 0 {
                for (col, int_str) in data.split_whitespace().map(|val| val.parse::<usize>().expect("Didn't manage to parse the value!")).enumerate() {
                    if let Some(val_set) = val_to_board.get_mut(&int_str) {
                        val_set.insert(board_idx);
                    } else {
                        val_to_board.insert(int_str, HashSet::from([board_idx]));
                    }

                    next_board.value_to_location.insert(int_str, (line-1, col));
                }
            }
        }

        bingo_boards.push(next_board);
    }

    for num in numbers_called {
        let matching_boards = val_to_board.get(&num).expect("We are trying to retrieve a value that we never inserted!");
        for board_idx in matching_boards {
            if let Some(final_score) = bingo_boards[*board_idx].mark_value(&num) {
                println!("Final score: {}", final_score);
                return;
            }
        }
    }

    println!("No winning boards found!");
}

pub fn check_for_one(binary_value: &str, bit_pos: usize) -> Option<bool> {
    match binary_value.chars().nth(bit_pos).unwrap_or('x')
        {
            '1' => {return Some(true)},
            '0' => {return Some(false)},
            _ => {return None}
        }
}

pub fn part2(input: String) {
    let line_input = input.lines();

    let (one_data, zero_data): (Vec::<&str>, Vec::<&str>) = line_input
        .partition(|line| check_for_one(line, 0).unwrap_or_else(|| panic!("Didn't get a proper binary string! Got {}", line)));

    let (mut oxygen_data, mut carbon_data) = if one_data.len() >= zero_data.len() {
        (one_data, zero_data)
    } else {
        (zero_data, one_data)
    };

    let mut curr_bit = 1;

    while oxygen_data.len() > 1 {
        let (one, zero): (Vec<&str>, Vec<&str>) = oxygen_data
            .iter()
            .partition(|line| check_for_one(line, curr_bit).unwrap_or_else(|| panic!("Didn't get a proper binary string! Got {}", line)));
        curr_bit += 1;
        oxygen_data = if one.len() >= zero.len() {
            one
        } else {
            zero
        };
    };

    let oxygen_score = match isize::from_str_radix(oxygen_data[0], 2) {
        Ok(score) => {score},
        Err(e) => {panic!("Couldn't parse the value of {}, got error: {}", oxygen_data[0], e)}
    };

    curr_bit = 1;

    while carbon_data.len() > 1 {
        let (one, zero): (Vec<&str>, Vec<&str>) = carbon_data
            .iter()
            .partition(|line| check_for_one(line, curr_bit).unwrap_or_else(|| panic!("Didn't get a proper binary string! Got {}", line)));
        curr_bit += 1;
        carbon_data = if one.len() >= zero.len() {
            zero
        } else {
            one
        };
    }

    let carbon_score = match isize::from_str_radix(carbon_data[0], 2) {
        Ok(score) => {score},
        Err(e) => {panic!("Couldn't parse the value of {}, got error: {}", carbon_data[0], e)}
    };

    println!("Life support rating: {}", oxygen_score * carbon_score);
}