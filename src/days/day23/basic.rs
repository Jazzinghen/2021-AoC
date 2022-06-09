use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::TryInto;

use hashbrown::HashSet;
use itertools::Itertools;

const TARGET_LOCATIONS: usize = 7;
// Forward costs from one location to another (to be fair it could just be one long vector)
const FORWARD_COSTS: [u32; 56] = [
    3, 2, 2, 4, 6, 8, 9, 5, 4, 2, 2, 4, 6, 7, 7, 6, 4, 2, 2, 4, 5, 9, 8, 6, 4, 2, 2, 3, 4, 3, 3, 5,
    7, 9, 10, 6, 5, 3, 3, 5, 7, 8, 8, 7, 5, 3, 3, 5, 6, 10, 9, 7, 5, 3, 3, 4,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum AmphiType {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Amphipod {
    node: usize,
    race: AmphiType,
    back_in_slot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DenStatus {
    amphipods: [Amphipod; 8],
    history: Vec<([Amphipod; 8], u32)>,
    total_distance: u32,
    cost: u32,
}

impl DenStatus {
    pub fn hash_string(&self) -> String {
        let mut repr_string = String::from("...............");
        for amphipod in self.amphipods.iter() {
            let amphi_char = match amphipod.race {
                AmphiType::Amber => "A",
                AmphiType::Bronze => "B",
                AmphiType::Copper => "C",
                AmphiType::Desert => "D",
            };

            repr_string.replace_range(amphipod.node..amphipod.node + 1, amphi_char);
        }

        repr_string
    }
}

impl Ord for DenStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        // Since we want a minimum cost queue we'll have to flip the check
        let cost_cmp = other.cost.cmp(&self.cost);
        if cost_cmp == Ordering::Equal {
            other.total_distance.cmp(&self.total_distance)
        } else {
            cost_cmp
        }
    }
}

impl PartialOrd for DenStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_initial_distance(node: usize, race: AmphiType) -> u32 {
    let target_row: usize = match race {
        AmphiType::Amber => 0,
        AmphiType::Bronze => 1,
        AmphiType::Copper => 2,
        AmphiType::Desert => 3,
    };
    let row = (node - TARGET_LOCATIONS) / 4;
    let col = (node - TARGET_LOCATIONS) % 4;

    let distance = col + 2 + ((target_row.abs_diff(row) + 1) * 2);

    distance.try_into().unwrap()
}

fn compute_distance(amphipod: &Amphipod) -> u32 {
    let distance = if amphipod.back_in_slot {
        0
    } else if amphipod.node >= TARGET_LOCATIONS {
        compute_initial_distance(amphipod.node, amphipod.race)
    } else {
        let home_node = match amphipod.race {
            AmphiType::Amber => TARGET_LOCATIONS + 4,
            AmphiType::Bronze => TARGET_LOCATIONS + 5,
            AmphiType::Copper => TARGET_LOCATIONS + 6,
            AmphiType::Desert => TARGET_LOCATIONS + 7,
        };

        get_backwards_cost(amphipod.node, home_node)
    };

    distance * amphipod.race as u32
}

fn parse_input(input: &str) -> [Amphipod; 8] {
    let mut result: [Amphipod; 8] = [Amphipod {
        node: 255,
        race: AmphiType::Amber,
        back_in_slot: false,
    }; 8];
    for (row, line) in input.lines().skip(2).take(2).map(|l| l.trim()).enumerate() {
        let cleaned_string = line.trim_matches('#');

        for (col, char) in cleaned_string.chars().step_by(2).take(4).enumerate() {
            let race = match char {
                'A' => AmphiType::Amber,
                'B' => AmphiType::Bronze,
                'C' => AmphiType::Copper,
                'D' => AmphiType::Desert,
                _ => panic!("We got a strange character between the amphipods!"),
            };
            let flat_id = row * 4 + col;
            result[flat_id] = Amphipod {
                node: flat_id + TARGET_LOCATIONS,
                race,
                back_in_slot: false,
            };
        }
    }

    let arrived_amphis = check_arrived(&result);

    for id in arrived_amphis.into_iter() {
        result[id].back_in_slot = true;
    }

    result
}

fn check_arrived(amphis: &[Amphipod; 8]) -> Vec<usize> {
    let mut arrived_amphis: Vec<usize> = Vec::new();
    for (id, amphi) in amphis.iter().enumerate() {
        match amphi.race {
            AmphiType::Amber => {
                if amphi.node == TARGET_LOCATIONS + 4 {
                    arrived_amphis.push(id);
                }
            }
            AmphiType::Bronze => {
                if amphi.node == TARGET_LOCATIONS + 5 {
                    arrived_amphis.push(id);
                }
            }
            AmphiType::Copper => {
                if amphi.node == TARGET_LOCATIONS + 6 {
                    arrived_amphis.push(id);
                }
            }
            AmphiType::Desert => {
                if amphi.node == TARGET_LOCATIONS + 7 {
                    arrived_amphis.push(id);
                }
            }
        }
    }

    let mut second_slot: Vec<usize> = Vec::new();

    for id in arrived_amphis.iter() {
        let arrived = &amphis[*id];
        for (first_row_id, amphi) in amphis.iter().enumerate() {
            if amphi.race == arrived.race && amphi.node == arrived.node - 4 {
                second_slot.push(first_row_id);
            };
        }
    }

    arrived_amphis.extend(second_slot.into_iter());

    arrived_amphis
}

fn get_forward_cost(start_node: usize, target_node: usize) -> u32 {
    let flat_id = (start_node - TARGET_LOCATIONS) * TARGET_LOCATIONS + target_node;
    FORWARD_COSTS[flat_id]
}

fn get_backwards_cost(start_node: usize, target_node: usize) -> u32 {
    let flat_id = (target_node - TARGET_LOCATIONS) * TARGET_LOCATIONS + start_node;
    FORWARD_COSTS[flat_id]
}

fn _print_state(amphis: &[Amphipod; 8]) {
    let mut hallway: String = String::from("#...........#");
    let mut first_nodes: String = String::from("###.#.#.#.###");
    let mut second_nodes: String = String::from("  #.#.#.#.#");

    for amphi in amphis {
        let amphi_char = match amphi.race {
            AmphiType::Amber => "A",
            AmphiType::Bronze => "B",
            AmphiType::Copper => "C",
            AmphiType::Desert => "D",
        };
        if amphi.node < TARGET_LOCATIONS {
            let string_loc = if amphi.node < 2 {
                amphi.node + 1
            } else if amphi.node < 5 {
                amphi.node * 2
            } else {
                amphi.node + 5
            };
            hallway.replace_range(string_loc..string_loc + 1, amphi_char);
        } else if amphi.node < TARGET_LOCATIONS + 4 {
            let string_loc = (amphi.node - TARGET_LOCATIONS) * 2;
            first_nodes.replace_range(string_loc + 3..string_loc + 4, amphi_char);
        } else {
            let string_loc = (amphi.node - TARGET_LOCATIONS - 4) * 2;
            second_nodes.replace_range(string_loc + 3..string_loc + 4, amphi_char);
        }
    }

    println!("#############");
    println!("{}", hallway);
    println!("{}", first_nodes);
    println!("{}", second_nodes);
    println!("  #########");
    println!();
}

fn get_hall_move_status(moving_amphipod_id: usize, status: &DenStatus) -> Option<DenStatus> {
    let moving_amphipod = status.amphipods.get(moving_amphipod_id).unwrap();
    let (hallway_target, mut target_node) = match moving_amphipod.race {
        AmphiType::Amber => (1, TARGET_LOCATIONS + 4),
        AmphiType::Bronze => (2, TARGET_LOCATIONS + 5),
        AmphiType::Copper => (3, TARGET_LOCATIONS + 6),
        AmphiType::Desert => (4, TARGET_LOCATIONS + 7),
    };
    if status
        .amphipods
        .iter()
        .filter(|amp| amp.race == moving_amphipod.race && amp.node == target_node)
        .count()
        > 0
    {
        target_node -= 4;
    }

    let target_node_available = !status.amphipods.iter().any(|amp| amp.node == target_node);
    let path_to_target_clear = !status.amphipods.iter().any(|amp| {
        if amp.node > TARGET_LOCATIONS || amp.node == moving_amphipod.node {
            false
        } else if moving_amphipod.node > hallway_target {
            amp.node > hallway_target && amp.node < moving_amphipod.node
        } else {
            amp.node <= hallway_target && amp.node > moving_amphipod.node
        }
    });

    if target_node_available && path_to_target_clear {
        let mut new_state = status.amphipods;
        new_state[moving_amphipod_id].node = target_node;
        new_state[moving_amphipod_id].back_in_slot = true;
        let new_cost =
            get_backwards_cost(moving_amphipod.node, target_node) * moving_amphipod.race as u32;
        let mut new_history = status.history.clone();
        new_history.push((new_state, status.cost + new_cost));
        Some(DenStatus {
            amphipods: new_state,
            history: new_history,
            total_distance: new_state.iter().map(compute_distance).sum(),
            cost: status.cost + new_cost,
        })
    } else {
        None
    }
}

fn get_room_move_status(
    moving_amphipod_id: usize,
    target_hallway_cell: usize,
    status: &DenStatus,
) -> Option<DenStatus> {
    let moving_amphipod = status.amphipods.get(moving_amphipod_id).unwrap();
    let start_node = moving_amphipod.node;

    if start_node > TARGET_LOCATIONS + 4
        && status
            .amphipods
            .iter()
            .any(|amp| amp.node == start_node - 4)
    {
        return None;
    };

    let hallway_target: usize = (0..4)
        .filter_map(|node| {
            if start_node == TARGET_LOCATIONS + node || start_node == TARGET_LOCATIONS + 4 + node {
                Some(node + 1)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    let occupied_hallway_nodes = status
        .amphipods
        .iter()
        .filter_map(|amp| if amp.node < 7 { Some(amp.node) } else { None })
        .collect_vec();

    if occupied_hallway_nodes.is_empty() {
        let mut new_state = status.amphipods;
        new_state[moving_amphipod_id].node = target_hallway_cell;
        let new_cost = get_forward_cost(moving_amphipod.node, target_hallway_cell)
            * moving_amphipod.race as u32;
        let mut new_history = status.history.clone();
        new_history.push((new_state, status.cost + new_cost));

        Some(DenStatus {
            amphipods: new_state,
            history: new_history,
            total_distance: new_state.iter().map(compute_distance).sum(),
            cost: status.cost + new_cost,
        })
    } else {
        let hallway_clear = !occupied_hallway_nodes.iter().any(|occ| {
            if target_hallway_cell == *occ {
                true
            } else if target_hallway_cell < hallway_target {
                *occ <= hallway_target && *occ > target_hallway_cell
            } else {
                *occ > hallway_target && *occ <= target_hallway_cell
            }
        });
        if hallway_clear {
            let mut new_state = status.amphipods;
            new_state[moving_amphipod_id].node = target_hallway_cell;
            let new_cost = get_forward_cost(moving_amphipod.node, target_hallway_cell)
                * moving_amphipod.race as u32;
            let mut new_history = status.history.clone();
            new_history.push((new_state, status.cost + new_cost));

            Some(DenStatus {
                amphipods: new_state,
                history: new_history,
                total_distance: new_state.iter().map(compute_distance).sum(),
                cost: status.cost + new_cost,
            })
        } else {
            None
        }
    }
}

fn compute_cost_heap(amphis: [Amphipod; 8]) -> u32 {
    let mut dijkstra_heap: BinaryHeap<DenStatus> = BinaryHeap::new();
    dijkstra_heap.push(DenStatus {
        amphipods: amphis,
        history: vec![(amphis, 0)],
        total_distance: amphis.iter().map(compute_distance).sum(),
        cost: 0,
    });

    let mut seen_statuses: HashSet<String> = HashSet::new();

    while let Some(current_status) = dijkstra_heap.pop() {
        let current_hash = current_status.hash_string();
        if !seen_statuses.insert(current_hash) {
            continue;
        }
        let arrived_amphis = current_status
            .amphipods
            .iter()
            .enumerate()
            .filter_map(|(id, amp)| if amp.back_in_slot { Some(id) } else { None })
            .collect_vec();

        if arrived_amphis.len() == amphis.len() {
            return current_status.cost;
        }

        for (amphi_id, amphi) in current_status
            .amphipods
            .iter()
            .enumerate()
            .filter(|(id, _)| !arrived_amphis.contains(id))
        {
            if amphi.node < TARGET_LOCATIONS {
                if let Some(next_status) = get_hall_move_status(amphi_id, &current_status) {
                    dijkstra_heap.push(next_status);
                }
            } else {
                for target in 0usize..7 {
                    if let Some(next_status) =
                        get_room_move_status(amphi_id, target, &current_status)
                    {
                        dijkstra_heap.push(next_status);
                    }
                }
            }
        }
    }

    u32::MAX
}

pub fn part1(input: &str) {
    let amphis = parse_input(input);

    let run_cost = compute_cost_heap(amphis);

    println!("Minimum cost: {}", run_cost);
}

#[cfg(test)]
mod tests {
    use super::*;

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
                race: AmphiType::Bronze,
                back_in_slot: false,
            },
            Amphipod {
                node: 8,
                race: AmphiType::Copper,
                back_in_slot: false,
            },
            Amphipod {
                node: 9,
                race: AmphiType::Bronze,
                back_in_slot: false,
            },
            Amphipod {
                node: 10,
                race: AmphiType::Desert,
                back_in_slot: false,
            },
            Amphipod {
                node: 11,
                race: AmphiType::Amber,
                back_in_slot: true,
            },
            Amphipod {
                node: 12,
                race: AmphiType::Desert,
                back_in_slot: false,
            },
            Amphipod {
                node: 13,
                race: AmphiType::Copper,
                back_in_slot: true,
            },
            Amphipod {
                node: 14,
                race: AmphiType::Amber,
                back_in_slot: false,
            },
        ];

        assert_eq!(amphis, ref_amphis);
    }

    #[test]
    fn simple_run() {
        let input_str = "#############
        #...........#
        ###A#C#B#D###
          #A#B#C#D#
          #########";

        let amphis = parse_input(input_str);

        let run_cost = compute_cost_heap(amphis);

        assert_eq!(run_cost, 460);
    }

    #[test]
    fn less_simple_run() {
        let input_str = "#############
        #...........#
        ###D#C#B#A###
          #A#B#C#D#
          #########";

        let amphis = parse_input(input_str);

        let run_cost = compute_cost_heap(amphis);

        assert_eq!(run_cost, 8470);
    }

    #[test]
    fn full_run() {
        let input_str = "#############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########";

        let amphis = parse_input(input_str);

        let run_cost = compute_cost_heap(amphis);

        assert_eq!(run_cost, 12521);
    }
}
