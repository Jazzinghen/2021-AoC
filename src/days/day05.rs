use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::aoc_lib::jazz_parser;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

impl Line {
    fn new(start: &Point, end: &Point) -> Line {
        let line_dir = if start.x == end.x {
            Direction::Vertical
        } else if start.y == end.y {
            Direction::Horizontal
        } else {
            Direction::Diagonal
        };
        let (actual_start, actual_end) = match line_dir {
            Direction::Vertical => {
                if start.y > end.y {
                    (end, start)
                } else {
                    (start, end)
                }
            }
            _ => {
                if start.x > end.x {
                    (end, start)
                } else {
                    (start, end)
                }
            }
        };

        if line_dir == Direction::Diagonal {
            let x_delta = if actual_end.x < actual_start.x {
                actual_start.x - actual_end.x
            } else {
                actual_end.x - actual_start.x
            };
            let y_delta = if actual_end.y < actual_start.y {
                actual_start.y - actual_end.y
            } else {
                actual_end.y - actual_start.y
            };
            assert_eq!(
                x_delta, y_delta,
                "Diagonal lines should have a 45 degree slope!"
            );
        }

        Line {
            start: *actual_start,
            end: *actual_end,
            direction: line_dir,
        }
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    let parser = separated_pair(jazz_parser::usize, tag(","), jazz_parser::usize);
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        Point::new(s.0, s.1)
    })(input)
}

fn segment(input: &str) -> IResult<&str, Line> {
    let parser = separated_pair(point, tag(" -> "), point);
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        Line::new(&s.0, &s.1)
    })(input)
}

pub fn part1(input: &str) {
    let intersections = intersection_check(input, false);
    println!("Number of intersections: {}", intersections);
}

pub fn part2(input: &str) {
    let intersections = intersection_check(input, true);
    println!("Number of intersections: {}", intersections);
}

fn intersection_check(input: &str, enable_diagonals: bool) -> i32 {
    let line_input = input.lines();

    let mut occupation_map = HashMap::<(usize, usize), usize>::new();
    let mut intersecting_points = 0;

    for line in line_input {
        let segment = segment(line).expect("Didn't manage to parse the input!").1;

        match segment.direction {
            Direction::Horizontal => {
                for col in segment.start.x..=segment.end.x {
                    let int_count = occupation_map.entry((col, segment.end.y)).or_insert(0);
                    *int_count += 1;
                    if *int_count == 2 {
                        intersecting_points += 1;
                    }
                }
            }
            Direction::Vertical => {
                for row in segment.start.y..=segment.end.y {
                    let int_count = occupation_map.entry((segment.end.x, row)).or_insert(0);
                    *int_count += 1;
                    if *int_count == 2 {
                        intersecting_points += 1;
                    }
                }
            }
            Direction::Diagonal => {
                if enable_diagonals {
                    let mut row = segment.start.y;
                    let row_direction = segment.end.y > row;
                    let mut col = segment.start.x;

                    while col <= segment.end.x {
                        let int_count = occupation_map.entry((col, row)).or_insert(0);
                        *int_count += 1;
                        if *int_count == 2 {
                            intersecting_points += 1;
                        }

                        col += 1;
                        row = if row_direction {
                            row + 1
                        } else {
                            row.checked_sub(1).unwrap_or_default()
                        };
                    }
                }
            }
        }
    }

    intersecting_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_diagonal() {
        let input_string = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let intersections = intersection_check(input_string, false);

        assert_eq!(intersections, 5);
    }

    #[test]
    fn diagonal() {
        let input_string = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let intersections = intersection_check(input_string, true);

        assert_eq!(intersections, 12);
    }
}
