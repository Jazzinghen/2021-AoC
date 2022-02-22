use std::collections::{HashMap, HashSet, VecDeque, BTreeMap};
use std::convert::TryFrom;

use nom::bytes::complete::{tag};
use nom::character::complete::{one_of, digit1, space0};
use nom::sequence::{separated_pair, preceded};
use nom::branch::{alt};
use nom::{IResult, ParseTo};

// Activation instructions parsers
fn point_location(input: &str) -> IResult<&str, ActivationInstruction> {
    let (rem_input, (x, y)) = separated_pair(
        digit1,
        tag(","),
        digit1
    )(input)?;

    Ok((rem_input, ActivationInstruction::Point((x.parse_to().unwrap(), y.parse_to().unwrap()))))
}

fn fold_instruction(input: &str) -> IResult<&str, ActivationInstruction> {
    let (useful_input, _) = tag("fold along ")(input)?;
    let (rem_input, (direction, coord)) = separated_pair(
        one_of("xy"),
        tag("="),
        digit1)(useful_input)?;

    let fold = if direction == 'x' {
        OrigamiFold::Vertical(coord.parse_to().unwrap())
    } else {
        OrigamiFold::Horizontal(coord.parse_to().unwrap())
    };

    Ok((rem_input, ActivationInstruction::Fold(fold)))
}

fn activation_instruction(input: &str) -> IResult<&str, ActivationInstruction> {
    preceded(space0,
        alt((point_location, fold_instruction)))(input)
}

// Folding instructions
#[derive(Clone, Debug, PartialEq, Eq)]
enum OrigamiFold {
    Vertical(usize),
    Horizontal(usize)
}

// Folding instructions
#[derive(Clone, Debug, PartialEq, Eq)]
enum ActivationInstruction {
    Point((usize, usize)),
    Fold(OrigamiFold)
}

#[derive(Debug)]
struct ActivationData {
    rows: BTreeMap<usize, HashSet<usize>>,
    cols: BTreeMap<usize, HashSet<usize>>,
    folding_queue: VecDeque<OrigamiFold>,
}

impl ActivationData {
    pub fn new(input: &str) -> ActivationData {
        let mut rows_init: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
        let mut cols_init: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
        let mut fold_q: VecDeque<OrigamiFold> = VecDeque::new();
        for line in input.lines().filter(|l| !l.is_empty()) {
            let (_, parsed_line) = activation_instruction(line).unwrap();
            match parsed_line {
                ActivationInstruction::Point((x, y)) => {
                    if let Some(row_entry) = rows_init.get_mut(&y) {
                        row_entry.insert(x);
                    } else {
                        rows_init.insert(y, HashSet::from([x]));
                    }

                    if let Some(col_entry) = cols_init.get_mut(&x) {
                        col_entry.insert(y);
                    } else {
                        cols_init.insert(x, HashSet::from([y]));
                    }
                },
                ActivationInstruction::Fold(f) => {
                    fold_q.push_back(f);
                }
            }
        };

        ActivationData{rows: rows_init, cols: cols_init, folding_queue: fold_q}
    }

    pub fn fold_once(&mut self) {
        match self.folding_queue.pop_front().unwrap() {
            OrigamiFold::Horizontal(row) => {self.horizontal_fold(row)},
            OrigamiFold::Vertical(col) => {self.vertical_fold(col)}
        }
    }

    pub fn fold_all(&mut self) {
        while self.folding_queue.len() > 0 {
            self.fold_once();
        }
    }

