use itertools::Itertools;
use nalgebra::Point3;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

enum CutPlane {
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

pub fn power_cube(input: &str) -> IResult<&str, PowerCuboid> {
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
pub struct Cuboid {
    top_right: Point3<i32>,
    bottom_left: Point3<i32>,
}

impl Cuboid {
    pub fn new(bottom_left: Point3<i32>, top_right: Point3<i32>) -> Self {
        Self {
            top_right,
            bottom_left,
        }
    }

    pub fn inside_volume(&self, volume: &Cuboid) -> bool {
        self.bottom_left >= volume.bottom_left && self.top_right <= volume.top_right
    }

    pub fn intersection_planes(&self, other: &Self) -> Option<[(CutPlane, CutPlane); 3]> {
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

        Some([
            (CutPlane::X(min_x), CutPlane::X(max_x + 1)),
            (CutPlane::Y(min_y), CutPlane::Y(max_y + 1)),
            (CutPlane::Z(min_z), CutPlane::Z(max_z + 1)),
        ])
    }

    pub fn extract_intersection(
        &self,
        cutting_planes: &[(CutPlane, CutPlane)],
    ) -> (Cuboid, Vec<Cuboid>) {
        // Compute all the resulting cuts from removing the intersection from us
        let (left_cut, first_intersection, right_cut) =
            self.one_axis_extraction(&cutting_planes[0].0, &cutting_planes[0].1);
        let (bottom_cut, second_intersection, top_cut) =
            first_intersection.one_axis_extraction(&cutting_planes[1].0, &cutting_planes[1].1);
        let (back_cut, final_intersection, front_cut) =
            second_intersection.one_axis_extraction(&cutting_planes[2].0, &cutting_planes[2].1);

        let mut result: Vec<Cuboid> = Vec::new();

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

    fn cut_along(&self, cut: &CutPlane) -> Option<(Self, Self)> {
        match *cut {
            CutPlane::X(coord) => {
                if coord <= self.bottom_left.x || coord > self.top_right.x {
                    None
                } else {
                    let new_left = Self {
                        bottom_left: self.bottom_left,
                        top_right: Point3::new(coord - 1, self.top_right.y, self.top_right.z),
                    };
                    let new_right = Self {
                        bottom_left: Point3::new(coord, self.bottom_left.y, self.bottom_left.z),
                        top_right: self.top_right,
                    };
                    Some((new_left, new_right))
                }
            }
            CutPlane::Y(coord) => {
                if coord <= self.bottom_left.y || coord > self.top_right.y {
                    None
                } else {
                    let new_bottom = Self {
                        bottom_left: self.bottom_left,
                        top_right: Point3::new(self.top_right.x, coord - 1, self.top_right.z),
                    };
                    let new_top = Self {
                        bottom_left: Point3::new(self.bottom_left.x, coord, self.bottom_left.z),
                        top_right: self.top_right,
                    };
                    Some((new_bottom, new_top))
                }
            }
            CutPlane::Z(coord) => {
                if coord <= self.bottom_left.z || coord > self.top_right.z {
                    None
                } else {
                    let new_back = Self {
                        bottom_left: self.bottom_left,
                        top_right: Point3::new(self.top_right.x, self.top_right.y, coord - 1),
                    };
                    let new_front = Self {
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
        low: &CutPlane,
        high: &CutPlane,
    ) -> (Option<Self>, Self, Option<Self>) {
        let mut result: (Option<Self>, Self, Option<Self>) = (None, self.clone(), None);
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IntersectionType {
    Equal(),
    Standard(Vec<PowerCuboid>),
    Superset(Vec<PowerCuboid>),
    Subset(Vec<PowerCuboid>),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PowerCuboid {
    volume: Cuboid,
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
            volume: Cuboid {
                top_right,
                bottom_left,
            },
            power_state,
        }
    }

    pub fn inside_volume(&self, volume: &Cuboid) -> bool {
        self.volume.inside_volume(volume)
    }

    pub fn intersect(&self, other: &Self) -> Option<(PowerCuboid, IntersectionType)> {
        let cutting_planes = self.volume.intersection_planes(&other.volume)?;

        // Compute all the resulting cuts from removing the intersection from us and the other cube
        let (our_intersection, our_cuts) = self.volume.extract_intersection(&cutting_planes);
        let (other_intersection, other_cuts) = other.volume.extract_intersection(&cutting_planes);

        // The intersection from both extractions
        assert!(our_intersection == other_intersection);

        let intersection = PowerCuboid {
            volume: other_intersection,
            power_state: other.power_state,
        };

        if our_cuts.is_empty() && other_cuts.is_empty() {
            return Some((intersection, IntersectionType::Equal()));
        }

        let empty_ours = our_cuts.is_empty();
        let empty_other = other_cuts.is_empty();

        let mut final_cuts: Vec<PowerCuboid> = our_cuts
            .into_iter()
            .map(|volume| Self {
                volume,
                power_state: self.power_state,
            })
            .collect_vec();
        final_cuts.extend(other_cuts.into_iter().map(|volume| Self {
            volume,
            power_state: other.power_state,
        }));

        if empty_ours {
            return Some((intersection, IntersectionType::Subset(final_cuts)));
        };

        if empty_other {
            return Some((intersection, IntersectionType::Superset(final_cuts)));
        }

        Some((intersection, IntersectionType::Standard(final_cuts)))
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
                volume: Cuboid {
                    top_right: Point3::new(12, 12, 12),
                    bottom_left: Point3::new(10, 10, 10),
                },
                power_state: true,
            },
            PowerCuboid {
                volume: Cuboid {
                    top_right: Point3::new(13, 13, 13),
                    bottom_left: Point3::new(11, 11, 11),
                },
                power_state: true,
            },
            PowerCuboid {
                volume: Cuboid {
                    top_right: Point3::new(11, 11, 11),
                    bottom_left: Point3::new(9, 9, 9),
                },
                power_state: false,
            },
            PowerCuboid {
                volume: Cuboid {
                    top_right: Point3::new(10, 10, 10),
                    bottom_left: Point3::new(10, 10, 10),
                },
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

        let ref_intersection = (
            PowerCuboid {
                volume: Cuboid {
                    top_right: Point3::new(12, 12, 12),
                    bottom_left: Point3::new(11, 11, 11),
                },
                power_state: true,
            },
            IntersectionType::Standard(vec![
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(10, 12, 12),
                        bottom_left: Point3::new(10, 10, 10),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 10, 12),
                        bottom_left: Point3::new(11, 10, 10),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 12, 10),
                        bottom_left: Point3::new(11, 11, 10),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(13, 13, 13),
                        bottom_left: Point3::new(13, 11, 11),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 13, 13),
                        bottom_left: Point3::new(11, 13, 11),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 12, 13),
                        bottom_left: Point3::new(11, 11, 13),
                    },
                    power_state: true,
                },
            ]),
        );

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
            volume: Cuboid {
                top_right: Point3::new(16, 16, 16),
                bottom_left: Point3::new(15, 15, 15),
            },
            power_state: true,
        };

        assert!(cubes[0].intersect(&far_cube).is_none());
    }

    #[test]
    fn self_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let (intersection, int_type) = cubes[0].intersect(&cubes[0]).unwrap();

        assert_eq!(int_type, IntersectionType::Equal());
        assert_eq!(intersection, cubes[0]);
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

        let ref_intersection = (
            PowerCuboid {
                volume: Cuboid {
                    top_right: Point3::new(12, 12, 12),
                    bottom_left: Point3::new(11, 11, 11),
                },
                power_state: false,
            },
            IntersectionType::Standard(vec![
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(10, 12, 12),
                        bottom_left: Point3::new(10, 10, 10),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 10, 12),
                        bottom_left: Point3::new(11, 10, 10),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 12, 10),
                        bottom_left: Point3::new(11, 11, 10),
                    },
                    power_state: true,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(13, 13, 13),
                        bottom_left: Point3::new(13, 11, 11),
                    },
                    power_state: false,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 13, 13),
                        bottom_left: Point3::new(11, 13, 11),
                    },
                    power_state: false,
                },
                PowerCuboid {
                    volume: Cuboid {
                        top_right: Point3::new(12, 12, 13),
                        bottom_left: Point3::new(11, 11, 13),
                    },
                    power_state: false,
                },
            ]),
        );

        assert_eq!(intersection, ref_intersection);
    }

    /*
    #[test]
    fn full_centre_power_cycle() {
        let input_string = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682";

        let target_volume = Cuboid::new(Point3::new(-50, -50, -50), Point3::new(50, 50, 50));

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .filter_map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                if cube.inside_volume(&target_volume) {
                    Some(cube)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(cubes.len(), 21);

        let mut final_power_cuboids = vec![cubes[0].clone()];

        for next_cuboid in cubes.into_iter().skip(1) {
            let mut cuboid_exploration = final_power_cuboids;
            let mut new_power_cuboids: Vec<PowerCuboid> = Vec::new();

            while let Some(other_cuboid) = cuboid_exploration.pop() {
                if let Some(cuts) = other_cuboid.intersect(&next_cuboid) {
                } else {
                    new_power_cuboids.push(other_cuboid);
                }
            }

            final_power_cuboids = new_power_cuboids;
        }
    }*/
}
