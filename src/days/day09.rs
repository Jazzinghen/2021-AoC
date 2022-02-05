use std::convert::{TryInto};
use std::collections::{HashSet};
use std::fmt;

use itertools::{Itertools, multiunzip};

struct RiskGrid {
    data: Vec<u8>,
    rows: usize,
    columns: usize
}

impl RiskGrid {
    pub fn new(input: &str) -> RiskGrid {
        let grid_rows = input.split_whitespace();
        let mut flat_data = Vec::new();
        let mut row_count = 0usize;
        let mut col_count = 0usize;
        for row in grid_rows {
            row_count += 1;
            col_count = row.len();
            for (col, risk) in row.chars().enumerate() {
                let risk_digit: u8 = risk.to_digit(10).unwrap().try_into().unwrap();
                flat_data.push(risk_digit);
            }
        }

        RiskGrid{data: flat_data, rows: row_count, columns: col_count}
    }

    pub fn get_risk(&self, location: &(usize, usize)) -> u8 {
        let flat_idx = location.0 * self.columns + location.1;
        *self.data.get(flat_idx).expect("Provided location is out of the grid bounds!")
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn find_local_minima(&self) -> Vec<((usize, usize), u8)> {
        let grid_locations = (0..self.rows).cartesian_product(0..self.columns);

        let mut visit_stack: Vec<(usize, usize)> = Vec::new();
        for loc in grid_locations {
            visit_stack.push(loc);
        }

        let mut already_visited: HashSet<(usize, usize)> = HashSet::new();
        let mut minimum_risks: Vec<((usize, usize), u8)> = Vec::new();

        let mut visits = 0usize;

        // Time for that graph exploration. I am using depth-first visit, but I could have used breadth-first as well
        while let Some(next_risk_loc) = visit_stack.pop() {
            let mut minimum_found = true;
            if !already_visited.contains(&next_risk_loc) {
                visits += 1;
                if next_risk_loc.0 > 0 {
                    let upper_location = (next_risk_loc.0 - 1, next_risk_loc.1);
                    if self.get_risk(&next_risk_loc) >= self.get_risk(&upper_location) {
                        minimum_found = false;
                        visit_stack.push(upper_location);
                    };
                };

                if next_risk_loc.0 < self.rows - 1 {
                    let lower_location = (next_risk_loc.0 + 1, next_risk_loc.1);
                    if self.get_risk(&next_risk_loc) >= self.get_risk(&lower_location) {
                        minimum_found = false;
                        visit_stack.push(lower_location);
                    };
                };

                if next_risk_loc.1 > 0 {
                    let left_location = (next_risk_loc.0, next_risk_loc.1 - 1);
                    if self.get_risk(&next_risk_loc) >= self.get_risk(&left_location) {
                        minimum_found = false;
                        visit_stack.push(left_location);
                    };
                };

                if next_risk_loc.1 < self.columns - 1 {
                    let right_location = (next_risk_loc.0, next_risk_loc.1 + 1);
                    if self.get_risk(&next_risk_loc) >= self.get_risk(&right_location) {
                        minimum_found = false;
                        visit_stack.push(right_location);
                    };
                };

                if minimum_found {
                    minimum_risks.push((next_risk_loc, self.get_risk(&next_risk_loc)));
                }

                already_visited.insert(next_risk_loc);
            }
        }

        return minimum_risks;
    }

    // This will be a reverse graph search based on some starting locations
    pub fn find_basin_sizes(&self, minima: &Vec<(usize, usize)>) -> Vec<u64> {
        let mut basins: Vec<u64> = Vec::new();
        for initial_location in minima {
            let mut visit_stack: Vec<(usize, usize)> = Vec::new();
            visit_stack.push(*initial_location);

            let mut already_visited: HashSet<(usize, usize)> = HashSet::new();

            let mut basin_size = 0u64;

            while let Some(next_basin_loc) = visit_stack.pop() {
                if !already_visited.contains(&next_basin_loc) {
                    basin_size += 1;
                    if next_basin_loc.0 > 0 {
                        let upper_location = (next_basin_loc.0 - 1, next_basin_loc.1);
                        if self.get_risk(&upper_location) < 9u8 {
                            visit_stack.push(upper_location);
                        };
                    };

                    if next_basin_loc.0 < self.rows - 1 {
                        let lower_location = (next_basin_loc.0 + 1, next_basin_loc.1);
                        if self.get_risk(&lower_location) < 9u8 {
                            visit_stack.push(lower_location);
                        };
                    };

                    if next_basin_loc.1 > 0 {
                        let left_location = (next_basin_loc.0, next_basin_loc.1 - 1);
                        if self.get_risk(&left_location) < 9u8 {
                            visit_stack.push(left_location);
                        };
                    };

                    if next_basin_loc.1 < self.columns - 1 {
                        let right_location = (next_basin_loc.0, next_basin_loc.1 + 1);
                        if self.get_risk(&right_location) < 9u8 {
                            visit_stack.push(right_location);
                        };
                    };

                    already_visited.insert(next_basin_loc);
                }
            }

            basins.push(basin_size);
        };

        return basins;
    }
}

impl fmt::Display for RiskGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid dimensions: {} x {}", self.rows, self.columns)?;

        for row in 0..self.rows {
            for col in 0..self.columns {
                let risk_location = (row, col);
                write!(f, "{} ", self.get_risk(&risk_location))?;
            }
            writeln!(f, "")?;
        };

        writeln!(f, "")
    }
}

pub fn part1(input: &str) {
    let risk_grid = RiskGrid::new(input);
    let (_, minima_risk): (Vec<_>, Vec<_>) = risk_grid.find_local_minima().iter().cloned().unzip();
    let risk_sum: u64 = minima_risk.iter().fold(0u64, |sum, val| sum + u64::from(*val + 1));
    println!("Sum of minima: {}", risk_sum);
}

pub fn part2(input: &str) {
    //let min_consumption = min_crab_fuel(input, linear_delta);
    //println!("Estimated minimum geometric cost: {}", min_consumption);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_points() {
        let input_string = "2199943210
                                 3987894921
                                 9856789892
                                 8767896789
                                 9899965678";

        let risk_grid = RiskGrid::new(input_string);
        let (_, minima_risk): (Vec<_>, Vec<_>) = risk_grid.find_local_minima().iter().cloned().unzip();
        let risk_sum: u64 = minima_risk.iter().fold(0u64, |sum, val| sum + u64::from(*val + 1));

        assert_eq!(risk_sum, 15u64);
    }

    #[test]
    fn basins() {
        let input_string = "2199943210
                                 3987894921
                                 9856789892
                                 8767896789
                                 9899965678";

        let risk_grid = RiskGrid::new(input_string);
        let (minima_locations, _): (Vec<_>, Vec<_>) = risk_grid.find_local_minima().iter().cloned().unzip();
        let basins = risk_grid.find_basin_sizes(&minima_locations);
        let top_basins = basins.iter().sorted().rev().take(3);
        let basin_area = top_basins.fold(1u64, |total, area| total * area);

        assert_eq!(basin_area, 1134u64);
    }
}