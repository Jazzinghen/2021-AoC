use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::vec;

use itertools::Itertools;

use rayon::prelude::*;

struct ImageEnhancer {
    lut: [bool; 512],
}

impl ImageEnhancer {
    fn new(raw_data: &str) -> Self {
        Self {
            lut: raw_data
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn enhance_picture(&self, picture: &SensorImage) -> SensorImage {
        let enhanced_rows = picture.rows + 2;
        let enhanced_cols = picture.cols + 2;

        let padding_data = if picture.padding_value {
            self.lut.last().unwrap()
        } else {
            self.lut.first().unwrap()
        };

        let coords: Vec<(usize, usize)> = (0..enhanced_rows)
            .cartesian_product(0..enhanced_cols)
            .collect();

        let data: Vec<bool> = coords
            .par_iter()
            .map(|(y, x)| {
                let matrix_data = picture.get_matrix(*x, *y);
                self.compute_pixel_value(&matrix_data)
            })
            .collect();

        SensorImage::from_vec_dimensions(&data, enhanced_rows, enhanced_cols, *padding_data)
    }

    fn compute_pixel_value(&self, pixels: &[bool; 9]) -> bool {
        let index = pixels.iter().fold(0usize, |total, pixel| {
            (total << 1) + if *pixel { 1 } else { 0 }
        });

        self.lut[index]
    }
}

impl fmt::Display for ImageEnhancer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "     0             8              16              24              31"
        )?;
        writeln!(
            f,
            "     |             |               |               |               |"
        )?;
        for (line, bits) in self.lut.iter().chunks(32).into_iter().enumerate() {
            write!(f, "{:3}  ", line * 32)?;
            writeln!(f, "{}", bits.map(|b| if *b { '1' } else { '0' }).join(" "))?;
        }
        writeln!(f)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SensorImage {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
    kernel_ids: [i32; 9],
    padding_value: bool,
}

impl SensorImage {
    fn new(raw_data: &str) -> Self {
        let rows = raw_data.lines().count();
        let cols = raw_data.lines().next().unwrap().chars().count();

        let row_padding = vec![false; (rows + 4) * 2];
        let mut data: Vec<bool> = row_padding.clone();
        for data_line in raw_data.lines() {
            data.extend([false, false]);
            data.extend(data_line.chars().map(|c| c == '#'));
            data.extend([false, false]);
        }
        data.extend(row_padding.into_iter());

        Self {
            rows,
            cols,
            data,
            kernel_ids: Self::compute_offsets(i32::try_from(cols + 4).unwrap()),
            padding_value: false,
        }
    }

    fn from_vec_dimensions(
        raw_data: &[bool],
        rows: usize,
        cols: usize,
        padding_value: bool,
    ) -> Self {
        let row_padding = vec![padding_value; (rows + 4) * 2];
        let mut data: Vec<bool> = row_padding.clone();
        for row in 0..rows {
            data.extend([padding_value, padding_value]);
            data.extend(raw_data.iter().skip(row * cols).take(cols));
            data.extend([padding_value, padding_value]);
        }
        data.extend(row_padding.into_iter());

        Self {
            rows,
            cols,
            data,
            kernel_ids: Self::compute_offsets(i32::try_from(cols + 4).unwrap()),
            padding_value,
        }
    }

    fn compute_offsets(padding: i32) -> [i32; 9] {
        (-1..=1)
            .cartesian_product(-1..=1)
            .map(|(y, x)| y * padding + x)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap()
    }

    // This function gets a row-major pixel matrix centred on the given location
    // The location is based on the "extended" picture as the first usable pixel
    // of the picture influences a matrix centred in [-1, -1]
    pub fn get_matrix(&self, x: usize, y: usize) -> [bool; 9] {
        assert!(x <= self.cols + 1 && y <= self.rows + 1);

        let centre_id = i32::try_from((y + 1) * (self.cols + 4) + x + 1).unwrap();

        self.kernel_ids
            .iter()
            .map(|offset| self.data[usize::try_from(centre_id + *offset).unwrap()])
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
    }

    // If we have a "lit" padding, that means that we will have INFINITE lit pixels
    pub fn get_lit_pixels(&self) -> Option<usize> {
        if !self.padding_value {
            Some(self.data.iter().filter(|pixel| **pixel).count())
        } else {
            None
        }
    }
}

impl fmt::Display for SensorImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Image dimensions: [{}; {}]", self.rows, self.cols)?;
        writeln!(
            f,
            "Padding data: {}",
            if self.padding_value { '#' } else { '.' }
        )?;
        for pixels in self
            .data
            .iter()
            .chunks(self.cols + 4)
            .into_iter()
            .skip(2)
            .take(self.rows)
        {
            let row_data: String = pixels
                .skip(2)
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

pub fn part1(input: &str) {
    let (enhancer, picture) = parse_input(input);

    let first_pass = enhancer.enhance_picture(&picture);
    let second_pass = enhancer.enhance_picture(&first_pass);

    println!(
        "The enhanced picture has {:?} lit pixels",
        second_pass.get_lit_pixels()
    );
}

pub fn part2(input: &str) {
    let (enhancer, picture) = parse_input(input);

    let mut final_image = picture;
    for _ in 0..50 {
        final_image = enhancer.enhance_picture(&final_image);
    }

    println!(
        "The enhanced picture has {:?} lit pixels",
        final_image.get_lit_pixels()
    );
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
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, true, false, false, true,
                false, false, false, false, false, true, false, false, false, false, false, false,
                false, false, true, true, false, false, true, false, false, false, false, false,
                false, true, false, false, false, false, false, false, false, false, true, true,
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false,
            ],
            rows: 5,
            cols: 5,
            kernel_ids: [-10, -9, -8, -1, 0, 1, 8, 9, 10],
            padding_value: false,
        };

