use ansi_term::Colour;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use hashbrown::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellType {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Segment {
    start: Point,
    end: Point,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(digit1, char(','), digit1),
        |(x, y): (&str, &str)| Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    )(input)
}

fn parse_rock_sequence(input: &str) -> IResult<&str, Vec<Segment>> {
    map(separated_list1(tag(" -> "), parse_point), |points| {
        points
            .iter()
            .tuple_windows()
            .map(|(start, end)| Segment {
                start: *start,
                end: *end,
            })
            .collect_vec()
    })(input)
}

#[derive(Debug)]
struct Cave {
    cells: Vec<CellType>,
    bottom_left: Point,
    top_right: Point,
    final_flow: Option<HashSet<Point>>,
}

impl Cave {
    fn compute_linear_id(&self, coordinate: Point) -> Result<usize, &str> {
        if coordinate.x > self.top_right.x
            || coordinate.x < self.bottom_left.x
            || coordinate.y > self.bottom_left.y
        {
            return Err("Coordinate is outside of the boundaries!");
        }

        let norm_x = coordinate.x - self.bottom_left.x;
        Ok(norm_x * (self.bottom_left.y + 1) + coordinate.y)
    }

    fn from_raw_segments(input: &str, infinite: bool) -> Self {
        let mut rock_segments: HashSet<Segment> = input
            .lines()
            .flat_map(|l| parse_rock_sequence(l.trim()).unwrap().1)
            .collect();

        let important_points: HashSet<Point> = rock_segments
            .iter()
            .flat_map(|seg| [seg.start, seg.end])
            .collect();

        let bottom = important_points
            .iter()
            .max_by(|left, right| left.y.cmp(&right.y))
            .unwrap()
            .y
            + if infinite { 0 } else { 2 };
        let leftmost_point = important_points
            .iter()
            .min_by(|left, right| left.x.cmp(&right.x))
            .unwrap()
            .x;
        let rightmost_point = important_points
            .iter()
            .max_by(|left, right| left.x.cmp(&right.x))
            .unwrap()
            .x;

        let left = if infinite {
            leftmost_point - 1
        } else {
            (500 - bottom).min(leftmost_point) - 1
        };
        let right = if infinite {
            rightmost_point + 1
        } else {
            (500 + bottom).max(rightmost_point) + 1
        };

        let y_range = bottom + 1;
        let x_range = (right - left) + 1;

        if !infinite {
            let floor = Segment {
                start: Point { x: left, y: bottom },
                end: Point {
                    x: right,
                    y: bottom,
                },
            };
            rock_segments.insert(floor);
        }

        let mut new_cave = Self {
            cells: vec![CellType::Empty; x_range * y_range],
            top_right: Point { x: right, y: 0 },
            bottom_left: Point { x: left, y: bottom },
            final_flow: None,
        };

        for segment in rock_segments.iter() {
            let start_x = segment.start.x.min(segment.end.x);
            let end_x = segment.start.x.max(segment.end.x);
            let start_y = segment.start.y.min(segment.end.y);
            let end_y = segment.start.y.max(segment.end.y);
            for cell_coord in (start_y..=end_y)
                .cartesian_product(start_x..=end_x)
                .map(|(y, x)| Point { x, y })
            {
                let cell_id = new_cave.compute_linear_id(cell_coord).unwrap();
                new_cave.cells[cell_id] = CellType::Rock;
            }
        }

        new_cave
    }

    fn simulate_one_step(&mut self) -> bool {
        let mut sand_coord = Point { x: 500, y: 0 };
        let mut maybe_flow: HashSet<Point> = HashSet::new();

        while let Some(new_y) = self.find_furthest_free_y(sand_coord) {
            maybe_flow.extend((sand_coord.y..=new_y).map(|y| Point { x: sand_coord.x, y }));
            let down_left = Point {
                x: sand_coord.x - 1,
                y: new_y + 1,
            };
            let down_left_id = self.compute_linear_id(down_left).unwrap();

            let down_right = Point {
                x: sand_coord.x + 1,
                y: new_y + 1,
            };
            let down_right_id = self.compute_linear_id(down_right).unwrap();

            if self.cells[down_left_id] == CellType::Empty {
                sand_coord = down_left
            } else if self.cells[down_right_id] == CellType::Empty {
                sand_coord = down_right;
            } else {
                sand_coord.y = new_y;
                let sand_id = self.compute_linear_id(sand_coord).unwrap();
                self.cells[sand_id] = CellType::Sand;
                return !(sand_coord.x == 500 && sand_coord.y == 0);
            }
        }

        maybe_flow
            .extend((sand_coord.y..=self.bottom_left.y).map(|y| Point { x: sand_coord.x, y }));
        self.final_flow = Some(maybe_flow);
        false
    }

