use std::cmp::{Ordering};
use std::convert::{TryInto};
use std::collections::{BinaryHeap, HashSet};
use std::fmt;

type Point = (usize, usize);
type GridCoord = (u8, u8);

// Exploration front
#[derive(Eq)]
struct RiskStep {
    location: Point,
    grid_location: GridCoord,
    a_star_risk: u64,
    risk: u64,
    visit_history: Vec<(Point, u8, u64)>
}

impl Ord for RiskStep {
    // We need the comparison to be inverse to use it in the A* sorted heap and minimize risk
    fn cmp(&self, other: &Self) -> Ordering {
        other.a_star_risk.cmp(&self.a_star_risk)
    }
}

impl PartialOrd for RiskStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RiskStep {
    fn eq(&self, other: &Self) -> bool {
        self.a_star_risk == other.a_star_risk
    }
}

// Grid
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
        for row_string in grid_rows {
            row_count += 1;
            col_count = row_string.len();
            for risk in row_string.chars() {
                let risk_digit: u8 = risk.to_digit(10).unwrap().try_into().unwrap();
                flat_data.push(risk_digit);
            }
        }

        RiskGrid{data: flat_data, rows: row_count, columns: col_count}
    }

    fn compute_flat_idx(&self, location: &Point) -> usize {
        location.0 * self.columns + location.1
    }

    fn get_risk(&self, location: &Point, grid_coord: &GridCoord) -> u8 {
        let flat_idx = self.compute_flat_idx(location);
        let base_risk = self.data.get(flat_idx).expect("Provided location is out of the grid bounds!");
        let total_risk = base_risk + grid_coord.0 + grid_coord.1;
        let extra = (total_risk / 10u8) * 9u8;
        return total_risk - extra;
    }

    fn get_neighbours(&self, location: &Point, grid_coord: &GridCoord, max_coords: &GridCoord) -> Vec<((Point, GridCoord), u8)> {
        let mut neighbours: Vec<((Point, GridCoord), u8)> = Vec::new();

        if location.0 > 0 {
            let neigh_coord: Point = (location.0 - 1, location.1);
            let neigh_risk = self.get_risk(&neigh_coord, grid_coord);
            neighbours.push(((neigh_coord, *grid_coord), neigh_risk));
        } else if grid_coord.0 > 0 {
            let neigh_coord: Point = (self.rows - 1, location.1);
            let neigh_grid: GridCoord = (grid_coord.0 - 1, grid_coord.1);
            let neigh_risk = self.get_risk(&neigh_coord, &neigh_grid);
            neighbours.push(((neigh_coord, neigh_grid), neigh_risk));
        };

        if location.0 < self.rows - 1 {
            let neigh_coord: Point = (location.0 + 1, location.1);
            let neigh_risk = self.get_risk(&neigh_coord, grid_coord);
            neighbours.push(((neigh_coord, *grid_coord), neigh_risk));
        } else if grid_coord.0 < max_coords.0 {
            let neigh_coord: Point = (0usize, location.1);
            let neigh_grid: GridCoord = (grid_coord.0 + 1, grid_coord.1);
            let neigh_risk = self.get_risk(&neigh_coord, &neigh_grid);
            neighbours.push(((neigh_coord, neigh_grid), neigh_risk));
        };

        if location.1 > 0 {
            let neigh_coord: Point = (location.0, location.1 - 1);
            let neigh_risk = self.get_risk(&neigh_coord, grid_coord);
            neighbours.push(((neigh_coord, *grid_coord), neigh_risk));
        } else if grid_coord.1 > 0 {
            let neigh_coord: Point = (location.0, self.columns - 1);
            let neigh_grid: GridCoord = (grid_coord.0, grid_coord.1 - 1);
            let neigh_risk = self.get_risk(&neigh_coord, &neigh_grid);
            neighbours.push(((neigh_coord, neigh_grid), neigh_risk));
        };

        if location.1 < self.columns - 1 {
            let neigh_coord: Point = (location.0, location.1 + 1);
            let neigh_risk = self.get_risk(&neigh_coord, grid_coord);
            neighbours.push(((neigh_coord, *grid_coord), neigh_risk));
        } else if grid_coord.1 < max_coords.1 {
            let neigh_coord: Point = (location.0, 0usize);
            let neigh_grid: GridCoord = (grid_coord.0, grid_coord.1 + 1);
            let neigh_risk = self.get_risk(&neigh_coord, &neigh_grid);
            neighbours.push(((neigh_coord, neigh_grid), neigh_risk));
        };

        return neighbours;
    }

    fn compute_a_star_coord(&self, grid_space_coord: &Point, grid_coord: &GridCoord) -> Point {
        let actual_a_star_col= usize::from(grid_coord.1) * self.columns + grid_space_coord.1;
        let actual_a_star_row = usize::from(grid_coord.0) * self.rows + grid_space_coord.0;

        (actual_a_star_row, actual_a_star_col)
    }

    pub fn find_lowest_risk_path(&self, start: &(Point, GridCoord), end: &(Point, GridCoord)) -> u64{
        let mut visited_locations: HashSet<Point> = HashSet::new();
        let mut exploration_front: BinaryHeap<RiskStep> = BinaryHeap::new();

        let actual_a_start = self.compute_a_star_coord(&start.0, &start.1);
        let actual_a_end = self.compute_a_star_coord(&end.0, &end.1);

        let max_grid_coords: GridCoord = end.1;

        let a_start_start: u64 = ((actual_a_end.0 - actual_a_start.0) + (actual_a_end.1 - actual_a_start.1)).try_into().unwrap();
        let first_step = RiskStep{location: start.0, grid_location: start.1, a_star_risk: a_start_start, risk: 0u64, visit_history: vec![(start.0, 0u8, 0u64)]};
        exploration_front.push(first_step);

        while let Some(current_step) = exploration_front.pop() {
            let actual_a_star_location = self.compute_a_star_coord(&current_step.location, &current_step.grid_location);
            visited_locations.insert(actual_a_star_location);

            let neighbours = self.get_neighbours(&current_step.location, &current_step.grid_location, &max_grid_coords);
            for ((coord, grid), risk) in neighbours {
                let a_star_coord = self.compute_a_star_coord(&coord, &grid);
                if coord == end.0 && grid == end.1 {
                    return current_step.risk + u64::from(risk);
                }
                if !visited_locations.contains(&a_star_coord) {
                    let distance_cost: u64 = ((actual_a_end.0 - a_star_coord.0) + (actual_a_end.1 - a_star_coord.1)).try_into().unwrap();
                    let next_risk: u64 = current_step.risk + u64::from(risk);
                    let mut next_history = current_step.visit_history.clone();
                    next_history.push((a_star_coord, risk, next_risk));
                    exploration_front.push(RiskStep{location: coord, grid_location: grid, a_star_risk: next_risk + distance_cost, risk: next_risk, visit_history: next_history});
                }
            }
        }

        panic!("We shouldn't get here without reaching the end.");
    }
}

