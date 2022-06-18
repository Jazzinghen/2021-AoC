use std::cmp::Ordering;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::env::temp_dir;
use std::fs::File;
use std::io::{Error, Write};

use hashbrown::HashSet;
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

fn beacon_data(input: &str) -> IResult<&str, (i64, i64, i64)> {
    delimited(
        space0,
        tuple((terminated(i64, tag(",")), terminated(i64, tag(",")), i64)),
        opt(newline),
    )(input)
}

fn sensor_data(input: &str) -> IResult<&str, SensorData> {
    let (rem_str, beacons) = many1(beacon_data)(input)?;

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

fn merge_sensors(
    dst: Vec<Point3<i64>>,
    beacons: &[Point3<i64>],
    origin_translation: &Vector3<i64>,
) -> Vec<Point3<i64>> {
    let mut beacons_set: HashSet<Point3<i64>> = dst.into_iter().collect();

    beacons_set.extend(beacons.iter().map(|b| b + origin_translation));
    beacons_set.into_iter().sorted_by(compare_points).collect()
}

fn reconstruct_beacon_map(
    base_data: &[Point3<i64>],
    sensors: &[SensorData],
) -> (Vec<Point3<i64>>, Vec<Point3<i64>>) {
    let mut full_map: Vec<Point3<i64>> = base_data.to_vec();
    let mut sensors_to_parse: HashSet<usize> = (0..sensors.len()).collect();
    let mut origins: Vec<Point3<i64>> = vec![Point3::new(0, 0, 0)];

    while !sensors_to_parse.is_empty() {
        let remaining_sensors: Vec<_> = sensors
            .iter()
            .enumerate()
            .filter(|(idx, _)| sensors_to_parse.contains(idx))
            .collect();
        let remaining_idx: Vec<_> = remaining_sensors.iter().map(|(id, _)| id).collect();
        println!("Remaining sensors to parse: {:?}", remaining_idx);
        for (sensor_idx, sensor_data) in remaining_sensors.into_iter() {
            if let Some((rot, translation)) = sensor_data.find_overlap(&full_map, 12) {
                full_map = merge_sensors(
                    full_map,
                    &sensor_data.rotated_data[usize::from(rot)],
                    &translation,
                );

                origins.push(Point3::from(translation));

                sensors_to_parse.remove(&sensor_idx);
            }
        }
    }

    (full_map, origins)
}

fn find_farthest_pair(origins: &[Point3<i64>]) -> (usize, (Point3<i64>, Point3<i64>)) {
    let mut farthest: (usize, usize) = (0, 0);
    let mut max_distance: usize = 0;

    for (left_idx, left_origin) in origins.iter().enumerate() {
        for (right_idx, right_origin) in origins.iter().enumerate().skip(left_idx + 1) {
            let distance_vector = right_origin - left_origin;
            let manhattan_distance: usize = distance_vector
                .abs()
                .iter()
                .map(|dist| usize::try_from(*dist).unwrap())
                .sum();

            if manhattan_distance > max_distance {
                farthest.0 = left_idx;
                farthest.1 = right_idx;
                max_distance = manhattan_distance;
            }
        }
    }

    (max_distance, (origins[farthest.0], origins[farthest.1]))
}

pub fn both_parts(input: &str) {
    let (_, mut sensors) = full_data(input).unwrap();

    for sensor in sensors.iter_mut().skip(1) {
        sensor.compute_rotations();
    }

    let (beacon_volume, origins) = reconstruct_beacon_map(&sensors[0].beacons, &sensors[1..]);
    let (manhattan_distance, _) = find_farthest_pair(&origins);

    println!("Total beacons count: {}", beacon_volume.len());
    println!("Distance between farthest beacons: {}", manhattan_distance);
}

pub fn _part2(_input: &str) {}

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

    #[test]
    fn beacon_volume_reconstruction() {
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
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
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

        for sensor in sensors.iter_mut().skip(1) {
            sensor.compute_rotations();
        }

        let (beacon_volume, _) = reconstruct_beacon_map(&sensors[0].beacons, &sensors[1..]);

        let ref_volume_str = "-892,524,684
        -876,649,763
        -838,591,734
        -789,900,-551
        -739,-1745,668
        -706,-3180,-659
        -697,-3072,-689
        -689,845,-530
        -687,-1600,576
        -661,-816,-575
        -654,-3158,-753
        -635,-1737,486
        -631,-672,1502
        -624,-1620,1868
        -620,-3212,371
        -618,-824,-621
        -612,-1695,1788
        -601,-1648,-643
        -584,868,-557
        -537,-823,-458
        -532,-1715,1894
        -518,-1681,-600
        -499,-1607,-770
        -485,-357,347
        -470,-3283,303
        -456,-621,1527
        -447,-329,318
        -430,-3130,366
        -413,-627,1469
        -345,-311,381
        -36,-1284,1171
        -27,-1108,-65
        7,-33,-71
        12,-2351,-103
        26,-1119,1091
        346,-2985,342
        366,-3059,397
        377,-2827,367
        390,-675,-793
        396,-1931,-563
        404,-588,-901
        408,-1815,803
        423,-701,434
        432,-2009,850
        443,580,662
        455,729,728
        456,-540,1869
        459,-707,401
        465,-695,1988
        474,580,667
        496,-1584,1900
        497,-1838,-617
        527,-524,1933
        528,-643,409
        534,-1912,768
        544,-627,-890
        553,345,-567
        564,392,-477
        568,-2007,-577
        605,-1665,1952
        612,-1593,1893
        630,319,-379
        686,-3108,-505
        776,-3184,-501
        846,-3110,-434
        1135,-1161,1235
        1243,-1093,1063
        1660,-552,429
        1693,-557,386
        1735,-437,1738
        1749,-1800,1813
        1772,-405,1572
        1776,-675,371
        1779,-442,1789
        1780,-1548,337
        1786,-1538,337
        1847,-1591,415
        1889,-1729,1762
        1994,-1805,1792";

        let ref_locations: Vec<_> = ref_volume_str
            .lines()
            .map(beacon_data)
            .filter_map(|pos_data| {
                pos_data
                    .map(|(_, data)| Point3::new(data.0, data.1, data.2))
                    .ok()
            })
            .sorted_by(compare_points)
            .collect();

        assert_eq!(beacon_volume.len(), ref_locations.len());
        assert_eq!(beacon_volume, ref_locations);
    }

    #[test]
    fn largest_beacon_distance() {
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
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
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

        for sensor in sensors.iter_mut().skip(1) {
            sensor.compute_rotations();
        }

        let (_, sensor_origins) = reconstruct_beacon_map(&sensors[0].beacons, &sensors[1..]);

        let (distance, (left_origin, right_origin)) = find_farthest_pair(&sensor_origins);

        let ref_origins: (Point3<i64>, Point3<i64>) =
            (Point3::new(1105, -1205, 1229), Point3::new(-92, -2380, -20));

        let origins_overlap = if left_origin == ref_origins.0 {
            right_origin == ref_origins.1
        } else if right_origin == ref_origins.0 {
            left_origin == ref_origins.1
        } else {
            false
        };

        assert!(origins_overlap);
        assert_eq!(distance, 3621);
    }
}