    fn vertical_fold(&mut self, col: usize) {
        let mut removed_cols: Vec<usize> = Vec::new();
        let mut moved_data_cols: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut moved_data_rows: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        let (&max_col, _) = self.cols.iter().next_back().unwrap();

        for (col_idx, points) in self.cols.range_mut(col..=max_col) {
            let target_col_idx = col * 2 - col_idx;
            moved_data_cols.insert(target_col_idx, points.iter().map(|i| *i).collect());
            removed_cols.push(*col_idx);

            for p in points.iter() {
                if !moved_data_rows.contains_key(p) {
                    moved_data_rows.insert(*p, Vec::new());
                }
                let target_row = moved_data_rows.get_mut(p).unwrap();
                target_row.push((*col_idx, target_col_idx));
            }
        }

        for col_idx in removed_cols {
            self.cols.remove(&col_idx);
        }

        for (new_col_idx, points) in moved_data_cols {
            if !self.cols.contains_key(&new_col_idx) {
                self.cols.insert(new_col_idx, HashSet::new());
            }

            let target_col = self.cols.get_mut(&new_col_idx).unwrap();
            target_col.extend(points.iter());
        }

        for (row_idx, moved_points) in moved_data_rows {
            let row = self.rows.get_mut(&row_idx).unwrap();
            for (src, dst) in moved_points {
                row.remove(&src);
                row.insert(dst);
            }
        }
    }

    fn horizontal_fold(&mut self, row: usize) {
        let mut removed_rows: Vec<usize> = Vec::new();
        let mut moved_data_rows: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut moved_data_cols: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        let (&max_row, _) = self.rows.iter().next_back().unwrap();

        for (row_idx, points) in self.rows.range_mut(row..=max_row) {
            let target_row_idx = row * 2 - row_idx;
            moved_data_rows.insert(target_row_idx, points.iter().map(|i| *i).collect());
            removed_rows.push(*row_idx);

            for p in points.iter() {
                if !moved_data_cols.contains_key(p) {
                    moved_data_cols.insert(*p, Vec::new());
                }
                let target_col = moved_data_cols.get_mut(p).unwrap();
                target_col.push((*row_idx, target_row_idx));
            }
        }

        for row_idx in removed_rows {
            self.rows.remove(&row_idx);
        }

        for (new_row_idx, points) in moved_data_rows {
            if !self.rows.contains_key(&new_row_idx) {
                self.rows.insert(new_row_idx, HashSet::new());
            }

            let target_row = self.rows.get_mut(&new_row_idx).unwrap();
            target_row.extend(points.iter());
        }

        for (col_idx, moved_points) in moved_data_cols {
            let col = self.cols.get_mut(&col_idx).unwrap();
            for (src, dst) in moved_points {
                col.remove(&src);
                col.insert(dst);
            }
        }
    }

    pub fn get_unique_points(self) -> u64 {
        self.rows.iter().fold(0u64, |sum, (_, points)| sum + u64::try_from(points.len()).unwrap())
    }

    pub fn print_activation(self) {
        let (&max_row, _) = self.rows.iter().next_back().unwrap();
        let (&max_col, _) = self.cols.iter().next_back().unwrap();

        for row_idx in 0..=max_row {
            if let Some(row) = self.rows.get(&row_idx) {
                for col in 0..=max_col {
                    if row.get(&col).is_some() {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            } else {
                for _ in 0..max_col {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn part1(input: &str) {
    let mut data = ActivationData::new(input);
    data.fold_once();
    let point_count = data.get_unique_points();
    println!("Amount of unique points after one fold: {}", point_count);
}

pub fn part2(input: &str) {
    let mut data = ActivationData::new(input);
    data.fold_all();
    println!("Activation paper after folding:");
    println!();
    data.print_activation();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_parsing() {
        let input_string = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";

        let data = ActivationData::new(input_string);

        assert_eq!(data.get_unique_points(), 18u64);
    }

    #[test]
    fn one_fold() {
        let input_string = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";

        let mut data = ActivationData::new(input_string);
        data.fold_once();

        assert_eq!(data.get_unique_points(), 17u64);
    }

    #[test]
    fn fold_all() {
        let input_string = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";

        let mut data = ActivationData::new(input_string);
        data.fold_all();

        assert_eq!(data.get_unique_points(), 16u64);
    }
}