use std::convert::TryFrom;

use itertools::Itertools;

const TARGET_LOCATIONS: u8 = 7;
// Forward costs from one location to another (to be fair it could just be one long vector)
const FORWARD_COSTS: [u8; 56] = [
    2, 1, 1, 3, 5, 7, 8, 4, 3, 1, 1, 3, 5, 6, 6, 5, 3, 1, 1, 3, 4, 8, 7, 5, 3, 1, 1, 2, 3, 2, 2, 4,
    6, 8, 9, 5, 4, 2, 2, 4, 6, 7, 7, 6, 4, 2, 2, 4, 5, 9, 8, 6, 4, 2, 2, 3,
];

/*
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
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AmphiType {
    Amber(),
    Bronze(),
    Copper(),
    Desert(),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Amphipod {
    node: u8,
    race: AmphiType,
    back_in_slot: bool,
}

fn parse_input(input: &str) -> [Amphipod; 8] {
    let mut result: [Amphipod; 8] = [Amphipod {
        node: 255,
        race: AmphiType::Amber(),
        back_in_slot: false,
    }; 8];
    for (row, line) in input.lines().skip(2).take(2).map(|l| l.trim()).enumerate() {
        let cleaned_string = line.trim_matches('#');

        println!("Cleaned string: {}", cleaned_string);

        for (col, char) in cleaned_string.chars().step_by(2).take(4).enumerate() {
            let race = match char {
                'A' => AmphiType::Amber(),
                'B' => AmphiType::Bronze(),
                'C' => AmphiType::Copper(),
                'D' => AmphiType::Desert(),
                _ => panic!("We got a strange character between the amphipods!"),
            };
            let flat_id = row * 4 + col;
            result[flat_id] = Amphipod {
                node: u8::try_from(flat_id).unwrap() + TARGET_LOCATIONS,
                race,
                back_in_slot: false,
            };
        }
    }

    for (id, amphi) in result.iter_mut().skip(4).take(4).enumerate() {
        match amphi.race {
            AmphiType::Amber() => {
                if amphi.node == TARGET_LOCATIONS + 4 {
                    amphi.back_in_slot = true
                }
            }
            AmphiType::Bronze() => {
                if amphi.node == TARGET_LOCATIONS + 5 {
                    amphi.back_in_slot = true
                }
            }
            AmphiType::Copper() => {
                if amphi.node == TARGET_LOCATIONS + 6 {
                    amphi.back_in_slot = true
                }
            }
            AmphiType::Desert() => {
                if amphi.node == TARGET_LOCATIONS + 7 {
                    amphi.back_in_slot = true
                }
            }
        }
    }

    for id in 0usize..4 {
        if result[id + 4].back_in_slot {
            let amphi_id = result[id].node;
            match result[id].race {
                AmphiType::Amber() => {
                    if amphi_id == TARGET_LOCATIONS {
                        result[id].back_in_slot = true
                    }
                }
                AmphiType::Bronze() => {
                    if amphi_id == TARGET_LOCATIONS + 1 {
                        result[id].back_in_slot = true
                    }
                }
                AmphiType::Copper() => {
                    if amphi_id == TARGET_LOCATIONS + 2 {
                        result[id].back_in_slot = true
                    }
                }
                AmphiType::Desert() => {
                    if amphi_id == TARGET_LOCATIONS + 3 {
                        result[id].back_in_slot = true
                    }
                }
            }
        }
    }

    result
}

fn find_cost(amphis: [Amphipod; 8]) -> u32 {
    let mut cost: u32 = 0;

    let occupied_hallway_slots = amphis
        .iter()
        .filter(|amphi| amphi.node < 7)
        .map(|amphi| amphi.node)
        .collect_vec();

    cost
}

#[cfg(test)]
mod tests {

    use hashbrown::HashMap;
    use itertools::Itertools;

    use super::*;

    const forward_network_A: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (7, 1, 1),
        (7, 2, 1),
        (2, 3, 2),
        (3, 4, 2),
        (4, 5, 2),
        (5, 6, 1),
    ];

    const forward_network_B: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (8, 2, 1),
        (8, 3, 1),
        (2, 1, 2),
        (3, 4, 2),
        (4, 5, 2),
        (5, 6, 1),
    ];

    const forward_network_C: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (9, 3, 1),
        (9, 4, 1),
        (2, 1, 2),
        (3, 2, 2),
        (4, 5, 2),
        (5, 6, 1),
    ];

    const forward_network_D: [(u8, u8, u8); 7] = [
        (1, 0, 1),
        (10, 4, 1),
        (10, 5, 1),
        (2, 1, 2),
        (3, 2, 2),
        (4, 3, 2),
        (5, 6, 1),
    ];

    fn explore_network(network: &[(u8, u8, u8); 7], start_node: u8) -> HashMap<(u8, u8), u8> {
        let mut costs: HashMap<(u8, u8), u8> = HashMap::new();
        let mut exploration: Vec<(u8, u8)> = network
            .iter()
            .filter_map(|(start, end, cost)| {
                if *start == start_node {
                    Some((*end, *cost))
                } else {
                    None
                }
            })
            .collect_vec();

        while let Some((next_node, next_cost)) = exploration.pop() {
            costs.insert((start_node, next_node), next_cost);
            costs.insert((start_node + 4, next_node), next_cost + 1);
            exploration.extend(network.iter().filter_map(|(start, end, cost)| {
                if *start == next_node {
                    Some((*end, *cost + next_cost))
                } else {
                    None
                }
            }));
        }

        costs
    }

    fn network_gen() {
        let mut total_forward_map = explore_network(&forward_network_A, 7);
        total_forward_map.extend(explore_network(&forward_network_B, 8));
        total_forward_map.extend(explore_network(&forward_network_C, 9));
        total_forward_map.extend(explore_network(&forward_network_D, 10));

        let total_forward_network = total_forward_map
            .into_iter()
            .sorted_by(|left, right| left.0.cmp(&right.0))
            .collect_vec();

        println!("Length: {}", total_forward_network.len());
        println!("{:?}", total_forward_network);
    }

    #[test]
    fn parse() {
        let input_str = "#############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########";

        let amphis = parse_input(input_str);

        let ref_amphis = [
            Amphipod {
                node: 7,
                race: AmphiType::Bronze(),
                back_in_slot: false,
            },
            Amphipod {
                node: 8,
                race: AmphiType::Copper(),
                back_in_slot: false,
            },
            Amphipod {
                node: 9,
                race: AmphiType::Bronze(),
                back_in_slot: false,
            },
            Amphipod {
                node: 10,
                race: AmphiType::Desert(),
                back_in_slot: false,
            },
            Amphipod {
                node: 11,
                race: AmphiType::Amber(),
                back_in_slot: true,
            },
            Amphipod {
                node: 12,
                race: AmphiType::Desert(),
                back_in_slot: false,
            },
            Amphipod {
                node: 13,
                race: AmphiType::Copper(),
                back_in_slot: true,
            },
            Amphipod {
                node: 14,
                race: AmphiType::Amber(),
                back_in_slot: false,
            },
        ];

        assert_eq!(amphis, ref_amphis);
    }
}