    pub fn simulate(&mut self, steps: Option<usize>) {
        if let Some(s) = steps {
            for _ in 0..s {
                self.simulate_one_step();
            }
        } else {
            let mut keep_on_simulating = true;
            while keep_on_simulating {
                keep_on_simulating = self.simulate_one_step();
            }
        }
    }

    pub fn find_furthest_free_y(&self, start: Point) -> Option<usize> {
        self.cells
            .iter()
            .skip(self.compute_linear_id(start).unwrap())
            .take((self.bottom_left.y - start.y) + 1)
            .position(|cell| *cell != CellType::Empty)
            .map(|y| start.y + y - 1)
    }

    fn print_cave_visual(&self) {
        for point in (0..=self.bottom_left.y)
            .cartesian_product(self.bottom_left.x..=self.top_right.x)
            .map(|(y, x)| Point { x, y })
        {
            if point.x == self.bottom_left.x {
                print!(
                    "{}",
                    Colour::White.bold().paint(format!("{:4} | ", point.y))
                );
            }

            let point_id = self.compute_linear_id(point).unwrap();
            let mut cell_char = match self.cells[point_id] {
                CellType::Empty => " ".to_string(),
                CellType::Rock => Colour::RGB(248, 248, 242).paint("#").to_string(),
                CellType::Sand => Colour::RGB(255, 184, 108).paint("o").to_string(),
            };
            if let Some(flow) = self.final_flow.as_ref() {
                if flow.contains(&point) {
                    cell_char = Colour::RGB(255, 184, 108).paint("~").to_string();
                }
            }
            if point.x == 500 && point.y == 0 {
                cell_char = Colour::Cyan.paint("+").to_string();
            }

            print!("{}", cell_char);

            if point.x == self.top_right.x {
                println!("{}", Colour::White.bold().paint(" |"));
            }
        }
        println!();
    }
}

pub fn part1(input: &str) {
    let mut cave = Cave::from_raw_segments(input, true);

    cave.simulate(None);

    let sand_amount = cave
        .cells
        .iter()
        .filter(|&&cell| cell == CellType::Sand)
        .count();

    cave.print_cave_visual();

    println!("Amount of sand in an unlimited cave: {}", sand_amount);
}

pub fn part2(input: &str) {
    let mut cave = Cave::from_raw_segments(input, false);

    cave.simulate(None);

    let sand_amount = cave
        .cells
        .iter()
        .filter(|&&cell| cell == CellType::Sand)
        .count();

    cave.print_cave_visual();

    println!("Amount of sand in a cave with floor: {}", sand_amount);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn simple_tops() {
        let cave = Cave::from_raw_segments(INPUT_STRING, true);
        let tops = (cave.bottom_left.x..=cave.top_right.x)
            .map(|x| cave.find_furthest_free_y(Point { x, y: 0 }))
            .collect_vec();

        let ref_tops: Vec<Option<usize>> = vec![
            None,
            Some(8),
            Some(8),
            Some(5),
            Some(5),
            Some(3),
            Some(8),
            Some(8),
            Some(8),
            Some(3),
            Some(3),
            None,
        ];

        assert_eq!(tops, ref_tops);
    }

    #[test]
    fn simple_flow() {
        let mut cave = Cave::from_raw_segments(INPUT_STRING, true);

        cave.simulate(None);

        let sand_amount = cave
            .cells
            .iter()
            .filter(|&&cell| cell == CellType::Sand)
            .count();

        assert_eq!(sand_amount, 24);
    }

    #[test]
    fn simple_floor() {
        let mut cave = Cave::from_raw_segments(INPUT_STRING, false);

        cave.simulate(None);

        let sand_amount = cave
            .cells
            .iter()
            .filter(|&&cell| cell == CellType::Sand)
            .count();

        assert_eq!(sand_amount, 93);
    }
}
