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

pub fn part2(input: String) {
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

    let mut last_score: usize = 0;
    let mut winning_boards = HashSet::<usize>::default();

    for num in numbers_called {
        let matching_boards = val_to_board.get(&num).expect("We are trying to retrieve a value that we never inserted!").difference(&winning_boards);
        let mut curr_winning_boards = HashSet::<usize>::default();
        for board_idx in matching_boards {
            if let Some(final_score) = bingo_boards[*board_idx].mark_value(&num) {
                curr_winning_boards.insert(*board_idx);
                last_score = final_score;
            }
        }
        winning_boards.extend(&curr_winning_boards);
    }

    println!("The last board to win has this score: {}", last_score);
}