use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::vec;

use itertools::Itertools;

use rayon::prelude::*;

struct ImageEnhancer {
    lut: Vec<bool>,
}

impl ImageEnhancer {
    fn new(raw_data: &str) -> Self {
        Self {
            lut: raw_data.chars().map(|c| c == '#').collect(),
        }
    }

    pub fn enhance_picture(&self, picture: &SensorImage) -> SensorImage {
        let enhanced_rows = picture.rows + 2;
        let enhanced_cols = picture.cols + 2;

        SensorImage {
            rows: enhanced_rows,
            cols: enhanced_cols,
            data: vec![],
            kernel_ids: [0; 9],
        }
    }

    fn compute_pixel_value(&self, pixels: &[bool]) -> bool {
        let index = pixels.iter().fold(0usize, |total, pixel| {
            (total << 2) + if *pixel { 1 } else { 0 }
        });
        self.lut[index]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SensorImage {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
    kernel_ids: [i32; 9],
}

impl SensorImage {
    fn new(raw_data: &str) -> Self {
        let rows = raw_data.lines().count();
        let cols = raw_data.lines().next().unwrap().chars().count();

        let row_padding = vec![false; rows + 4];
        let mut data: Vec<bool> = row_padding.clone();
        for data_line in raw_data.lines() {
            data.extend([false, false]);
            data.extend(data_line.chars().map(|c| c == '#'));
            data.extend([false, false]);
        }
        data.extend(row_padding.into_iter());

        let mut kernel_ids = [0; 9];
        let padding = i32::try_from(cols).unwrap() + 4;
        for (id, (y, x)) in (-1..=1).cartesian_product(-1..=1).enumerate() {
            kernel_ids[id] = y * padding + x;
        }

        Self {
            rows,
            cols,
            data,
            kernel_ids,
        }
    }

    pub fn get_pixel_id(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.rows && y < self.cols {
            Some((y + 2) * self.rows + x + 2)
        } else {
            None
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        self.get_pixel_id(x, y).map(|idx| self.data[idx])
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        if let Some(idx) = self.get_pixel_id(x, y) {
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn get_matrix(&self, x: usize, y: usize) -> Vec<&bool> {
        assert!(x < self.cols && y < self.rows);

        vec![]
    }
}

impl fmt::Display for SensorImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Image dimensions: [{}; {}]", self.rows, self.cols)?;
        for row in 0..self.rows {
            let row_data: String = self
                .data
                .iter()
                .skip((row + 1) * self.cols + 1)
                .take(self.cols)
                .map(|p| if *p { '#' } else { '.' })
                .collect();
            writeln!(f, "{}", row_data)?;
        }
        writeln!(f)
    }
}

fn parse_input(input: &str) -> (ImageEnhancer, SensorImage) {
    let mut lines = input.lines();

    let enhance_string: String = lines.next().unwrap().split_whitespace().collect();

    let image_string: String = lines
        .skip(1)
        .map(|l| format!("{}\n", l.trim_start()))
        .collect();

    (
        ImageEnhancer::new(&enhance_string),
        SensorImage::new(&image_string),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        let input_string =
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
            #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
            .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
            .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
            .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
            ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
            ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

            #..#.
            #....
            ##..#
            ..#..
            ..###";

        let (enhancer, picture) = parse_input(input_string);

        let ref_picture = SensorImage {
            data: vec![
                false, false, false, false, false, false, false, false, false, false, false, true,
                false, false, true, false, false, false, false, false, true, false, false, false,
                false, false, false, false, false, true, true, false, false, true, false, false,
                false, false, false, false, true, false, false, false, false, false, false, false,
                false, true, true, true, false, false, false, false, false, false, false, false,
                false, false, false,
            ],
            rows: 5,
            cols: 5,
            kernel_ids: [-10, -9, -8, -1, 0, 1, 8, 9, 10],
        };

        assert_eq!(enhancer.lut.len(), 512);
        assert_eq!(picture, ref_picture);
    }
}
