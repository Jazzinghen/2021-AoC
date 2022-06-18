use itertools::Itertools;
use nalgebra::Point3;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

use std::convert::TryFrom;

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

    let x_range = (axes[0].0.min(axes[0].1), axes[0].0.max(axes[0].1) + 1);
    let y_range = (axes[1].0.min(axes[1].1), axes[1].0.max(axes[1].1) + 1);
    let z_range = (axes[2].0.min(axes[2].1), axes[2].0.max(axes[2].1) + 1);

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

    pub fn volume(&self) -> u64 {
        let sizes = (self.top_right - self.bottom_left).abs();

        sizes
            .into_iter()
            .map(|length| u64::try_from(*length).unwrap())
            .product()
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        if self.bottom_left.x > other.top_right.x || self.top_right.x < other.bottom_left.x {
            return None;
        }
        if self.bottom_left.y > other.top_right.y || self.top_right.y < other.bottom_left.y {
            return None;
        }
        if self.bottom_left.z > other.top_right.z || self.top_right.z < other.bottom_left.z {
            return None;
        }

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

        Some(Self {
            bottom_left: Point3::new(min_x, min_y, min_z),
            top_right: Point3::new(max_x, max_y, max_z),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PowerCuboid {
    cuboid: Cuboid,
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
            cuboid: Cuboid {
                top_right,
                bottom_left,
            },
            power_state,
        }
    }

    pub fn inside_volume(&self, volume: &Cuboid) -> bool {
        self.cuboid.inside_volume(volume)
    }

    pub fn intersect(&self, other: &Self) -> Option<PowerCuboid> {
        let intersection_cuboid = self.cuboid.intersect(&other.cuboid)?;

        Some(PowerCuboid {
            cuboid: intersection_cuboid,
            power_state: other.power_state,
        })
    }

    fn compute_on_volume(&self, other_cuboids: &[PowerCuboid]) -> u64 {
        let conflicts = other_cuboids
            .iter()
            .filter_map(|c| self.intersect(c))
            .collect_vec();

        let confict_volume: u64 = conflicts
            .iter()
            .enumerate()
            .map(|(idx, cube)| cube.compute_on_volume(&conflicts[idx + 1..]))
            .sum();

        self.cuboid.volume().checked_sub(confict_volume).unwrap()
    }
}

pub fn part1(input: &str) {
    let target_volume = Cuboid::new(Point3::new(-50, -50, -50), Point3::new(51, 51, 51));

    let cubes = input
        .lines()
        .filter_map(|line| {
            let (_, cube) = power_cube(line).unwrap();
            if cube.inside_volume(&target_volume) {
                Some(cube)
            } else {
                None
            }
        })
        .collect_vec();

    assert_eq!(cubes.len(), 20);

    let final_volume: u64 = cubes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.power_state)
        .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
        .sum();

    println!("Number of on voxels: {}", final_volume);
}

