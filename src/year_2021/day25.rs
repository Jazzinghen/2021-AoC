use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use std::fmt;

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
struct SeaCucumber {
    direction: Direction,
    position: Point,
}

#[derive(Debug, PartialEq, Eq)]
struct SeaFloor {
    width: usize,
    height: usize,
    cucumbers: Vec<SeaCucumber>,
    by_position: HashMap<Point, usize>,
    could_move: HashSet<usize>,
    timestep: usize,
}

impl SeaFloor {
    fn new(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut cucumbers: Vec<SeaCucumber> = Vec::new();

        for (row, raw_data) in input.lines().enumerate() {
            for (col, cell_data) in raw_data
                .trim()
                .char_indices()
                .filter(|(_, cell)| *cell != '.')
            {
                match cell_data {
                    '>' => cucumbers.push(SeaCucumber {
                        direction: Direction::Right,
                        position: (row, col),
                    }),
                    'v' => cucumbers.push(SeaCucumber {
                        direction: Direction::Down,
                        position: (row, col),
                    }),
                    _ => panic!("We got some strange runaway character: {}", cell_data),
                }
            }
        }

        let mut by_position: HashMap<Point, usize> = HashMap::new();

        for (id, cucumb) in cucumbers.iter().enumerate() {
            by_position.insert(cucumb.position, id);
        }

        let mut could_move: HashSet<usize> = HashSet::new();

        for (id, cucumb) in cucumbers.iter().enumerate() {
            let (row, col) = &cucumb.position;

            let target_position = match cucumb.direction {
                Direction::Down => {
                    let target_row = if *row == height - 1 { 0 } else { *row + 1 };
                    (target_row, *col)
                }
                Direction::Right => {
                    let target_col = if *col == width - 1 { 0 } else { *col + 1 };
                    (*row, target_col)
                }
            };

            if !by_position.contains_key(&target_position) {
                could_move.insert(id);
            }
        }

        Self {
            width,
            height,
            cucumbers,
            by_position,
            could_move,
            timestep: 0,
        }
    }

    pub fn find_final_state(&mut self) -> Result<(), &'static str> {
        while !self.could_move.is_empty() && self.timestep < 1000 {
            self.step();
        }

