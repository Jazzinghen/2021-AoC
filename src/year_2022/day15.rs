use hashbrown::HashSet;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::sequence::{pair, preceded, separated_pair};
use nom::IResult;
use rayon::prelude::*;

use crate::aoc_lib::jazz_parser::usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    location: Point,
    range: u32,
}

impl Sensor {
    pub fn coverage_at(&self, location: Point) -> Result<Option<Segment>, &str> {
        if self.location.x != location.x && self.location.y != location.y {
            return Err("Location not on the same axis!");
        }

        if self.location.x == location.x {
            let delta_y = self.location.y.abs_diff(location.y);
            if delta_y > self.range {
                Ok(None)
            } else {
                let x_range = self.range - delta_y;
                Ok(Some(Segment {
                    start: Point {
                        x: location.x - i32::try_from(x_range).unwrap(),
                        y: location.y,
                    },
                    end: Point {
                        x: location.x + i32::try_from(x_range).unwrap(),
                        y: location.y,
                    },
                }))
            }
        } else {
            let delta_x = self.location.x.abs_diff(location.x);
            if delta_x > self.range {
                Ok(None)
            } else {
                let y_range = self.range - delta_x;
                Ok(Some(Segment {
                    start: Point {
                        x: location.x,
                        y: location.y - i32::try_from(y_range).unwrap(),
                    },
                    end: Point {
                        x: location.x,
                        y: location.y + i32::try_from(y_range).unwrap(),
                    },
                }))
            }
        }
    }
}

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map(
        pair(opt(char('-')), digit1),
        |(sign, value): (Option<char>, &str)| {
            let sign_mul = if sign.is_some() { -1 } else { 1 };
            value.parse::<i32>().unwrap() * sign_mul
        },
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), parse_integer),
            tag(", "),
            preceded(tag("y="), parse_integer),
        ),
        |(x, y)| Point { x, y },
    )(input)
}

fn parse_sensor(input: &str) -> IResult<&str, (Sensor, Point)> {
    map(
        preceded(
            tag("Sensor at "),
            separated_pair(parse_point, tag(": closest beacon is at "), parse_point),
        ),
        |(sensor, beacon)| {
            let delta_x = sensor.x.abs_diff(beacon.x);
            let delta_y = sensor.y.abs_diff(beacon.y);
            (
                Sensor {
                    location: sensor,
                    range: delta_x + delta_y,
                },
                beacon,
            )
        },
    )(input)
}

fn y_coverage(
    y: i32,
    sensors: &[Sensor],
    beacons: &[Point],
    limit: Option<i32>,
) -> (usize, Option<Vec<i32>>) {
    let coverage_parts: Vec<Segment> = sensors
        .iter()
        .filter_map(|sens| {
            sens.coverage_at(Point {
                x: sens.location.x,
                y,
            })
            .unwrap()
        })
        .sorted_by(|left, right| left.start.x.cmp(&right.start.x))
        .collect_vec();

    let mut merged_coverage: Vec<Segment> = Vec::new();
    let mut current_segment: Segment = *coverage_parts.first().unwrap();

    for seg in coverage_parts.iter() {
        if seg.start.x > current_segment.end.x {
            merged_coverage.push(current_segment);
            current_segment = *seg;
        } else {
            current_segment.end.x = current_segment.end.x.max(seg.end.x);
        }
    }

    merged_coverage.push(current_segment);

    if let Some(x_limit) = limit {
        for segment in merged_coverage.iter_mut() {
            segment.start.x = segment.start.x.clamp(0, x_limit);
            segment.end.x = segment.end.x.clamp(0, x_limit);
        }
    }

    let line_beacons: HashSet<Point> = beacons.iter().filter(|&b| b.y == y).cloned().collect();
    let empty_spots = merged_coverage.iter().fold(0usize, |acc, &seg| {
        acc + usize::try_from(seg.end.x.abs_diff(seg.start.x)).unwrap()
            + (seg.start.x <= 0 && seg.end.x > 0) as usize
    });

    let non_empty: HashSet<Point> = merged_coverage
        .iter()
        .flat_map(|seg| {
            line_beacons
                .iter()
                .filter(|&b| b.x >= seg.start.x && b.x <= seg.end.x)
        })
        .cloned()
        .collect();

    if limit.is_none() {
        (empty_spots - non_empty.len(), None)
    } else {
        let possible_points: HashSet<Point> = merged_coverage
            .iter()
            .tuple_windows()
            .flat_map(|(left, right)| (left.end.x + 1..right.start.x).map(|x| Point { x, y }))
            .collect();
        let actual_points: Vec<i32> = possible_points
            .difference(&non_empty)
            .map(|point| point.x)
            .collect_vec();
        (empty_spots - non_empty.len(), Some(actual_points))
    }
}

fn find_distress_point(max: i32, sensors: &[Sensor], beacons: &[Point]) -> Point {
    let single_line = (0..=max)
        .into_par_iter()
        .map(|y| (y, y_coverage(y, sensors, beacons, Some(max)).1))
        .reduce(
            || (0, None),
            |a, b| {
                if let Some(possible_points) = a.1.as_ref() {
                    if possible_points.len() == 1 {
                        a
                    } else {
                        b
                    }
                } else {
                    b
                }
            },
        );

    Point {
        x: single_line.1.unwrap_or_else(|| vec![-1])[0],
        y: single_line.0,
    }
}

pub fn part1(input: &str) {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    for (s, b) in input.lines().map(|l| parse_sensor(l.trim()).unwrap().1) {
        sensors.push(s);
        beacons.push(b);
    }

    let (empty_cells, _) = y_coverage(2000000, &sensors, &beacons, None);

    println!("Amount of empty spots on line 2000000: {}", empty_cells);
}

pub fn part2(input: &str) {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    for (s, b) in input.lines().map(|l| parse_sensor(l.trim()).unwrap().1) {
        sensors.push(s);
        beacons.push(b);
    }

    let point = find_distress_point(4000000, &sensors, &beacons);

    let disdress_frequency =
        usize::try_from(point.x).unwrap() * 4000000 + usize::try_from(point.y).unwrap();

    println!("Distress signal frequency: {}", disdress_frequency);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn simple_coverage() {
        let mut sensors: Vec<Sensor> = Vec::new();
        let mut beacons: Vec<Point> = Vec::new();

        for (s, b) in INPUT_STRING
            .lines()
            .map(|l| parse_sensor(l.trim()).unwrap().1)
        {
            sensors.push(s);
            beacons.push(b);
        }

        let (empty_cells, _) = y_coverage(10, &sensors, &beacons, None);

        assert_eq!(empty_cells, 26);
    }

    #[test]
    fn simple_single_distress() {
        let mut sensors: Vec<Sensor> = Vec::new();
        let mut beacons: Vec<Point> = Vec::new();

        for (s, b) in INPUT_STRING
            .lines()
            .map(|l| parse_sensor(l.trim()).unwrap().1)
        {
            sensors.push(s);
            beacons.push(b);
        }

        let point = find_distress_point(20, &sensors, &beacons);

        assert_eq!(point, Point { x: 14, y: 11 });
        assert_eq!(point.x * 4000000 + point.y, 56000011);
    }
}
