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

    pub fn batch_enhance(&self, picture: &SensorImage, steps: usize) -> SensorImage {
        let enhanced_rows = picture.rows + (steps + 1) * 2;
        let enhanced_cols = picture.cols + (steps + 1) * 2;

        let mut data_flip: Vec<bool> = vec![picture.padding_value; enhanced_rows * enhanced_cols];

        for (row, chunk) in picture
            .data
            .iter()
            .chunks(picture.cols + 4)
            .into_iter()
            .enumerate()
        {
            let target_row_offset = (steps + row - 1) * enhanced_cols + steps - 1;

            for (col, bit) in chunk.enumerate() {
                data_flip[target_row_offset + col] = *bit;
            }
        }

        let mut data_flop: Vec<bool> = data_flip.clone();

        let offset_matrix = SensorImage::compute_offsets(i32::try_from(enhanced_cols).unwrap());
        let coords: Vec<(usize, usize)> = (0..enhanced_rows)
            .cartesian_product(0..enhanced_cols)
            .collect();

        for iteration in 0..steps {
            let (input, output) = if iteration % 2 == 0 {
                (&data_flip, &mut data_flop)
            } else {
                (&data_flop, &mut data_flip)
            };

            *output = coords
                .par_iter()
                .map(|(y, x)| {
                    let centre_id = (y * enhanced_cols) + x;
                    if *y == 0 || *y >= enhanced_rows - 1 || *x == 0 || *x >= enhanced_cols - 1 {
                        if input[centre_id] {
                            *self.lut.last().unwrap()
                        } else {
                            *self.lut.first().unwrap()
                        }
                    } else {
                        let matrix_data = ImageEnhancer::get_matrix(
                            input,
                            i32::try_from(centre_id).unwrap(),
                            &offset_matrix,
                        );
                        self.compute_pixel_value(&matrix_data)
                    }
                })
                .collect();
        }

        let data = if (steps - 1) % 2 == 0 {
            &data_flop
        } else {
            &data_flip
        };

        let cropped_output: Vec<bool> = (1..enhanced_rows - 1)
            .cartesian_product(1..enhanced_cols - 1)
            .into_iter()
            .map(|(y, x)| (y * enhanced_cols) + x)
            .map(|id| data[id])
            .collect();

        SensorImage::from_vec_dimensions(
            &cropped_output,
            enhanced_rows - 2,
            enhanced_cols - 2,
            data[0],
        )
    }

    fn compute_pixel_value(&self, pixels: &[bool; 9]) -> bool {
        let index = pixels
            .iter()
            .fold(0usize, |total, pixel| (total << 1) + (*pixel as usize));

        self.lut[index]
    }

    fn get_matrix(data: &[bool], centre_id: i32, offsets: &[i32; 9]) -> [bool; 9] {
        offsets
            .iter()
            .map(|offset| data[usize::try_from(centre_id + *offset).unwrap()])
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
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

    /*
    let mut final_image = picture;
    for _ in 0..50 {
        final_image = enhancer.enhance_picture(&final_image);
    }
    */

    let final_image = enhancer.batch_enhance(&picture, 50);

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
    fn one_enhancement_step_batch() {
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

        let first_pass = enhancer.batch_enhance(&picture, 1);

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
    fn full_enhancement_batch() {
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

        let second_pass = enhancer.batch_enhance(&picture, 2);

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
    fn totally_bonkers_batch() {
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

        let final_image = enhancer.batch_enhance(&picture, 50);

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

    #[test]
    fn full_enhancement_flipping_batch() {
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

        let second_pass = enhancer.batch_enhance(&picture, 2);

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
    fn full_enhancement_one_change_batch() {
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

        let second_pass = enhancer.batch_enhance(&picture, 2);

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