pub fn part2(input: &str) {
    let cubes = input
        .lines()
        .map(|line| {
            let (_, cube) = power_cube(line).unwrap();
            cube
        })
        .collect_vec();

    let final_volume: u64 = cubes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.power_state)
        .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
        .sum();

    println!("Number of on voxels: {}", final_volume);
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
                cuboid: Cuboid {
                    top_right: Point3::new(13, 13, 13),
                    bottom_left: Point3::new(10, 10, 10),
                },
                power_state: true,
            },
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(14, 14, 14),
                    bottom_left: Point3::new(11, 11, 11),
                },
                power_state: true,
            },
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(12, 12, 12),
                    bottom_left: Point3::new(9, 9, 9),
                },
                power_state: false,
            },
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(11, 11, 11),
                    bottom_left: Point3::new(10, 10, 10),
                },
                power_state: true,
            },
        ];

        assert_eq!(cubes, ref_cubes);
    }

    #[test]
    fn negative_volume() {
        let test_cuboid = Cuboid::new(Point3::new(-12, -12, -12), Point3::new(-9, -9, -9));

        assert_eq!(test_cuboid.volume(), 27);
    }

    #[test]
    fn crossover_volume() {
        let test_cuboid = Cuboid::new(Point3::new(-3, -3, -3), Point3::new(2, 2, 2));

        assert_eq!(test_cuboid.volume(), 125);
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

        let ref_intersection = PowerCuboid {
            cuboid: Cuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(11, 11, 11),
            },
            power_state: true,
        };

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(intersection, ref_intersection);
        assert_eq!(final_volume, 46);
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
            cuboid: Cuboid {
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

        let intersection = cubes[0].intersect(&cubes[0]).unwrap();

        assert_eq!(intersection, cubes[0]);
        assert_eq!(intersection.cuboid.volume(), 27);
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

        let ref_intersection = PowerCuboid {
            cuboid: Cuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(11, 11, 11),
            },
            power_state: false,
        };

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(intersection, ref_intersection);
        assert_eq!(final_volume, 19);
    }

    #[test]
    fn tri_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        off x=11..13,y=11..13,z=11..13
        on x=12..14,y=10..12,z=10..12";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 41);
    }

    #[test]
    fn longer_test() {
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

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 39);
    }

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

        let target_volume = Cuboid::new(Point3::new(-50, -50, -50), Point3::new(51, 51, 51));

        let cubes = input_string
            .lines()
            .filter_map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                if cube.inside_volume(&target_volume) {
                    Some(cube)
                } else {
                    None
                }
            })
            .collect_vec();

        assert_eq!(cubes.len(), 20);

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 590784);
    }

    #[test]
    fn full_on_bonkers_activation() {
        let input_string = "on x=-5..47,y=-31..22,z=-19..33
        on x=-44..5,y=-27..21,z=-14..35
        on x=-49..-1,y=-11..42,z=-10..38
        on x=-20..34,y=-40..6,z=-44..1
        off x=26..39,y=40..50,z=-2..11
        on x=-41..5,y=-41..6,z=-36..8
        off x=-43..-33,y=-45..-28,z=7..25
        on x=-33..15,y=-32..19,z=-34..11
        off x=35..47,y=-46..-34,z=-11..5
        on x=-14..36,y=-6..44,z=-16..29
        on x=-57795..-6158,y=29564..72030,z=20435..90618
        on x=36731..105352,y=-21140..28532,z=16094..90401
        on x=30999..107136,y=-53464..15513,z=8553..71215
        on x=13528..83982,y=-99403..-27377,z=-24141..23996
        on x=-72682..-12347,y=18159..111354,z=7391..80950
        on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
        on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
        on x=-52752..22273,y=-49450..9096,z=54442..119054
        on x=-29982..40483,y=-108474..-28371,z=-24328..38471
        on x=-4958..62750,y=40422..118853,z=-7672..65583
        on x=55694..108686,y=-43367..46958,z=-26781..48729
        on x=-98497..-18186,y=-63569..3412,z=1232..88485
        on x=-726..56291,y=-62629..13224,z=18033..85226
        on x=-110886..-34664,y=-81338..-8658,z=8914..63723
        on x=-55829..24974,y=-16897..54165,z=-121762..-28058
        on x=-65152..-11147,y=22489..91432,z=-58782..1780
        on x=-120100..-32970,y=-46592..27473,z=-11695..61039
        on x=-18631..37533,y=-124565..-50804,z=-35667..28308
        on x=-57817..18248,y=49321..117703,z=5745..55881
        on x=14781..98692,y=-1341..70827,z=15753..70151
        on x=-34419..55919,y=-19626..40991,z=39015..114138
        on x=-60785..11593,y=-56135..2999,z=-95368..-26915
        on x=-32178..58085,y=17647..101866,z=-91405..-8878
        on x=-53655..12091,y=50097..105568,z=-75335..-4862
        on x=-111166..-40997,y=-71714..2688,z=5609..50954
        on x=-16602..70118,y=-98693..-44401,z=5197..76897
        on x=16383..101554,y=4615..83635,z=-44907..18747
        off x=-95822..-15171,y=-19987..48940,z=10804..104439
        on x=-89813..-14614,y=16069..88491,z=-3297..45228
        on x=41075..99376,y=-20427..49978,z=-52012..13762
        on x=-21330..50085,y=-17944..62733,z=-112280..-30197
        on x=-16478..35915,y=36008..118594,z=-7885..47086
        off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
        off x=2032..69770,y=-71013..4824,z=7471..94418
        on x=43670..120875,y=-42068..12382,z=-24787..38892
        off x=37514..111226,y=-45862..25743,z=-16714..54663
        off x=25699..97951,y=-30668..59918,z=-15349..69697
        off x=-44271..17935,y=-9516..60759,z=49131..112598
        on x=-61695..-5813,y=40978..94975,z=8655..80240
        off x=-101086..-9439,y=-7088..67543,z=33935..83858
        off x=18020..114017,y=-48931..32606,z=21474..89843
        off x=-77139..10506,y=-89994..-18797,z=-80..59318
        off x=8476..79288,y=-75520..11602,z=-96624..-24783
        on x=-47488..-1262,y=24338..100707,z=16292..72967
        off x=-84341..13987,y=2429..92914,z=-90671..-1318
        off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
        off x=-27365..46395,y=31009..98017,z=15428..76570
        off x=-70369..-16548,y=22648..78696,z=-1892..86821
        on x=-53470..21291,y=-120233..-33476,z=-44150..38147
        off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

        let cubes = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect_vec();

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 2758514936282235);
    }
}
