use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::aoc_lib::jazz_parser;

type SteamPath = Vec<(NodeIndex, u8)>;
type GeneratedPaths = Vec<(u64, SteamPath)>;

fn parse_valve(input: &str) -> IResult<&str, (&str, u64, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), jazz_parser::u64),
        preceded(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        ),
    ))(input)
}

struct VolcanoNetwork {
    valve_graph: UnGraph<(String, u64), ()>,
    min_distance: HashMap<NodeIndex, HashMap<NodeIndex, u8>>,
    non_zero_nodes: HashSet<NodeIndex>,
    root_node: NodeIndex,
}

impl VolcanoNetwork {
    fn new() -> Self {
        Self {
            valve_graph: UnGraph::new_undirected(),
            min_distance: HashMap::new(),
            non_zero_nodes: HashSet::new(),
            root_node: NodeIndex::new(0),
        }
    }

    fn from_description(input: &str) -> Self {
        let mut new_volcano = VolcanoNetwork::new();

        let mut valve_lookup: HashMap<String, NodeIndex> = HashMap::new();

        for (valve_name, valve_value, neigh) in
            input.lines().map(|l| parse_valve(l.trim()).unwrap().1)
        {
            let curr_id = new_volcano
                .valve_graph
                .add_node((valve_name.to_string(), valve_value));

            if valve_value > 0 {
                new_volcano.non_zero_nodes.insert(curr_id);
            }

            if valve_name == "AA" {
                new_volcano.root_node = curr_id;
                new_volcano.non_zero_nodes.insert(new_volcano.root_node);
            }

            valve_lookup.insert(valve_name.to_string(), curr_id);

            for neigh_name in neigh {
                if let Some(neigh_id) = valve_lookup.get(neigh_name) {
                    new_volcano.valve_graph.add_edge(curr_id, *neigh_id, ());
                }
            }
        }

        for node_id in new_volcano.non_zero_nodes.iter() {
            new_volcano.min_distance.insert(
                *node_id,
                HashMap::from_iter(
                    dijkstra(&new_volcano.valve_graph, *node_id, None, |_| 1u8)
                        .into_iter()
                        .filter(|(id, _)| new_volcano.non_zero_nodes.contains(id)),
                ),
            );
        }

        new_volcano
    }

    pub fn compute_max_steam(&self, max_time: u8) -> (u64, SteamPath) {
        let mut active_nodes: HashSet<NodeIndex> = HashSet::new();

        let all_paths = self
            .compute_steam(
                &mut active_nodes,
                &self.root_node,
                i16::from(max_time - (self.valve_graph[self.root_node].1 > 0) as u8),
            )
            .unwrap();

        all_paths
            .into_iter()
            .max_by(|(left_steam, _), (right_steam, _)| left_steam.cmp(right_steam))
            .unwrap()
    }

    pub fn compute_dual_max(&self, max_time: u8) -> u64 {
        let mut active_nodes: HashSet<NodeIndex> = HashSet::new();

        let all_paths = self
            .compute_steam(
                &mut active_nodes,
                &self.root_node,
                i16::from(max_time - (self.valve_graph[self.root_node].1 > 0) as u8),
            )
            .unwrap();

        let mut path_sets: Vec<(u64, HashSet<NodeIndex>)> = Vec::new();

        for (steam, path) in all_paths.iter() {
            let path_set: HashSet<NodeIndex> = path
                .iter()
                .filter_map(|(id, _)| {
                    if *id != self.root_node {
                        Some(id)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect();
            path_sets.push((*steam, path_set));
        }

        let mut result: u64 = 0;

        for (my_id, (my_steam, my_set)) in path_sets.iter().enumerate() {
            let sets_to_test = path_sets
                .iter()
                .skip(my_id + 1)
                .filter(|(steam, set)| {
                    if (steam + my_steam) > result {
                        my_set.is_disjoint(set)
                    } else {
                        false
                    }
                })
                .collect_vec();

            for (elephant_steam, _) in sets_to_test.into_iter() {
                result = result.max(my_steam + elephant_steam);
            }
        }

        result
    }

    fn compute_steam(
        &self,
        active_nodes: &mut HashSet<NodeIndex>,
        start_node: &NodeIndex,
        minutes_remaining: i16,
    ) -> Option<GeneratedPaths> {
        if minutes_remaining <= 0 {
            return None;
        }

        active_nodes.insert(*start_node);

        let node_throughput = self.valve_graph[*start_node].1;
        let node_contribution = node_throughput * u64::try_from(minutes_remaining).unwrap();

        let mut paths_from_here: Vec<(u64, Vec<(NodeIndex, u8)>)> = Vec::new();
        paths_from_here.push((
            node_contribution,
            vec![(*start_node, u8::try_from(minutes_remaining).unwrap())],
        ));

        let base_path: Vec<(NodeIndex, u8)> =
            vec![(*start_node, u8::try_from(minutes_remaining).unwrap())];
        let min_distances = self.min_distance.get(start_node).unwrap();

        let available_ids = self
            .non_zero_nodes
            .iter()
            .filter(|&id| !active_nodes.contains(id))
            .collect_vec();

        for next_id in available_ids {
            let distance = min_distances[next_id] + 1u8;

            let next_remaining = minutes_remaining - i16::from(distance);

            if let Some(next_paths) = self.compute_steam(active_nodes, next_id, next_remaining) {
                for (next_steam, next_path) in next_paths {
                    let mut full_next = base_path.clone();
                    full_next.extend(next_path.into_iter());
                    paths_from_here.push((node_contribution + next_steam, full_next));
                }
            }
        }
        active_nodes.remove(start_node);

        Some(paths_from_here)
    }
}

pub fn part1(input: &str) {
    let volcano = VolcanoNetwork::from_description(input);
    let (max_steam, _) = volcano.compute_max_steam(30);

    println!("Max steam released: {}", max_steam);
}

pub fn part2(input: &str) {
    let volcano = VolcanoNetwork::from_description(input);
    let max_steam = volcano.compute_dual_max(26);

    println!("Max steam when running two agents: {}", max_steam);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve JJ has flow rate=21; tunnel leads to valve II
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve II has flow rate=0; tunnels lead to valves AA, JJ";

    #[test]
    fn simple_volcano() {
        let volcano = VolcanoNetwork::from_description(INPUT_STRING);
        let (max_steam, _) = volcano.compute_max_steam(30);

        assert_eq!(max_steam, 1651);
    }

    #[test]
    fn simple_dual() {
        let volcano = VolcanoNetwork::from_description(INPUT_STRING);
        let max_steam = volcano.compute_dual_max(26);

        assert_eq!(max_steam, 1707);
    }
}
