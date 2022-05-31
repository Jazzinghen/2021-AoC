use itertools::Itertools;
use nalgebra::Point3;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

enum CutAxis {
    X(i32),
    Y(i32),
    Z(i32),
}

fn power(input: &str) -> IResult<&str, bool> {
    let (rem_str, power) = alt((tag("on"), tag("off")))(input)?;

    Ok((rem_str, power == "on"))
}

fn axis_range(input: &str) -> IResult<&str, (i32, i32)> {
    let (rem_str, (first_raw, second_raw)) = preceded(
        alt((tag("x="), tag("y="), tag("z="))),
        separated_pair(
            pair(opt(tag("-")), digit1),
            tag(".."),
            pair(opt(tag("-")), digit1),
        ),
    )(input)?;

    let first_value = format!("{}{}", first_raw.0.unwrap_or(""), first_raw.1);
    let second_value = format!("{}{}", second_raw.0.unwrap_or(""), second_raw.1);

    Ok((
        rem_str,
        (first_value.parse().unwrap(), second_value.parse().unwrap()),
    ))
}

fn power_cube(input: &str) -> IResult<&str, PowerCuboid> {
    let (rem_str, power_state) = delimited(space0, power, space0)(input)?;

    let (rem_str, axes) = separated_list1(tag(","), axis_range)(rem_str)?;

    let x_range = (axes[0].0.min(axes[0].1), axes[0].0.max(axes[0].1));
    let y_range = (axes[1].0.min(axes[1].1), axes[1].0.max(axes[1].1));
    let z_range = (axes[2].0.min(axes[2].1), axes[2].0.max(axes[2].1));

    Ok((
        rem_str,
        PowerCuboid::new(power_state, x_range, y_range, z_range),
    ))
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PowerCuboid {
    top_right: Point3<i32>,
    bottom_left: Point3<i32>,
    power_state: bool,
}

impl PowerCuboid {
    fn new(
        power_state: bool,
        x_range: (i32, i32),
        y_range: (i32, i32),
        z_range: (i32, i32),
    ) -> Self {
        let bottom_left: Point3<i32> = Point3::new(x_range.0, y_range.0, z_range.0);
        let top_right: Point3<i32> = Point3::new(x_range.1, y_range.1, z_range.1);

        Self {
            top_right,
            bottom_left,
            power_state,
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Vec<Self>> {
        if self.bottom_left > other.top_right || self.top_right < other.bottom_left {
            return None;
        };

        let (min_x, max_x) = (
            self.bottom_left.x.max(other.bottom_left.x),
            self.top_right.x.min(other.top_right.x),
        );
        let (min_y, max_y) = (
            self.bottom_left.y.max(other.bottom_left.y),
            self.top_right.y.min(other.top_right.y),
        );
        let (min_z, max_z) = (
            self.bottom_left.z.max(other.bottom_left.z),
            self.top_right.z.min(other.top_right.z),
        );

        let cutting_planes = vec![
            (CutAxis::X(min_x), CutAxis::X(max_x + 1)),
            (CutAxis::Y(min_y), CutAxis::Y(max_y + 1)),
            (CutAxis::Z(min_z), CutAxis::Z(max_z + 1)),
        ];

        // Add intersection to the result
        let mut final_cubes = vec![Self {
            top_right: Point3::new(max_x, max_y, max_z),
            bottom_left: Point3::new(min_x, min_y, min_z),
            power_state: other.power_state,
        }];

        // Compute all the resulting cuts from removing the intersection from us and the other cube
        let (our_intersection, our_cuts) = self.extract_intersection(&cutting_planes);
        let (other_intersection, other_cuts) = other.extract_intersection(&cutting_planes);

        // The intersection from both extractions
        assert!(
            our_intersection.top_right == final_cubes[0].top_right
                && our_intersection.bottom_left == final_cubes[0].bottom_left
        );
        assert!(
            other_intersection.top_right == final_cubes[0].top_right
                && other_intersection.bottom_left == final_cubes[0].bottom_left
        );

        final_cubes.extend(our_cuts.into_iter());
        final_cubes.extend(other_cuts.into_iter());

        Some(final_cubes)
    }

    fn cut_along(&self, cut: &CutAxis) -> Option<(Self, Self)> {
        match *cut {
            CutAxis::X(coord) => {
                if coord <= self.bottom_left.x || coord > self.top_right.x {
                    None
                } else {
                    let new_left = Self {
                        power_state: self.power_state,
                        bottom_left: self.bottom_left,
                        top_right: Point3::new(coord - 1, self.top_right.y, self.top_right.z),
                    };
                    let new_right = Self {
                        power_state: self.power_state,
                        bottom_left: Point3::new(coord, self.bottom_left.y, self.bottom_left.z),
                        top_right: self.top_right,
                    };
                    Some((new_left, new_right))
                }
            }
            CutAxis::Y(coord) => {
                if coord <= self.bottom_left.y || coord > self.top_right.y {
                    None
                } else {
                    let new_bottom = Self {
                        power_state: self.power_state,
                        bottom_left: self.bottom_left,
                        top_right: Point3::new(self.top_right.x, coord - 1, self.top_right.z),
                    };
                    let new_top = Self {
                        power_state: self.power_state,
                        bottom_left: Point3::new(self.bottom_left.x, coord, self.bottom_left.z),
                        top_right: self.top_right,
                    };
                    Some((new_bottom, new_top))
                }
            }
            CutAxis::Z(coord) => {
                if coord <= self.bottom_left.z || coord > self.top_right.z {
                    None
                } else {
                    let new_back = Self {
                        power_state: self.power_state,
                        bottom_left: self.bottom_left,
                        top_right: Point3::new(self.top_right.x, self.top_right.y, coord - 1),
                    };
                    let new_front = Self {
                        power_state: self.power_state,
                        bottom_left: Point3::new(self.bottom_left.x, self.bottom_left.y, coord),
                        top_right: self.top_right,
                    };
                    Some((new_back, new_front))
                }
            }
        }
    }

    fn one_axis_extraction(
        &self,
        low: &CutAxis,
        high: &CutAxis,
    ) -> (Option<PowerCuboid>, PowerCuboid, Option<PowerCuboid>) {
        let mut result: (Option<PowerCuboid>, PowerCuboid, Option<PowerCuboid>) =
            (None, self.clone(), None);
        if let Some(our_first_cut_low) = self.cut_along(low) {
            result.0 = Some(our_first_cut_low.0);
            if let Some(second_cut) = our_first_cut_low.1.cut_along(high) {
                result.1 = second_cut.0;
                result.2 = Some(second_cut.1);
            } else {
                result.1 = our_first_cut_low.1;
            }
        } else if let Some(second_cut) = self.cut_along(high) {
            result.1 = second_cut.0;
            result.2 = Some(second_cut.1);
        };

        result
    }

    pub fn extract_intersection(
        &self,
        cutting_planes: &[(CutAxis, CutAxis)],
    ) -> (PowerCuboid, Vec<PowerCuboid>) {
        // Compute all the resulting cuts from removing the intersection from us
        let (left_cut, first_intersection, right_cut) =
            self.one_axis_extraction(&cutting_planes[0].0, &cutting_planes[0].1);
        let (bottom_cut, second_intersection, top_cut) =
            first_intersection.one_axis_extraction(&cutting_planes[1].0, &cutting_planes[1].1);
        let (back_cut, final_intersection, front_cut) =
            second_intersection.one_axis_extraction(&cutting_planes[2].0, &cutting_planes[2].1);

        let mut result: Vec<PowerCuboid> = Vec::new();

        if let Some(left) = left_cut {
            result.push(left)
        };

        if let Some(right) = right_cut {
            result.push(right)
        };

        if let Some(bottom) = bottom_cut {
            result.push(bottom)
        };

        if let Some(top) = top_cut {
            result.push(top)
        };

        if let Some(back) = back_cut {
            result.push(back)
        };

        if let Some(front) = front_cut {
            result.push(front)
        };

        (final_intersection, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let ref_cubes = vec![
            PowerCuboid {
                top_right: Point3::new(12, 12, 12),
                bottom_left: Point3::new(10, 10, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(11, 11, 11),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(11, 11, 11),
                bottom_left: Point3::new(9, 9, 9),
                power_state: false,
            },
            PowerCuboid {
                top_right: Point3::new(10, 10, 10),
                bottom_left: Point3::new(10, 10, 10),
                power_state: true,
            },
        ];

        assert_eq!(cubes, ref_cubes);
    }

    #[test]
    fn basic_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let intersection = cubes[0].intersect(&cubes[1]).unwrap();

        let ref_intersection = vec![
            PowerCuboid {
                top_right: Point3::new(12, 12, 12),
                bottom_left: Point3::new(11, 11, 11),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(10, 12, 12),
                bottom_left: Point3::new(10, 10, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(12, 10, 12),
                bottom_left: Point3::new(11, 10, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(12, 12, 10),
                bottom_left: Point3::new(11, 11, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(13, 11, 11),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(12, 13, 13),
                bottom_left: Point3::new(11, 13, 11),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(12, 12, 13),
                bottom_left: Point3::new(11, 11, 13),
                power_state: true,
            },
        ];

        assert_eq!(intersection, ref_intersection);
    }

    #[test]
    fn no_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let far_cube = PowerCuboid {
            top_right: Point3::new(16, 16, 16),
            bottom_left: Point3::new(15, 15, 15),
            power_state: true,
        };

        assert!(cubes[0].intersect(&far_cube).is_none());
    }

    #[test]
    fn power_switch_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        off x=11..13,y=11..13,z=11..13";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let intersection = cubes[0].intersect(&cubes[1]).unwrap();

        let ref_intersection = vec![
            PowerCuboid {
                top_right: Point3::new(12, 12, 12),
                bottom_left: Point3::new(11, 11, 11),
                power_state: false,
            },
            PowerCuboid {
                top_right: Point3::new(10, 12, 12),
                bottom_left: Point3::new(10, 10, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(12, 10, 12),
                bottom_left: Point3::new(11, 10, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(12, 12, 10),
                bottom_left: Point3::new(11, 11, 10),
                power_state: true,
            },
            PowerCuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(13, 11, 11),
                power_state: false,
            },
            PowerCuboid {
                top_right: Point3::new(12, 13, 13),
                bottom_left: Point3::new(11, 13, 11),
                power_state: false,
            },
            PowerCuboid {
                top_right: Point3::new(12, 12, 13),
                bottom_left: Point3::new(11, 11, 13),
                power_state: false,
            },
        ];

        assert_eq!(intersection, ref_intersection);
    }
}
