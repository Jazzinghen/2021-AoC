use std::cmp::Ordering;
use std::convert::{TryInto};
use std::collections::{BinaryHeap, HashSet};
use std::fmt;

type Point = (usize, usize);

// Exploration front
#[derive(Eq)]
struct RiskStep {
    location: Point,
    a_star_risk: u64,
    risk: u64
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

    fn get_risk(&self, location: &Point) -> u8 {
        let flat_idx = self.compute_flat_idx(location);
        *self.data.get(flat_idx).expect("Provided location is out of the grid bounds!")
    }

    fn get_neighbours(&self, location: &Point) -> Vec<(Point, u8)> {
        let mut neighbours: Vec<(Point, u8)> = Vec::new();

        if location.0 > 0 {
            let neigh_coord: Point = (location.0 - 1, location.1);
            let neigh_risk = self.get_risk(&neigh_coord);
            neighbours.push((neigh_coord, neigh_risk));
        };

        if location.0 < self.rows - 1 {
            let neigh_coord: Point = (location.0 + 1, location.1);
            let neigh_risk = self.get_risk(&neigh_coord);
            neighbours.push((neigh_coord, neigh_risk));
        };

        if location.1 > 0 {
            let neigh_coord: Point = (location.0, location.1 - 1);
            let neigh_risk = self.get_risk(&neigh_coord);
            neighbours.push((neigh_coord, neigh_risk));
        };

        if location.1 < self.columns - 1 {
            let neigh_coord: Point = (location.0, location.1 + 1);
            let neigh_risk = self.get_risk(&neigh_coord);
            neighbours.push((neigh_coord, neigh_risk));
        };

        return neighbours;
    }

    pub fn find_lowest_risk_path(self, start: &Point, end: &Point) -> u64{
        let mut visited_locations: HashSet<Point> = HashSet::new();
        let mut exploration_front: BinaryHeap<RiskStep> = BinaryHeap::new();

        let a_start_start: u64 = ((end.0 - start.0) + (end.1 - start.1)).try_into().unwrap();
        let first_step = RiskStep{location: *start, a_star_risk: a_start_start, risk: 0u64};
        exploration_front.push(first_step);

        while let Some(current_step) = exploration_front.pop() {
            visited_locations.insert(current_step.location);
            let neighbours = self.get_neighbours(&current_step.location);
            for (coord, risk) in neighbours {
                if coord == *end {
                    return current_step.risk + u64::from(risk);
                }
                if !visited_locations.contains(&coord) {
                    let distance_cost: u64 = ((end.0 - coord.0) + (end.1 - coord.1)).try_into().unwrap();
                    let next_risk: u64 = current_step.risk + u64::from(risk);
                    exploration_front.push(RiskStep{location: coord, a_star_risk: next_risk + distance_cost, risk: next_risk});
                }
            }
        }

        panic!("We shouldn't get here without reaching the end.");
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
    let start: Point = (0, 0);
    let end: Point = (risk_grid.rows - 1, risk_grid.columns - 1);
    println!("Lowest risk path sum: {}", risk_grid.find_lowest_risk_path(&start, &end));
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

        assert_eq!(risk_grid.find_lowest_risk_path(&start, &end), 40u64);
    }
}