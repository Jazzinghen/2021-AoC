use std::cmp::{max, min};

use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use crate::aoc_lib::jazz_parser;

// Point type
type Point = (i32, i32);

struct Trench {
    top_left: Point,
    bottom_right: Point,
}

impl Trench {
    pub fn new(x_range: &(i32, i32), y_range: &(i32, i32)) -> Trench {
        let top_left = (min(x_range.0, x_range.1), max(y_range.0, y_range.1));
        let bottom_right = (max(x_range.0, x_range.1), min(y_range.0, y_range.1));

        Trench {
            top_left,
            bottom_right,
        }
    }

    fn best_x(&self) -> i32 {
        let mut curr_x_speed = 0i32;
        while curr_x_speed * (curr_x_speed + 1i32) / 2i32 < self.top_left.0 {
            curr_x_speed += 1i32;
        }

        curr_x_speed
    }

    pub fn coolest_speed(&self) -> Point {
        (self.best_x(), -(self.bottom_right.1 + 1i32))
    }
}

// Edge parser
fn target(input: &str) -> IResult<&str, Trench> {
    let (remain_str, (x_range, y_range)) = preceded(
        tag("target area: "),
        separated_pair(
            preceded(
                tag("x="),
                separated_pair(jazz_parser::i32, tag(".."), jazz_parser::i32),
            ),
            tag(", "),
            preceded(
                tag("y="),
                separated_pair(jazz_parser::i32, tag(".."), jazz_parser::i32),
            ),
        ),
    )(input)?;

    Ok((remain_str, Trench::new(&x_range, &y_range)))
}

pub fn part1(input: &str) {
    let (_, target_trench) = target(input).unwrap();
    let start_v = target_trench.coolest_speed();
    let max_height: i32 = start_v.1 * (start_v.1 + 1i32) / 2i32;
    println!("Maximum height for provided trench: {}", max_height);
}

// pub fn part2(input: &str) {
//     let cave_net = CaveNetwork::new(input);
//     let path_count = cave_net.find_paths(true);
//     println!(
//         "Amount of unique paths to the exit considering repeating caves: {}",
//         path_count
//     );
// }

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::iter::FromIterator;

    impl Trench {
        pub fn is_point_inside(&self, point: &Point) -> bool {
            (point.0 >= self.top_left.0 && point.0 <= self.bottom_right.0)
                && (point.1 <= self.top_left.1 && point.1 >= self.bottom_right.1)
        }
    }

    fn compute_trajectory(initial_velocities: &Point, target: &Trench) -> Vec<Point> {
        let mut trajectory_steps: Vec<Point> = Vec::new();
        let mut current_location: Point = (0i32, 0i32);
        let mut simulation_step: u16 = 0u16;

        while (current_location.0 < target.bottom_right.0
            && current_location.1 > target.bottom_right.1)
            && !target.is_point_inside(&current_location)
        {
            let x_step = max(initial_velocities.0 - i32::from(simulation_step), 0i32);
            let y_step = initial_velocities.1 - i32::from(simulation_step);
            current_location.0 += x_step;
            current_location.1 += y_step;

            simulation_step += 1u16;
            trajectory_steps.push(current_location);
        }

        trajectory_steps
    }

    fn _print_trajectory(steps: &[Point], target_trench: &Trench) {
        let mut curr_y = *steps.iter().map(|(_, y)| y).max().unwrap();
        let min_y = min(steps.last().unwrap().1, target_trench.bottom_right.1);
        let max_x = max(steps.last().unwrap().0, target_trench.bottom_right.0);

        let steps_set: HashSet<&Point> = HashSet::from_iter(steps.iter());

        while curr_y >= min_y {
            for col in 0i32..=max_x {
                let curr_coords: Point = (col, curr_y);
                if col == 0i32 && curr_y == 0i32 {
                    print!("S");
                } else if steps_set.contains(&curr_coords) {
                    print!("#");
                } else if target_trench.is_point_inside(&curr_coords) {
                    print!("T");
                } else {
                    print!(".");
                }
            }

            println!();
            curr_y -= 1i32;
        }
    }

    #[test]
    fn trajectory_test() {
        let input_string = "target area: x=20..30, y=-10..-5";
        let (_, target_trench) = target(input_string).unwrap();
        let start_v: Point = (7i32, 2i32);

        let reference_trajectory: Vec<Point> = vec![
            (7i32, 2i32),
            (13i32, 3i32),
            (18i32, 3i32),
            (22i32, 2i32),
            (25i32, 0i32),
            (27i32, -3i32),
            (28i32, -7i32),
        ];

        let traj = compute_trajectory(&start_v, &target_trench);

        assert_eq!(traj, reference_trajectory);

        let start_v: Point = (6i32, 3i32);
        let reference_trajectory: Vec<Point> = vec![
            (6i32, 3i32),
            (11i32, 5i32),
            (15i32, 6i32),
            (18i32, 6i32),
            (20i32, 5i32),
            (21i32, 3i32),
            (21i32, 0i32),
            (21i32, -4i32),
            (21i32, -9i32),
        ];

        let traj = compute_trajectory(&start_v, &target_trench);

        assert_eq!(traj, reference_trajectory);

        let start_v: Point = (9i32, 0i32);
        let reference_trajectory: Vec<Point> =
            vec![(9i32, 0i32), (17i32, -1i32), (24i32, -3i32), (30i32, -6i32)];

        let traj = compute_trajectory(&start_v, &target_trench);

        assert_eq!(traj, reference_trajectory);

        let start_v: Point = (17i32, -4i32);
        let reference_trajectory: Vec<Point> = vec![(17i32, -4i32), (33i32, -9i32)];

        let traj = compute_trajectory(&start_v, &target_trench);

        assert_eq!(traj, reference_trajectory);
    }

    #[test]
    fn compute_max_height() {
        let input_string = "target area: x=20..30, y=-10..-5";
        let (_, target_trench) = target(input_string).unwrap();
        let start_v: Point = target_trench.coolest_speed();

        let max_height: i32 = start_v.1 * (start_v.1 + 1i32) / 2i32;

        assert_eq!(max_height, 45i32);
    }
}