        assert_eq!(enhancer.lut.len(), 512);
        assert_eq!(picture, ref_picture);
    }

    #[test]
    fn one_enhancement_step() {
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

        let first_pass = enhancer.enhance_picture(&picture);

        let first_pass_ref = SensorImage {
            rows: 7,
            cols: 7,
            data: vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, true, true, false, true, true, false, false, false, false, false, true,
                false, false, true, false, true, false, false, false, false, false, true, true,
                false, true, false, false, true, false, false, false, false, true, true, true,
                true, false, false, true, false, false, false, false, false, true, false, false,
                true, true, false, false, false, false, false, false, false, true, true, false,
                false, true, false, false, false, false, false, false, false, true, false, true,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false,
            ],
            kernel_ids: [-12, -11, -10, -1, 0, 1, 10, 11, 12],
            padding_value: false,
        };

        assert_eq!(first_pass, first_pass_ref);
    }

    #[test]
    fn full_enhancement() {
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

        let first_pass = enhancer.enhance_picture(&picture);
        let second_pass = enhancer.enhance_picture(&first_pass);

        let second_pass_ref = SensorImage {
            rows: 9,
            cols: 9,
            data: vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, true,
                false, false, false, false, false, false, true, false, false, true, false, true,
                false, false, false, false, false, false, true, false, true, false, false, false,
                true, true, true, false, false, false, false, true, false, false, false, true,
                true, false, true, false, false, false, false, false, true, false, false, false,
                false, false, true, false, true, false, false, false, false, false, true, false,
                true, true, true, true, true, false, false, false, false, false, false, false,
                true, false, true, true, true, true, true, false, false, false, false, false,
                false, false, true, true, false, true, true, false, false, false, false, false,
                false, false, false, false, true, true, true, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false,
            ],
            kernel_ids: [-14, -13, -12, -1, 0, 1, 12, 13, 14],
            padding_value: false,
        };

        assert_eq!(second_pass, second_pass_ref);
        assert_eq!(second_pass.get_lit_pixels(), Some(35));
    }

    #[test]
    fn totally_bonkers_enhancement() {
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

        let mut final_image = picture;

        for _ in 0..50 {
            final_image = enhancer.enhance_picture(&final_image);
        }

        assert_eq!(final_image.get_lit_pixels(), Some(3351));
    }

    #[test]
    fn full_enhancement_flipping_pad() {
        let input_string =
            "#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
            #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
            .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
            .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
            .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
            ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
            ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#...

            #..#.
            #....
            ##..#
            ..#..
            ..###";

        let (enhancer, picture) = parse_input(input_string);

        let first_pass = enhancer.enhance_picture(&picture);
        let second_pass = enhancer.enhance_picture(&first_pass);

        let second_pass_ref = SensorImage {
            rows: 9,
            cols: 9,
            data: vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, true, false,
                false, false, false, false, false, false, true, true, true, false, false, false,
                false, false, false, false, false, false, false, false, true, false, false, true,
                true, true, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false, false, false, false, false, false, false, true, false,
                false, true, true, true, false, true, false, false, false, false, true, false,
                false, true, true, true, true, true, false, false, false, false, false, false,
                false, false, true, false, false, false, false, true, false, false, false, false,
                false, false, true, false, false, false, false, false, true, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false,
            ],
            kernel_ids: [-14, -13, -12, -1, 0, 1, 12, 13, 14],
            padding_value: false,
        };

        assert_eq!(second_pass, second_pass_ref);
        assert_eq!(second_pass.get_lit_pixels(), Some(24));
    }

    #[test]
    fn full_enhancement_one_pad_change() {
        let input_string =
            "#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
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

        let first_pass = enhancer.enhance_picture(&picture);
        let second_pass = enhancer.enhance_picture(&first_pass);

        let second_pass_ref = SensorImage {
            rows: 9,
            cols: 9,
            data: vec![
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                false, false, false, false, false, false, true, true, true, true, true, true, true,
                false, false, false, false, false, true, false, true, true, true, true, true, true,
                true, true, true, false, false, false, false, false, true, true, true, true, true,
                true, true, false, false, true, true, true, false, true, true, true, true, true,
                false, false, false, false, false, false, true, false, false, true, true, true,
                true, false, true, false, false, true, true, true, false, true, true, true, true,
                true, true, false, false, true, true, true, true, true, false, true, true, true,
                true, true, true, false, true, false, false, false, false, true, true, true, true,
                true, true, true, true, false, false, false, false, false, true, true, true, true,
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true, true, true, true, true, true, true, true, true,
            ],
            kernel_ids: [-14, -13, -12, -1, 0, 1, 12, 13, 14],
            padding_value: true,
        };

        assert_eq!(second_pass, second_pass_ref);
        assert!(second_pass.get_lit_pixels().is_none());
    }
}
