use hashbrown::HashSet;
use itertools::Itertools;
use nom::character::complete::{char, digit1, one_of};
use nom::sequence::separated_pair;
use nom::IResult;

type Point = (i64, i64);

enum Direction {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

fn instruction_line(input: &str) -> IResult<&str, Direction> {
    let (rem_input, (direction, steps)) = separated_pair(one_of("UDLR"), char(' '), digit1)(input)?;
    let steps_num: i64 = steps.parse().unwrap();
    let instruction = match direction {
        'U' => Direction::Up(steps_num),
        'D' => Direction::Down(steps_num),
        'L' => Direction::Left(steps_num),
        'R' => Direction::Right(steps_num),
        _ => panic!("We should never get here!"),
    };

    Ok((rem_input, instruction))
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rope {
    knot_locations: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            knot_locations: vec![(0, 0); knots + 1],
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn update_knot_position(&mut self, parent_knot_id: usize) -> bool {
        let (parent_y, parent_x) = self.knot_locations[parent_knot_id];
        let (knot_y, knot_x) = self.knot_locations[parent_knot_id + 1];

        let delta_y = parent_y.abs_diff(knot_y);
        let delta_x = parent_x.abs_diff(knot_x);

        if delta_y < 2 && delta_x < 2 {
            false
        } else {
            let mut new_location = (knot_y, knot_x);
            new_location.0 += match parent_y.cmp(&knot_y) {
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Less => -1,
            };
            new_location.1 += match parent_x.cmp(&knot_x) {
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Less => -1,
            };
            self.knot_locations[parent_knot_id + 1] = new_location;
            true
        }
    }

    fn update_knots(&mut self) {
        let mut parent_id: usize = 0;
        let parents_qty = self.knot_locations.len() - 1;
        while parent_id < parents_qty && self.update_knot_position(parent_id) {
            parent_id += 1;
        }

        if parent_id == parents_qty {
            self.visited.insert(*self.knot_locations.last().unwrap());
        }
    }

    pub fn move_head(&mut self, instruction: Direction) {
        match instruction {
            Direction::Up(steps) => {
                let starting_y = self.knot_locations.first().unwrap().0;
                for y in starting_y + 1..=starting_y + steps {
                    self.knot_locations.first_mut().unwrap().0 = y;
                    self.update_knots();
                }
            }
            Direction::Down(steps) => {
                let starting_y = self.knot_locations.first().unwrap().0;
                for y in (starting_y - steps..starting_y).rev() {
                    self.knot_locations.first_mut().unwrap().0 = y;
                    self.update_knots();
                }
            }
            Direction::Right(steps) => {
                let starting_x = self.knot_locations.first().unwrap().1;
                for x in starting_x + 1..=starting_x + steps {
                    self.knot_locations.first_mut().unwrap().1 = x;
                    self.update_knots();
                }
            }
            Direction::Left(steps) => {
                let starting_x = self.knot_locations.first().unwrap().1;
                for x in (starting_x - steps..starting_x).rev() {
                    self.knot_locations.first_mut().unwrap().1 = x;
                    self.update_knots();
                }
            }
        }
    }

    pub fn _print_moves(&self) {
        let knots_max_y = self.knot_locations.iter().max_by_key(|(y, _)| y).unwrap().0 + 1;
        let knots_min_y = self.knot_locations.iter().min_by_key(|(y, _)| y).unwrap().0;
        let knots_max_x = self.knot_locations.iter().max_by_key(|(_, x)| x).unwrap().1 + 1;
        let knots_min_x = self.knot_locations.iter().min_by_key(|(_, x)| x).unwrap().1;

        let max_y = self.visited.iter().max_by_key(|(y, _)| y).unwrap().0 + 1;
        let min_y = self.visited.iter().min_by_key(|(y, _)| y).unwrap().0;
        let max_x = self.visited.iter().max_by_key(|(_, x)| x).unwrap().1 + 1;
        let min_x = self.visited.iter().min_by_key(|(_, x)| x).unwrap().1;

        let bottom_left = (min_y.min(knots_min_y), min_x.min(knots_min_x));
        let top_right = (max_y.max(knots_max_y), max_x.max(knots_max_x));

        let y_range: usize = (top_right.0 - bottom_left.0).try_into().unwrap();
        let x_range: usize = (top_right.1 - bottom_left.1).try_into().unwrap();

        let mut visit_matrix = vec![false; x_range * y_range];
        for visited_point in self.visited.iter() {
            let linear_id = _compute_linear_id(visited_point, &bottom_left, &top_right);
            visit_matrix[linear_id] = true;
        }

        let mut map_data: Vec<char> = Vec::new();
        println!(
            "Printing from [{}, {}] to [{}, {}]",
            bottom_left.0, bottom_left.1, top_right.0, top_right.1
        );
        for grid_location in (bottom_left.0..top_right.0)
            .rev()
            .cartesian_product(bottom_left.1..top_right.1)
            .map(|(y, x)| (y, x))
        {
            let linear_id = _compute_linear_id(&grid_location, &bottom_left, &top_right);
            if visit_matrix[linear_id] {
                map_data.push('#');
            } else {
                map_data.push('.');
            }
        }
        let head_point = self.knot_locations.first().unwrap();
        let head_linear = _compute_linear_id(head_point, &bottom_left, &top_right);
        map_data[head_linear] = 'H';
        for (knot_id, knot_point) in self.knot_locations.iter().enumerate().skip(1).rev() {
            let lin_id = _compute_linear_id(knot_point, &bottom_left, &top_right);
            map_data[lin_id] = char::from_digit(knot_id.try_into().unwrap(), 16).unwrap();
        }

        for line in &map_data.iter().chunks(x_range) {
            let text_line: String = line.collect();
            println!("{}", text_line);
        }
    }
}

fn _compute_linear_id(location: &Point, bottom_left: &Point, top_right: &Point) -> usize {
    let x_range: usize = (top_right.1 - bottom_left.1).try_into().unwrap();
    let normalized_y: usize = (top_right.0 - location.0 - 1).try_into().unwrap();
    let normalized_x: usize = (location.1 - bottom_left.1).try_into().unwrap();
    normalized_y * x_range + normalized_x
}

pub fn part1(input: &str) {
    let mut rope = Rope::new(1);
    for line in input.lines().map(|l| l.trim()) {
        let (_, direction) = instruction_line(line).unwrap();
        rope.move_head(direction);
    }

    println!("Visited locations: {}", rope.visited.len());
}

pub fn part2(input: &str) {
    let mut rope = Rope::new(9);
    for line in input.lines().map(|l| l.trim()) {
        let (_, direction) = instruction_line(line).unwrap();
        rope.move_head(direction);
    }

    println!("Visited locations by the 9th knot: {}", rope.visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_follow() {
        let input: &str = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        let mut test_rope = Rope::new(1);
        for line in input.lines().map(|l| l.trim()) {
            let (_, direction) = instruction_line(line).unwrap();
            test_rope.move_head(direction);
        }

        assert_eq!(test_rope.visited.len(), 13);
    }

    #[test]
    fn simple_long_follow() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        let mut test_rope = Rope::new(9);
        for line in input.lines().map(|l| l.trim()) {
            let (_, direction) = instruction_line(line).unwrap();
            test_rope.move_head(direction);
        }

        assert_eq!(test_rope.visited.len(), 1);
    }

    #[test]
    fn long_follow() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";

        let mut test_rope = Rope::new(9);
        for line in input.lines().map(|l| l.trim()) {
            let (_, direction) = instruction_line(line).unwrap();
            test_rope.move_head(direction);
        }

        assert_eq!(test_rope.visited.len(), 36);
    }
}
