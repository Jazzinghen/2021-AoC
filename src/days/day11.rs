use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;

use itertools::Itertools;

type Point = (usize, usize);

struct OctopusGrid {
    data: Vec<u8>,
    rows: usize,
    columns: usize,
    energy_to_location: [HashSet<Point>; 11],
}

impl OctopusGrid {
    pub fn new(input: &str) -> OctopusGrid {
        let grid_rows = input.split_whitespace();
        let mut flat_data = Vec::new();
        let mut energy_to_location: [HashSet<Point>; 11] = [(); 11].map(|_| Default::default());

        let mut row_count = 0usize;
        let mut col_count = 0usize;
        for (row, row_string) in grid_rows.enumerate() {
            row_count += 1;
            col_count = row_string.len();
            for (col, risk) in row_string.chars().enumerate() {
                let risk_digit: u8 = risk.to_digit(10).unwrap().try_into().unwrap();
                energy_to_location[usize::from(risk_digit)].insert((row, col));
                flat_data.push(risk_digit);
            }
        }

        OctopusGrid {
            data: flat_data,
            rows: row_count,
            columns: col_count,
            energy_to_location,
        }
    }

    fn compute_flat_idx(&self, location: &Point) -> usize {
        location.0 * self.columns + location.1
    }

    pub fn get_energy(&self, location: &Point) -> u8 {
        let flat_idx = self.compute_flat_idx(location);
        *self
            .data
            .get(flat_idx)
            .expect("Provided location is out of the grid bounds!")
    }

    fn increase_energy(&mut self, location: &Point) {
        let flat_idx = self.compute_flat_idx(location);
        *self
            .data
            .get_mut(flat_idx)
            .expect("Provided location is out of the grid bounds!") += 1u8;
    }

    fn reset_energy(&mut self, location: &Point) {
        let flat_idx = self.compute_flat_idx(location);
        *self
            .data
            .get_mut(flat_idx)
            .expect("Provided location is out of the grid bounds!") = 0u8;
    }

    fn get_neighbours(&self, location: &Point) -> HashSet<Point> {
        let mut neighbours: HashSet<Point> = HashSet::new();

        let up = if location.0 > 0 {
            location.0 - 1
        } else {
            location.0
        };

        let down = if location.0 < self.rows - 1 {
            location.0 + 1
        } else {
            location.0
        };

        let left = if location.1 > 0 {
            location.1 - 1
        } else {
            location.1
        };

        let right = if location.1 < self.columns - 1 {
            location.1 + 1
        } else {
            location.1
        };

        let neigh_cross_product = (up..=down).cartesian_product(left..=right);

        neighbours.extend(neigh_cross_product);
        neighbours.remove(location);

        neighbours
    }

    fn step(&mut self) -> u64 {
        let mut flashing_octopi: HashSet<Point> = HashSet::new();

        self.data.iter_mut().for_each(|octo| *octo += 1);

        // Increase energies by one
        for energy in (0..=10).rev().skip(1) {
            self.energy_to_location[energy + 1]
                .extend(self.energy_to_location[energy].clone().iter());
            self.energy_to_location[energy].clear();
        }

        while !self.energy_to_location[10].is_empty() {
            for flashing_octo in &self.energy_to_location[10].clone() {
                let close_octos = self.get_neighbours(flashing_octo);
                for neighbour in close_octos {
                    let nei_energy: usize = self.get_energy(&neighbour).into();
                    if nei_energy < 10 {
                        self.increase_energy(&neighbour);
                        self.energy_to_location[nei_energy + 1].insert(neighbour);
                        self.energy_to_location[nei_energy].remove(&neighbour);
                    }
                }
                flashing_octopi.insert(*flashing_octo);
                self.energy_to_location[10].remove(flashing_octo);
            }
        }

        for spent_octopus in flashing_octopi.iter() {
            self.reset_energy(spent_octopus);
        }

        flashing_octopi.len().try_into().unwrap()
    }

    pub fn step_for(&mut self, t: usize) -> u64 {
        (0..t).fold(0u64, |sum, _| sum + self.step())
    }

    pub fn first_coordinated_flash(&mut self) -> u64 {
        let mut step = 1u64;
        let area: u64 = (self.rows * self.columns).try_into().unwrap();
        while self.step() != area {
            step += 1;
        }

        step
    }
}

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid dimensions: {} x {}", self.rows, self.columns)?;

        for row in 0..self.rows {
            for col in 0..self.columns {
                let risk_location = (row, col);
                write!(f, "{} ", self.get_energy(&risk_location))?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Reverse energies lookup: ")?;
        for (energy, locations) in self.energy_to_location.iter().enumerate() {
            write!(f, "{}: ", energy)?;
            for loc in locations {
                write!(f, "{}, {}; ", loc.0, loc.1)?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

pub fn part1(input: &str) {
    let mut octo_grid = OctopusGrid::new(input);
    let final_flashes = octo_grid.step_for(100);
    println!("Flashes after 100 steps: {}", final_flashes);
}

pub fn part2(input: &str) {
    let mut octo_grid = OctopusGrid::new(input);
    let synchronization_flash = octo_grid.first_coordinated_flash();
    println!("First synchronized step: {}", synchronization_flash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_steps() {
        let input_string = "11111
                            19991
                            19191
                            19991
                            11111";

        let mut octo_grid = OctopusGrid::new(input_string);

        assert_eq!(octo_grid.step_for(2), 9u64);
    }

    #[test]
    fn base_flash_computation() {
        let input_string = "5483143223
                            2745854711
                            5264556173
                            6141336146
                            6357385478
                            4167524645
                            2176841721
                            6882881134
                            4846848554
                            5283751526";

        let mut octo_grid = OctopusGrid::new(input_string);

        let ten_steps = octo_grid.step_for(10);

        // After 10 turns
        assert_eq!(ten_steps, 204u64);

        // After 100 turns
        assert_eq!(ten_steps + octo_grid.step_for(90), 1656u64);
    }

    #[test]
    fn first_synchronized_flash() {
        let input_string = "5483143223
                            2745854711
                            5264556173
                            6141336146
                            6357385478
                            4167524645
                            2176841721
                            6882881134
                            4846848554
                            5283751526";

        let mut octo_grid = OctopusGrid::new(input_string);

        assert_eq!(octo_grid.first_coordinated_flash(), 195u64);
    }
}
