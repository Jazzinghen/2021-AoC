use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

use hashbrown::HashMap;

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

enum AmphiType {
    Amber(),
    Bronze(),
    Copper(),
    Desert(),
}

struct Amphipod {
    node: u8,
    race: AmphiType,
}

#[cfg(test)]
mod tests {
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

    #[test]
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

        panic!("Oh no!");
    }
}
