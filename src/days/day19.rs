use std::vec;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, i64, newline, space0};
use nom::combinator::opt;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use nalgebra::Point3;

#[derive(Debug, PartialEq, Eq)]
struct SensorData {
    beacons: Vec<Point3<i64>>,
}

impl SensorData {
    pub fn new(raw_data: &[(i64, i64, i64)]) -> Self {
        SensorData {
            beacons: raw_data
                .iter()
                .map(|(x, y, z)| Point3::new(*x, *y, *z))
                .collect(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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
                    Point3::from([4, 1, 0]),
                    Point3::from([3, 3, 0]),
                ],
            },
            SensorData {
                beacons: vec![
                    Point3::from([-1, -1, 0]),
                    Point3::from([-5, 0, 0]),
                    Point3::from([-2, 1, 0]),
                ],
            },
        ];

        assert_eq!(sensors, ref_sensors);
    }
}
