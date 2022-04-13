use hashbrown::HashSet;
use std::cmp::{max, min};
use std::iter::FromIterator;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use crate::aoc_lib::jazz_parser;

// This problem was simple in my mind, but then my tendency of screwing up loop-based algorithms with
// "by one" errors is legendary

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

    // This function uses the formula to compute the sum of the first n natural numbers to find the first
    // x velocity that will get to 0 once in range of the trench. This works because x velocities reach 0
    // and cannot go lower, so the final distance = sum(ini_x, ini_x - 1, .., 1, 0)
    // Using this value we can go crazy with the y velocities, as we won't overshoot horizontally the
    // trench no matter how high we go
    fn best_x(&self) -> i32 {
        let mut curr_x_speed = 0i32;
        while curr_x_speed * (curr_x_speed + 1i32) / 2i32 < self.top_left.0 {
            curr_x_speed += 1i32;
        }

        curr_x_speed
    }

    // After running a ton of computations I stopped and thought about how the ship moves. To end in the
    // trench we just need to be at `min_y + 1` y speed when we are at `y = 0` (No matter the time step
    // thanks to the zero x computation), as then the next step below 0 will be `min_y`
    // Since the speed follows a parabolic trajectory we just need to set the starting velocity to the
    // negative of the target speed at `y = 0`
    pub fn coolest_speed(&self) -> Point {
        (self.best_x(), -(self.bottom_right.1 + 1i32))
    }

    // This function computes ALL the initial vertical velocities and how many steps will take the ship to reach the
    // trench height.
    // The function first computes all the speeds that will get to the trench directly and in how many steps and use
    // that value to compute the time required for the opposite speed (i.e. how much to follow the parabolic trajectory)
    fn get_potential_y_velocities(&self) -> Vec<(i32, i32)> {
        let max_y = -(self.bottom_right.1 + 1i32);
        let mut available_velocities: Vec<(i32, i32)> = vec![(self.bottom_right.1, 1i32)];
        for start_y in 0i32..=max_y {
            let mut y_check = 0i32;
            let mut step = 0i32;
            while y_check > self.top_left.1 {
                y_check -= start_y + step;
                step += 1i32;
            }

            while y_check >= self.bottom_right.1 {
                available_velocities.push((-start_y, step));
                available_velocities.push((start_y, start_y * 2 + step));
                y_check -= start_y + step;
                step += 1i32;
            }
        }

        available_velocities
    }

    // This is the same as "best_x" but computes all the possible initial horizontal velocities
    // that will make the ship hit the trench with 0 horizontal speed
    fn get_all_zero_x(&self) -> Vec<i32> {
        let mut curr_x_speed = self.best_x();
        let mut zero_x_velocities: Vec<i32> = Vec::new();
        while curr_x_speed * (curr_x_speed + 1i32) / 2i32 <= self.bottom_right.0 {
            zero_x_velocities.push(curr_x_speed);
            curr_x_speed += 1i32;
        }

        zero_x_velocities
    }

    // This function computes all the initial horizontal velocities that will make the ship
    // hit the trench in `steps` time
    fn get_x_velocities(&self, steps: i32) -> Vec<i32> {
        let mut x_vels: Vec<i32> = Vec::new();

        for pot_vel in steps..=self.bottom_right.0 {
            let final_x_vel = pot_vel - steps;
            let final_x =
                ((pot_vel + 1i32) * pot_vel) / 2i32 - ((final_x_vel + 1i32) * final_x_vel) / 2i32;

            if final_x >= self.top_left.0 && final_x <= self.bottom_right.0 {
                x_vels.push(pot_vel);
            }
        }

        x_vels
    }

    // Since we can shoot the ship directly at the trench we can use all the coordinates of the target
    // trench as initial velocities
    fn one_step_velocities(&self) -> Vec<Point> {
        let platform_locations = (self.top_left.0..=self.bottom_right.0)
            .cartesian_product(self.bottom_right.1..=self.top_left.1);

        platform_locations.collect_vec()
    }

    // This is pretty straightforward:
    //   - Get all the kown velocities
    //   - Get all the working y velocities and how much they take to reach the trench
    //      - For each of these velocities find the x velocities that reach the trench in that amount of steps
    //      - Add the velocity vector to the set
    //   - Get all the initial horizontal velocities that make the ship reach the trench with `vel_x = 0`
    //      - Combine each of these velocities with all the y velocities that take `initial_x` steps or more
    //        to reach the trench
    //      - Add these vectors to the set
    pub fn compute_initial_velocities(&self) -> HashSet<Point> {
        let mut velocities: HashSet<Point> =
            HashSet::from_iter(self.one_step_velocities().into_iter());

        let all_initial_y_vels = self.get_potential_y_velocities();

        for (vel, steps) in all_initial_y_vels.iter() {
            let x_vels = self.get_x_velocities(*steps);
            let vel_combos = x_vels.into_iter().map(|x| (x, *vel));
            velocities.extend(vel_combos);
        }

        let zero_x_vels = self.get_all_zero_x();

        for x in zero_x_vels {
            for (y, _) in all_initial_y_vels.iter().filter(|(_, steps)| *steps >= x) {
                velocities.insert((x, *y));
            }
        }

        velocities
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

pub fn part2(input: &str) {
    let (_, target_trench) = target(input).unwrap();
    let initial_velocities: HashSet<Point> = target_trench.compute_initial_velocities();
    println!("Amount of initial velocities: {}", initial_velocities.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::character::complete::multispace1;
    use nom::multi::separated_list1;

    impl Trench {
        pub fn is_point_inside(&self, point: &Point) -> bool {
            (point.0 >= self.top_left.0 && point.0 <= self.bottom_right.0)
                && (point.1 <= self.top_left.1 && point.1 >= self.bottom_right.1)
        }
    }

    fn vel(input: &str) -> IResult<&str, Point> {
        let (remain_str, (vel_x, vel_y)) =
            separated_pair(jazz_parser::i32, tag(","), jazz_parser::i32)(input)?;
        Ok((remain_str, (vel_x, vel_y)))
    }

    fn velocities(input: &str) -> IResult<&str, Vec<Point>> {
        separated_list1(multispace1, vel)(input)
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

    #[test]
    fn compute_available_y() {
        let input_string = "target area: x=20..30, y=-10..-5";
        let (_, target_trench) = target(input_string).unwrap();

        let ref_y_velocities: HashSet<i32> = HashSet::from_iter(vec![
            -8, 4, -5, -3, -9, -7, 6, 9, -4, 5, 7, -10, 0, 2, 8, -2, -1, 1, -6, 3,
        ]);

        let computed_y_vels: HashSet<i32> = HashSet::from_iter(
            target_trench
                .get_potential_y_velocities()
                .into_iter()
                .map(|(y, _)| y),
        );

        assert_eq!(computed_y_vels, ref_y_velocities);
    }

    #[test]
    fn compute_all_start_velocities() {
        let input_string = "target area: x=20..30, y=-10..-5";
        let (_, target_trench) = target(input_string).unwrap();

        let input_result = "23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
            25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
            8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
            26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
            20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
            25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
            25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
            8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
            24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
            7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
            23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
            27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
            8,-2    27,-8   30,-5   24,-7";

        let (_, reference_velocities) = velocities(input_result).unwrap();

        let reference_set: HashSet<Point> = HashSet::from_iter(reference_velocities.into_iter());

        let initial_velocities: HashSet<Point> = target_trench.compute_initial_velocities();

        assert_eq!(initial_velocities, reference_set);
    }
}