        if self.timestep >= 1000 {
            Err("Didn't manage to find a stable state (1000 timesteps")
        } else {
            Ok(())
        }
    }

    pub fn step(&mut self) {
        self.timestep += 1;

        let possible_horizontal = self
            .could_move
            .iter()
            .filter(|id| self.cucumbers[**id].direction == Direction::Right)
            .cloned()
            .collect_vec();

        for hor_id in possible_horizontal {
            let (row, col) = self.cucumbers[hor_id].position;
            let target_col = if col == self.width - 1 { 0 } else { col + 1 };
            let target_position = (row, target_col);
            if self.by_position.contains_key(&target_position) {
                self.could_move.remove(&hor_id);
            } else {
                let row_above = if row == 0 { self.height - 1 } else { row - 1 };

                let previous_above = (row_above, col);
                if let Some(prev_cucumber) = self.by_position.get(&previous_above) {
                    if self.cucumbers[*prev_cucumber].direction == Direction::Down {
                        self.could_move.insert(*prev_cucumber);
                    }
                }
                let current_above = (row_above, target_col);
                if let Some(curr_cucumber) = self.by_position.get(&current_above) {
                    if self.cucumbers[*curr_cucumber].direction == Direction::Down {
                        self.could_move.remove(curr_cucumber);
                    }
                }

                let prev_col = if col == 0 { self.width - 1 } else { col - 1 };
                let prev_behind = (row, prev_col);
                if let Some(behind_cucumber) = self.by_position.get(&prev_behind) {
                    if self.cucumbers[*behind_cucumber].direction == Direction::Right {
                        self.could_move.insert(*behind_cucumber);
                    }
                }
                let next_col = if target_col == self.width - 1 {
                    0
                } else {
                    target_col + 1
                };
                let curr_front = (row, next_col);
                if self.by_position.contains_key(&curr_front) {
                    self.could_move.remove(&hor_id);
                }

                self.by_position.remove(&self.cucumbers[hor_id].position);
                self.by_position.insert(target_position, hor_id);

                self.cucumbers[hor_id].position = target_position;
            }
        }

        let possible_vertical = self
            .could_move
            .iter()
            .filter(|id| self.cucumbers[**id].direction == Direction::Down)
            .cloned()
            .collect_vec();

        for vert_id in possible_vertical {
            let (row, col) = self.cucumbers[vert_id].position;
            let target_row = if row == self.height - 1 { 0 } else { row + 1 };
            let target_position = (target_row, col);
            if self.by_position.contains_key(&target_position) {
                self.could_move.remove(&vert_id);
            } else {
                let col_left = if col == 0 { self.width - 1 } else { col - 1 };
                let row_above = if row == 0 { self.height - 1 } else { row - 1 };

                let previous_above = (row_above, col);
                if let Some(prev_above) = self.by_position.get(&previous_above) {
                    if self.cucumbers[*prev_above].direction == Direction::Down {
                        self.could_move.insert(*prev_above);
                    }
                }
                let next_row = if target_row == self.height - 1 {
                    0
                } else {
                    target_row + 1
                };
                let curr_below = (next_row, col);
                if self.by_position.contains_key(&curr_below) {
                    self.could_move.remove(&vert_id);
                }

                let current_left = (target_row, col_left);
                if let Some(curr_cucumber) = self.by_position.get(&current_left) {
                    if self.cucumbers[*curr_cucumber].direction == Direction::Right {
                        self.could_move.remove(curr_cucumber);
                    }
                }
                let prev_left = (row, col_left);
                if let Some(prev_cucumber) = self.by_position.get(&prev_left) {
                    if self.cucumbers[*prev_cucumber].direction == Direction::Right {
                        self.could_move.insert(*prev_cucumber);
                    }
                }

                self.by_position.remove(&self.cucumbers[vert_id].position);
                self.by_position.insert(target_position, vert_id);

                self.cucumbers[vert_id].position = target_position;
            }
        }
    }
}

impl fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0usize..self.height {
            let mut raw_line: Vec<char> = vec!['.'; self.width];
            for cell_id in (0usize..self.width).filter_map(|col| self.by_position.get(&(row, col)))
            {
                let col = self.cucumbers[*cell_id].position.1;
                match self.cucumbers[*cell_id].direction {
                    Direction::Down => raw_line[col] = 'v',
                    Direction::Right => raw_line[col] = '>',
                };
            }

