use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::env::temp_dir;
use std::fs::File;
use std::io::{Error, Write};

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, i64, newline, space0};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

use nalgebra::{Matrix3, Point3, Vector3};

const ROTATION_MATRICES: [[[i64; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
];

fn compare_points(left: &Point3<i64>, right: &Point3<i64>) -> Ordering {
    let x_cmp = left.x.cmp(&right.x);
    if x_cmp != Ordering::Equal {
        return x_cmp;
    }

    let y_cmp = left.y.cmp(&right.y);
    if y_cmp != Ordering::Equal {
        return y_cmp;
    }

    left.z.cmp(&right.z)
}

fn _write_beacons(filename: &str, rotated_data: &[Vec<Point3<i64>>]) -> Result<(), Error> {
    let mut file_path = temp_dir();
    file_path.push(filename);

    let mut output = File::create(file_path)?;

    for translation_block in rotated_data.iter() {
        writeln!(output, "==== Shifted block start ====")?;
        writeln!(output)?;

        for beacon_location in translation_block {
            writeln!(
                output,
                "{}, {}, {}",
                beacon_location.x, beacon_location.y, beacon_location.z
            )?;
        }

        writeln!(output)?;
        writeln!(output, "==== Shifted block end ====")?;
        writeln!(output)?;
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct SensorData {
    beacons: Vec<Point3<i64>>,
    rotated_data: [Vec<Point3<i64>>; 24],
}

impl SensorData {
    pub fn new(raw_data: &[(i64, i64, i64)]) -> Self {
        SensorData {
            beacons: raw_data
                .iter()
                .map(|(x, y, z)| Point3::new(*x, *y, *z))
                .sorted_by(compare_points)
                .collect(),
            rotated_data: Default::default(),
        }
    }

    pub fn compute_rotations(&mut self) {
        for (rot_idx, rotation_matrix) in ROTATION_MATRICES
            .iter()
            .map(|mat| Matrix3::from_iterator(mat.iter().flatten().cloned()))
            .enumerate()
        {
            self.rotated_data[rot_idx] = self
                .beacons
                .iter()
                .map(|beacon| rotation_matrix * *beacon)
                .sorted_by(compare_points)
                .collect();
        }
    }

    pub fn find_overlap(
        &self,
        beacons: &[Point3<i64>],
        overlaps_needed: usize,
    ) -> Option<(u8, Vector3<i64>)> {
        let mut max_overlaps: usize = 0;
        let mut rotation: u8 = 0;
        let mut translation: Vector3<i64> = Vector3::zeros();

        // There have to be at least 12 shared beacons to have two overlapped sensor spaces
        let last_start_beacon = beacons.len() - overlaps_needed;
        for (rot_idx, rotated_beacons) in self.rotated_data.iter().enumerate() {
            for (shift_idx, shift_target) in beacons.iter().take(last_start_beacon).enumerate() {
                for (origin_id, curr_origin) in
                    rotated_beacons.iter().take(last_start_beacon).enumerate()
                {
                    let mut current_overlaps: usize = 1;
                    let block_translation: Vector3<i64> = shift_target - curr_origin;

                    // Sadly I cannot use iterators due to the double condition of the while loop :(
                    let mut ref_idx = shift_idx + 1;
                    let mut curr_idx: usize = origin_id + 1;

                    while ref_idx < beacons.len() && curr_idx < rotated_beacons.len() {
                        let ref_beacon = beacons.get(ref_idx).unwrap();
                        let translated_beacon =
                            rotated_beacons.get(curr_idx).unwrap() + block_translation;

                        match compare_points(&translated_beacon, ref_beacon) {
                            Ordering::Greater => ref_idx += 1,
                            Ordering::Less => {
                                curr_idx += 1;
                            }
                            Ordering::Equal => {
                                ref_idx += 1;
                                curr_idx += 1;
                                current_overlaps += 1;
                            }
                        }
                    }

                    if current_overlaps > max_overlaps {
                        max_overlaps = current_overlaps;
                        rotation = rot_idx.try_into().unwrap(); // This in always < 24 given it's a static-length array
                        translation = block_translation;
                    }
                }
            }
        }

        if max_overlaps >= overlaps_needed {
            Some((rotation, translation))
        } else {
            None
        }
    }
}

type RotationIDX = (usize, u8);

struct AlignmentData {
    origin: RotationIDX,
    target: RotationIDX,
    translation: Vector3<i64>,
}

fn sensor_data(input: &str) -> IResult<&str, SensorData> {
    let (rem_str, beacons) = many1(delimited(
        space0,
        tuple((terminated(i64, tag(",")), terminated(i64, tag(",")), i64)),
        opt(newline),
    ))(input)?;

    Ok((rem_str, SensorData::new(&beacons)))
}

fn full_data(input: &str) -> IResult<&str, Vec<SensorData>> {
    many1(delimited(
        delimited(
            space0,
            delimited(tag("--- scanner "), digit1, tag(" ---")),
            newline,
        ),
        sensor_data,
        opt(newline),
    ))(input)
}

fn find_overlap_pairs(sensors: &[SensorData]) -> Vec<AlignmentData> {
    let mut alignment_exploration: VecDeque<(RotationIDX, RotationIDX)> = VecDeque::new();
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Utility function to generate all the rotation matrices I used to create the const data
    fn _generate_rot_matrices() -> Vec<Matrix3<i8>> {
        let roll_mat: Matrix3<i8> = Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0);
        let twist_cw_mat: Matrix3<i8> = Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0);
        let twist_ccw_mat: Matrix3<i8> = Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0);

        let mut curr_matrix: Matrix3<i8> = Matrix3::identity();
        let mut rot_matrices: Vec<Matrix3<i8>> = Vec::new();

        for _ in 0..3 {
            let roll_columns: Vec<Vector3<i8>> =
                curr_matrix.column_iter().map(|c| roll_mat * c).collect();
            curr_matrix = Matrix3::from_columns(&roll_columns);
            rot_matrices.push(curr_matrix);

            for _ in 0..3 {
                let twist_columns: Vec<Vector3<i8>> = curr_matrix
                    .column_iter()
                    .map(|c| twist_cw_mat * c)
                    .collect();
                curr_matrix = Matrix3::from_columns(&twist_columns);
                rot_matrices.push(curr_matrix);
            }

            let roll_columns: Vec<Vector3<i8>> =
                curr_matrix.column_iter().map(|c| roll_mat * c).collect();
            curr_matrix = Matrix3::from_columns(&roll_columns);
            rot_matrices.push(curr_matrix);

            for _ in 0..3 {
                let twist_columns: Vec<Vector3<i8>> = curr_matrix
                    .column_iter()
                    .map(|c| twist_ccw_mat * c)
                    .collect();
                curr_matrix = Matrix3::from_columns(&twist_columns);
                rot_matrices.push(curr_matrix);
            }
        }

        rot_matrices
    }

    #[test]
    fn input_parsing() {
        let input_string = "--- scanner 0 ---
        0,2,0
        4,1,0
        3,3,0

        --- scanner 1 ---
        -1,-1,0
        -5,0,0
        -2,1,0";

        let (_, sensors) = full_data(input_string).unwrap();

        let ref_sensors: Vec<SensorData> = vec![
            SensorData {
                beacons: vec![
                    Point3::from([0, 2, 0]),
                    Point3::from([3, 3, 0]),
                    Point3::from([4, 1, 0]),
                ],
                rotated_data: Default::default(),
            },
            SensorData {
                beacons: vec![
                    Point3::from([-5, 0, 0]),
                    Point3::from([-2, 1, 0]),
                    Point3::from([-1, -1, 0]),
                ],
                rotated_data: Default::default(),
            },
        ];

        assert_eq!(sensors, ref_sensors);
    }

    #[test]
    fn simple_overlap_check() {
        let input_string = "--- scanner 0 ---
        0,2,0
        4,1,0
        3,3,0

        --- scanner 1 ---
        2,0,-2
        1,0,-6
        0,0,-3
        -1,0,-5";

        let (_, mut sensors) = full_data(input_string).unwrap();

        sensors[1].compute_rotations();

        assert_eq!(
            sensors[1].find_overlap(&sensors[0].beacons, 2),
            Some((18, Vector3::<i64>::new(6, 3, 0)))
        );
    }

    #[test]
    fn sensor_pair_overlap() {
        let input_string = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390";

        let (_, mut sensors) = full_data(input_string).unwrap();

        sensors[1].compute_rotations();

        assert_eq!(
            sensors[1].find_overlap(&sensors[0].beacons, 12),
            Some((22, Vector3::<i64>::new(68, -1246, -43)))
        );
    }

    #[test]
    fn sensor_pair_overlap_fail() {
        let input_string = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14";

        let (_, mut sensors) = full_data(input_string).unwrap();

        sensors[1].compute_rotations();

        assert_eq!(sensors[1].find_overlap(&sensors[0].beacons, 12), None);
    }
}
