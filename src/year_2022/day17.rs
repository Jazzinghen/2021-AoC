use ansi_term::Colour;
use itertools::Itertools;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn get_vector(&self) -> Point {
        match self {
            Self::Left => Point { x: -1, y: 0 },
            Self::Right => Point { x: 1, y: 0 },
            Self::Down => Point { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tetromino {
    location: Point,
    shape: TetrominoShape,
}

impl Tetromino {
    fn new(id: usize, y: i64) -> Self {
        Self {
            location: Point { x: 2, y: y + 4 },
            shape: TETROMINO_ORDER[id],
        }
    }

    pub fn step(&self, dir: Direction) -> Self {
        Self {
            location: self.location + dir.get_vector(),
            shape: self.shape,
        }
    }

    pub fn world_points(&self) -> Vec<Point> {
        self.shape
            .get_points()
            .iter()
            .map(|p| self.location + *p)
            .collect_vec()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TetrominoShape {
    Cross,
    Flat,
    L,
    Square,
    Stick,
}

impl TetrominoShape {
    pub fn get_points(&self) -> Vec<Point> {
        match self {
            Self::Flat => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            Self::Cross => vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 2 },
            ],
            Self::Square => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
            Self::Stick => vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ],
            Self::L => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ],
        }
    }
}

static TETROMINO_ORDER: [TetrominoShape; 5] = [
    TetrominoShape::Flat,
    TetrominoShape::Cross,
    TetrominoShape::L,
    TetrominoShape::Stick,
    TetrominoShape::Square,
];

#[derive(Debug)]
struct StoneTetris {
    max_height: i64,
    occupation_grid: Vec<bool>,
    tetromino_id: usize,
    steam_directions: Vec<Direction>,
    steam_id: usize,
}

impl StoneTetris {
    fn new(input: &str) -> Self {
        Self {
            max_height: -1,
            occupation_grid: vec![false; 7 * 6],
            tetromino_id: 0,
            steam_directions: input
                .trim()
                .chars()
                .map(|c| {
                    if c == '<' {
                        Direction::Left
                    } else {
                        Direction::Right
                    }
                })
                .collect_vec(),
            steam_id: 0,
        }
    }

    fn validation_copy(&self) -> Self {
        Self {
            max_height: -1,
            occupation_grid: vec![false; 7 * 6],
            tetromino_id: 0,
            steam_directions: self.steam_directions.clone(),
            steam_id: 0,
        }
    }

    pub fn reset(&mut self) {
        self.max_height = -1;
        self.occupation_grid = vec![false; 7 * 6];
        self.tetromino_id = 0;
        self.steam_id = 0;
    }

    fn linear_id(&self, location: &Point) -> usize {
        usize::try_from(location.y).unwrap() * 7 + usize::try_from(location.x).unwrap()
    }

    fn check_collision(&self, tetro: &Tetromino) -> bool {
        tetro.world_points().into_iter().any(|point| {
            if point.x < 0 || point.x >= 7 || point.y < 0 {
                true
            } else {
                let id = self.linear_id(&point);
                self.occupation_grid[id]
            }
        })
    }

    pub fn find_max_height(&mut self, turn: usize) -> i64 {
        let (base, rep, height) = self.find_repetition(false);
        let full_reps = i64::try_from((turn - base) / rep).unwrap();
        let easy_height = full_reps * height;
        self.reset();
        let rem_turns = (turn - base) % rep;
        self.simulate(base + rem_turns);
        let rem_height = self.max_height;

        println!(
            "Height of {} reps of size {}: {}",
            full_reps, rep, easy_height
        );
        println!(
            "Height of base {} + remainder of {} % {} = {}: {}",
            base,
            (turn - base),
            rep,
            rem_turns,
            rem_height
        );

        easy_height + rem_height
    }

    fn tick_once(&self, tetro: &mut Tetromino) -> bool {
        let jet_direction = self.steam_directions[self.steam_id];

        if !self.check_collision(&tetro.step(jet_direction)) {
            tetro.location += jet_direction.get_vector();
        }

        if !self.check_collision(&tetro.step(Direction::Down)) {
            tetro.location += Direction::Down.get_vector();
            true
        } else {
            false
        }
    }

    fn simulate(&mut self, ticks: usize) -> Vec<(TetrominoShape, (usize, usize))> {
        let mut turn_steam: Vec<(TetrominoShape, (usize, usize))> = Vec::new();
        for _ in 0..ticks {
            let dropped_tetromino = TETROMINO_ORDER[self.tetromino_id];
            let steam_range = self.drop_next();
            turn_steam.push((dropped_tetromino, steam_range));
        }

        turn_steam
    }

    fn find_repetition(&mut self, print_results: bool) -> (usize, usize, i64) {
        let mut directional_patterns: Vec<(TetrominoShape, (usize, usize))> = Vec::new();
        let mut heights: Vec<i64> = Vec::new();

        let mut turn_found: usize = 0;
        let mut repetition_start_found: bool = false;

        for _ in 0..2000 {
            let dropped_tetromino = TETROMINO_ORDER[self.tetromino_id];
            let curr_checked_pattern = (dropped_tetromino, self.drop_next());
            directional_patterns.push(curr_checked_pattern);

            heights.push(self.max_height);
        }

        let mut turn: usize = 1999;
        while !repetition_start_found {
            let dropped_tetromino = TETROMINO_ORDER[self.tetromino_id];
            let curr_checked_pattern = (dropped_tetromino, self.drop_next());

            heights.push(self.max_height);
            for (id, pattern) in directional_patterns.iter().enumerate().rev() {
                if *pattern == curr_checked_pattern {
                    let mut validation_tetris = self.validation_copy();
                    validation_tetris.simulate(id);
                    let base_height = validation_tetris.max_height;

                    let potential_rep = turn - id;
                    let mut valid_pattern = true;

                    validation_tetris.simulate(potential_rep);

                    for check_round in 0u32..6 {
                        validation_tetris.simulate(2usize.pow(check_round) * potential_rep);
                        let max_height = validation_tetris.max_height;
                        let mid_height = base_height + ((max_height - base_height) / 2);

                        let base_id = validation_tetris.linear_id(&Point {
                            x: 0,
                            y: base_height + 1,
                        });
                        let max_first = validation_tetris.linear_id(&Point {
                            x: 6,
                            y: mid_height,
                        });
                        let max_second = validation_tetris.linear_id(&Point {
                            x: 6,
                            y: max_height,
                        });

                        valid_pattern &= validation_tetris.occupation_grid[base_id..=max_first]
                            == validation_tetris.occupation_grid[max_first + 1..=max_second];
                    }

                    if valid_pattern {
                        repetition_start_found = true;
                        turn_found = id;
                    }
                }
            }
            directional_patterns.push(curr_checked_pattern);
            turn += 1;
        }

        // Let's verify?

        let repetition_range = turn - turn_found - 1;

        if print_results {
            self.print_diff(turn_found, repetition_range, repetition_range * 2);
        }

        (
            turn_found,
            repetition_range,
            heights[turn_found + repetition_range] - heights[turn_found],
        )
    }

    fn drop_next(&mut self) -> (usize, usize) {
        let mut curr_tetromino = Tetromino::new(self.tetromino_id, self.max_height);
        self.tetromino_id = (self.tetromino_id + 1) % TETROMINO_ORDER.len();
        let start_steam_id = self.steam_id;

        while self.tick_once(&mut curr_tetromino) {
            self.steam_id = (self.steam_id + 1) % self.steam_directions.len();
        }

        self.max_height = self.max_height.max(
            curr_tetromino
                .world_points()
                .iter()
                .max_by(|left, right| left.y.cmp(&right.y))
                .unwrap()
                .y,
        );

        self.occupation_grid
            .resize(usize::try_from((self.max_height + 8) * 7).unwrap(), false);

        for tetro_point in curr_tetromino.world_points() {
            let point_id = self.linear_id(&tetro_point);
            self.occupation_grid[point_id] = true
        }

        // Need to advance one last time
        self.steam_id = (self.steam_id + 1) % self.steam_directions.len();

        (start_steam_id, self.steam_id)
    }

    pub fn _print_state(&self) {
        let mut tetris_lines: Vec<String> = Vec::new();
        for line in &self.occupation_grid.iter().chunks(7) {
            let formatted_line = line
                .map(|occupied| {
                    if *occupied {
                        Colour::RGB(255, 184, 108).paint("#").to_string()
                    } else {
                        Colour::RGB(248, 248, 242).paint(".").to_string()
                    }
                })
                .join("");
            tetris_lines.push(formatted_line);
        }

        let mut y: usize = tetris_lines.len();
        while let Some(line) = tetris_lines.pop() {
            y -= 1;
            if y % 10 == 0 {
                print!("{:5} ", y);
            } else {
                print!("      ");
            }
            println!("|{}|", line);
        }
        println!("      +-------+");
    }

    pub fn print_diff(&self, base: usize, first_time: usize, second_time: usize) {
        println!();
        println!();
        let mut diff_tetris = self.validation_copy();

        diff_tetris.simulate(base);
        let base_height = diff_tetris.max_height;
        diff_tetris.simulate(first_time);
        let first_max_height = diff_tetris.max_height;
        diff_tetris.simulate(second_time - first_time);
        let second_max_height = diff_tetris.max_height;

        let base_id = diff_tetris.linear_id(&Point {
            x: 0,
            y: base_height + 1,
        });
        let max_first = diff_tetris.linear_id(&Point {
            x: 6,
            y: first_max_height,
        });
        let max_second = diff_tetris.linear_id(&Point {
            x: 6,
            y: second_max_height,
        });

        let first_half = &diff_tetris.occupation_grid[base_id..=max_first];
        let mut first_strings: Vec<String> = Vec::new();
        for line in &first_half.iter().chunks(7) {
            let formatted_line = line
                .map(|occupied| {
                    if *occupied {
                        Colour::RGB(255, 184, 108).paint("#").to_string()
                    } else {
                        Colour::RGB(248, 248, 242).paint(".").to_string()
                    }
                })
                .join("");
            first_strings.push(formatted_line);
        }

        let second_half = &diff_tetris.occupation_grid[max_first + 1..=max_second];
        let mut second_strings: Vec<String> = Vec::new();
        for line in &second_half.iter().chunks(7) {
            let formatted_line = line
                .map(|occupied| {
                    if *occupied {
                        Colour::RGB(255, 184, 108).paint("#").to_string()
                    } else {
                        Colour::RGB(248, 248, 242).paint(".").to_string()
                    }
                })
                .join("");
            second_strings.push(formatted_line);
        }

        let mut right_chunks: Vec<Vec<bool>> = Vec::new();
        for chunk in &second_half.iter().chunks(7) {
            right_chunks.push(chunk.cloned().collect_vec());
        }

        let mut diff_strings: Vec<String> = Vec::new();
        for left_data in &first_half.iter().chunks(7) {
            let mut formatted_line: String = String::new();
            for (left, right) in left_data.zip(right_chunks[diff_strings.len()].iter()) {
                let next_diff = if left == right {
                    if *left {
                        Colour::Cyan.paint("#").to_string()
                    } else {
                        ".".to_string()
                    }
                } else if *left {
                    Colour::Green.paint("#").to_string()
                } else {
                    Colour::Red.paint("#").to_string()
                };
                formatted_line.push_str(&next_diff);
            }
            diff_strings.push(formatted_line);
        }

        let max_height = first_strings.len().max(second_strings.len());
        first_strings.resize(max_height, "       ".to_string());
        diff_strings.resize(max_height, "       ".to_string());
        second_strings.resize(max_height, "       ".to_string());
        for line_id in (0..max_height).rev() {
            if line_id % 10 == 0 {
                print!("{:5} ", line_id);
            } else {
                print!("      ");
            }
            print!("|{}|", first_strings[line_id]);
            if line_id % 10 == 0 {
                print!(" ------- ");
            } else {
                print!("         ");
            }
            print!("|{}|", diff_strings[line_id]);
            if line_id % 10 == 0 {
                print!(" ------- ");
            } else {
                print!("         ");
            }
            println!("|{}|", second_strings[line_id]);
        }
        println!("      +-------+         +-------+         +-------+");

        println!();
        println!();
    }
}

pub fn part1(input: &str) {
    let mut tetris = StoneTetris::new(input);
    let max_height = tetris.find_max_height(2022);

    println!("Highest rock at: {}", max_height + 1);
}

pub fn part2(input: &str) {
    let mut tetris = StoneTetris::new(input);
    let max_height = tetris.find_max_height(1000000000000);

    println!(
        "Highest rock after an unreasonable amount of time: {}",
        max_height
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn simple_repetition() {
        let mut tetris = StoneTetris::new(INPUT_STRING);
        let max_height = tetris.find_max_height(1000000000000);

        assert_eq!(max_height + 1, 1514285714288);
    }

    #[test]
    fn simple_fall() {
        let mut tetris = StoneTetris::new(INPUT_STRING);
        let max_height = tetris.find_max_height(2022);

        assert_eq!(max_height + 1, 3068);
    }
}