            writeln!(f, "{}", raw_line.into_iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) {
    let mut sea_floor = SeaFloor::new(input);

    sea_floor.find_final_state().unwrap();

    println!(
        "The cucumbers stop moving at timestep {}",
        sea_floor.timestep + 1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SeaFloor {
        pub fn _find_differences(&self, other: &Self) {
            let self_print_data = format!("{}", self)
                .lines()
                .map(|line| line.to_string())
                .collect_vec();
            let other_print_data = format!("{}", other)
                .lines()
                .map(|line| line.to_string())
                .collect_vec();

            let data_width = self_print_data.len();
            println!(
                "{:^data_width$}\t{:^data_width$}\t{:^data_width$}",
                "Self", "Diff", "Other"
            );

            for row in 0usize..self.height {
                let mut raw_line: Vec<char> = vec!['.'; self.width];
                for cell_id in
                    (0usize..self.width).filter_map(|col| self.by_position.get(&(row, col)))
                {
                    let other_char = if let Some(other_cucumber) =
                        other.by_position.get(&self.cucumbers[*cell_id].position)
                    {
                        match other.cucumbers[*other_cucumber].direction {
                            Direction::Down => 'v',
                            Direction::Right => '>',
                        }
                    } else {
                        '.'
                    };

                    let col = self.cucumbers[*cell_id].position.1;
                    let our_char = match self.cucumbers[*cell_id].direction {
                        Direction::Down => 'v',
                        Direction::Right => '>',
                    };

                    raw_line[col] = if other_char == our_char {
                        our_char
                    } else {
                        '*'
                    };
                }

                println!(
                    "{}\t{}\t{}",
                    self_print_data[row],
                    raw_line.into_iter().collect::<String>(),
                    other_print_data[row]
                );
            }
        }

        pub fn _print_movable_cucumbers(&self) {
            for cucumber in self
                .could_move
                .iter()
                .map(|id| self.cucumbers.get(*id).unwrap())
                .sorted_by(|left, right| Ord::cmp(&left.position, &right.position))
            {
                print!(
                    "\tCucumber at position [{}, {}] could move ",
                    cucumber.position.0, cucumber.position.1
                );
                match cucumber.direction {
                    Direction::Right => println!("right."),
                    Direction::Down => println!("down."),
                };
            }
        }
    }

    #[test]
    fn parse_data() {
        let input_string = "..........
        .>v....v..
        .......>..
        ..........";

        let sea_floor = SeaFloor::new(input_string);

        let ref_floor = SeaFloor {
            width: 10,
            height: 4,
            cucumbers: vec![
                SeaCucumber {
                    direction: Direction::Right,
                    position: (1, 1),
                },
                SeaCucumber {
                    direction: Direction::Down,
                    position: (1, 2),
                },
                SeaCucumber {
                    direction: Direction::Down,
                    position: (1, 7),
                },
                SeaCucumber {
                    direction: Direction::Right,
                    position: (2, 7),
                },
            ],
            by_position: HashMap::from([((1, 1), 0), ((1, 2), 1), ((1, 7), 2), ((2, 7), 3)]),
            could_move: HashSet::from([1, 3]),
            timestep: 0,
        };

        assert_eq!(sea_floor, ref_floor);
    }

    #[test]
    fn single_line() {
        let input_string = "...>>>>>...";

        let mut sea_floor = SeaFloor::new(input_string);

        let ref_data = ["...>>>>.>..", "...>>>.>.>."];

        for raw_state in ref_data {
            sea_floor.step();

            let ref_floor = SeaFloor::new(raw_state);

            for cucumber in sea_floor.cucumbers.iter() {
                let ref_cucumber_id = ref_floor.by_position.get(&cucumber.position).unwrap();
                assert_eq!(
                    cucumber.direction,
                    ref_floor.cucumbers[*ref_cucumber_id].direction
                );
            }
        }
    }

    #[test]
    fn simple_steps() {
        let input_string = "...>...
        .......
        ......>
        v.....>
        ......>
        .......
        ..vvv..";

        let mut sea_floor = SeaFloor::new(input_string);

        let ref_input = vec![
            "..vv>..
        .......
        >......
        v.....>
        >......
        .......
        ....v..",
            "....v>.
        ..vv...
        .>.....
        ......>
        v>.....
        .......
        .......",
            "......>
        ..v.v..
        ..>v...
        >......
        ..>....
        v......
        .......",
            ">......
        ..v....
        ..>.v..
        .>.v...
        ...>...
        .......
        v......",
        ];

        for input_data in ref_input {
            let ref_state = SeaFloor::new(input_data);
            sea_floor.step();

            println!("Status at step {}", sea_floor.timestep);
            println!();
            println!("{}", sea_floor);
            println!();

            for current_cucumber in sea_floor.cucumbers.iter() {
                let position = current_cucumber.position;
                let ref_id = ref_state.by_position.get(&position).unwrap();
                let ref_cucumber = ref_state.cucumbers.get(*ref_id).unwrap();
                assert_eq!(current_cucumber.direction, ref_cucumber.direction);
            }
        }
    }

    #[test]
    fn simple_stable_state() {
        let input_string = "v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>";

        let mut sea_floor = SeaFloor::new(input_string);

        sea_floor.find_final_state().unwrap();
        assert_eq!(sea_floor.timestep + 1, 58);

        let mut sea_floor = SeaFloor::new(input_string);

        let ref_data = vec![
            (
                1usize,
                "....>.>v.>
                v.v>.>v.v.
                >v>>..>v..
                >>v>v>.>.v
                .>v.v...v.
                v>>.>vvv..
                ..v...>>..
                vv...>>vv.
                >.v.v..v.v",
            ),
            (
                2usize,
                ">.v.v>>..v
                v.v.>>vv..
                >v>.>.>.v.
                >>v>v.>v>.
                .>..v....v
                .>v>>.v.v.
                v....v>v>.
                .vv..>>v..
                v>.....vv.",
            ),
            (
                3usize,
                "v>v.v>.>v.
                v...>>.v.v
                >vv>.>v>..
                >>v>v.>.v>
                ..>....v..
                .>.>v>v..v
                ..v..v>vv>
                v.v..>>v..
                .v>....v..",
            ),
            (
                4usize,
                "v>..v.>>..
                v.v.>.>.v.
                >vv.>>.v>v
                >>.>..v>.>
                ..v>v...v.
                ..>>.>vv..
                >.v.vv>v.v
                .....>>vv.
                vvv>...v..",
            ),
            (
                5usize,
                "vv>...>v>.
                v.v.v>.>v.
                >.v.>.>.>v
                >v>.>..v>>
                ..v>v.v...
                ..>.>>vvv.
                .>...v>v..
                ..v.v>>v.v
                v.v.>...v.",
            ),
            (
                10usize,
                "..>..>>vv.
                v.....>>.v
                ..v.v>>>v>
                v>.>v.>>>.
                ..v>v.vv.v
                .v.>>>.v..
                v.v..>v>..
                ..v...>v.>
                .vv..v>vv.",
            ),
            (
                20usize,
                "v>.....>>.
                >vv>.....v
                .>v>v.vv>>
                v>>>v.>v.>
                ....vv>v..
                .v.>>>vvv.
                ..v..>>vv.
                v.v...>>.v
                ..v.....v>",
            ),
            (
                30usize,
                ".vv.v..>>>
                v>...v...>
                >.v>.>vv.>
                >v>.>.>v.>
                .>..v.vv..
                ..v>..>>v.
                ....v>..>v
                v.v...>vv>
                v.v...>vvv",
            ),
            (
                40usize,
                ">>v>v..v..
                ..>>v..vv.
                ..>>>v.>.v
                ..>>>>vvv>
                v.....>...
                v.v...>v>>
                >vv.....v>
                .>v...v.>v
                vvv.v..v.>",
            ),
            (
                50usize,
                "..>>v>vv.v
                ..v.>>vv..
                v.>>v>>v..
                ..>>>>>vv.
                vvv....>vv
                ..v....>>>
                v>.......>
                .vv>....v>
                .>v.vv.v..",
            ),
            (
                55usize,
                "..>>v>vv..
                ..v.>>vv..
                ..>>v>>vv.
                ..>>>>>vv.
                v......>vv
                v>v....>>v
                vvv...>..>
                >vv.....>.
                .>v.vv.v..",
            ),
            (
                56usize,
                "..>>v>vv..
                ..v.>>vv..
                ..>>v>>vv.
                ..>>>>>vv.
                v......>vv
                v>v....>>v
                vvv....>.>
                >vv......>
                .>v.vv.v..",
            ),
            (
                57usize,
                "..>>v>vv..
                ..v.>>vv..
                ..>>v>>vv.
                ..>>>>>vv.
                v......>vv
                v>v....>>v
                vvv.....>>
                >vv......>
                .>v.vv.v..",
            ),
            (
                58usize,
                "..>>v>vv..
                ..v.>>vv..
                ..>>v>>vv.
                ..>>>>>vv.
                v......>vv
                v>v....>>v
                vvv.....>>
                >vv......>
                .>v.vv.v..",
            ),
        ];

        for (step, step_ref_data) in ref_data {
            let ref_state = SeaFloor::new(step_ref_data);

            while sea_floor.timestep < step {
                sea_floor.step();
            }

            for current_cucumber in sea_floor.cucumbers.iter() {
                let position = current_cucumber.position;
                let ref_id = ref_state.by_position.get(&position).unwrap();
                let ref_cucumber = ref_state.cucumbers.get(*ref_id).unwrap();
                assert_eq!(current_cucumber.direction, ref_cucumber.direction);
            }
        }
    }
}