impl fmt::Display for RiskGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid dimensions: {} x {}", self.rows, self.columns)?;

        let grid_coords: GridCoord = (0u8, 0u8);
        for row in 0..self.rows {
            for col in 0..self.columns {
                let risk_location = (row, col);
                write!(f, "{} ", self.get_risk(&risk_location, &grid_coords))?;
            }
            writeln!(f, "")?;
        };
        writeln!(f, "")
    }
}

pub fn part1(input: &str) {
    let risk_grid = RiskGrid::new(input);
    let start: Point = (0, 0);
    let end: Point = (risk_grid.rows - 1, risk_grid.columns - 1);
    let grid_coord: GridCoord = (0u8, 0u8);
    println!("Lowest risk path sum: {}", risk_grid.find_lowest_risk_path(&(start, grid_coord), &(end, grid_coord)));
}

pub fn _part2(_input: &str) {
    // let mut octo_grid = OctopusGrid::new(input);
    // let synchronization_flash = octo_grid.first_coordinated_flash();
    // println!("First synchronized step: {}", synchronization_flash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_out_path() {
        let input_string = "1163751742
                            1381373672
                            2136511328
                            3694931569
                            7463417111
                            1319128137
                            1359912421
                            3125421639
                            1293138521
                            2311944581";

        let risk_grid = RiskGrid::new(input_string);
        let start: Point = (0, 0);
        let end: Point = (risk_grid.rows - 1, risk_grid.columns - 1);
        let grid_coord: GridCoord = (0u8, 0u8);

        assert_eq!(risk_grid.find_lowest_risk_path(&(start, grid_coord), &(end, grid_coord)), 40u64);
    }

    #[test]
    fn full_grid_path() {
        let input_string = "1163751742
                            1381373672
                            2136511328
                            3694931569
                            7463417111
                            1319128137
                            1359912421
                            3125421639
                            1293138521
                            2311944581";

        let risk_grid = RiskGrid::new(input_string);
        let start: Point = (0, 0);
        let end: Point = (risk_grid.rows - 1, risk_grid.columns - 1);
        let start_grid_coord: GridCoord = (0u8, 0u8);
        let end_grid_coord: GridCoord = (4u8, 4u8);

        assert_eq!(risk_grid.find_lowest_risk_path(&(start, start_grid_coord), &(end, end_grid_coord)), 315u64);
    }
}