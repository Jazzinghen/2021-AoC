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
    minimized_graph: UnGraph<NodeIndex, u8>,
    min_distance: HashMap<NodeIndex, HashMap<NodeIndex, u8>>,
    non_zero_nodes: HashSet<NodeIndex>,
    root_node: NodeIndex,
    min_root: NodeIndex,
}

impl VolcanoNetwork {
    fn new() -> Self {
        Self {
            valve_graph: UnGraph::new_undirected(),
            minimized_graph: UnGraph::new_undirected(),
            min_distance: HashMap::new(),
            non_zero_nodes: HashSet::new(),
            root_node: NodeIndex::new(0),
            min_root: NodeIndex::new(0),
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

        for node_id in new_volcano.non_zero_nodes.iter() {
            let min_id = new_volcano.minimized_graph.add_node(*node_id);
            if new_volcano.valve_graph[*node_id].0 == "AA" {
                new_volcano.min_root = min_id;
            }
        }

        for node_id in new_volcano.minimized_graph.node_indices() {
            for other_id in new_volcano
                .minimized_graph
                .node_indices()
                .filter(|id| *id != node_id)
            {
                let standard_node = new_volcano.minimized_graph[node_id];
                let standard_other = new_volcano.minimized_graph[other_id];
                let weight = new_volcano.min_distance[&standard_node][&standard_other];
                new_volcano
                    .minimized_graph
                    .update_edge(node_id, other_id, weight);
            }
        }

        new_volcano
    }

    pub fn compute_max_steam(&self) -> (u64, Vec<(NodeIndex, u8, u64)>) {
        let mut active_nodes: HashSet<NodeIndex> = HashSet::new();

        active_nodes.insert(self.root_node);
        let first_step: (NodeIndex, u8) = if self.valve_graph[self.root_node].1 > 0 {
            (self.root_node, 1)
        } else {
            (self.root_node, 0)
        };

        self.compute_steam(&mut active_nodes, &self.root_node, 30 - first_step.1)
    }

    fn compute_path_steam(&self, path: &[(NodeIndex, u8)]) -> u64 {
        path.iter()
            .rev()
            .fold((0u8, 0u64), |(time, steam), (id, timestamp)| {
                let new_time = time + (30 - *timestamp);
                let activation = self.valve_graph[*id].1;
                let new_steam = steam + (activation * u64::from(new_time));
                (new_time, new_steam)
            })
            .1
    }

    fn compute_steam(
        &self,
        active_nodes: &mut HashSet<NodeIndex>,
        start_node: &NodeIndex,
        minutes_remaining: u8,
    ) -> (u64, Vec<(NodeIndex, u8, u64)>) {
        let current_steam: u64 = active_nodes
            .iter()
            .map(|node_id| self.valve_graph[*node_id].1)
            .sum();
        let mut max_steam: u64 = current_steam * u64::from(minutes_remaining);
        let mut path: Vec<(NodeIndex, u8, u64)> = vec![(*start_node, 30 - minutes_remaining, 0)];

        let mut max_path: Vec<(NodeIndex, u8, u64)> = Vec::new();
        let min_distances = self.min_distance.get(start_node).unwrap();
        let available_ids = self
            .non_zero_nodes
            .iter()
            .filter(|&id| !active_nodes.contains(id))
            .collect_vec();

        for next_id in available_ids {
            let distance = min_distances[next_id] + 1u8;

            if distance < minutes_remaining {
                active_nodes.insert(*next_id);

                let next_remaining = minutes_remaining - distance;

                let (mut current_max, mut path) =
                    self.compute_steam(active_nodes, next_id, next_remaining);

                current_max += current_steam * u64::from(distance);
                for (_, _, cumulative) in path.iter_mut() {
                    *cumulative += current_steam * u64::from(distance);
                }

                if current_max > max_steam {
                    max_steam = current_max;
                    max_path = path;
                }

                active_nodes.remove(next_id);
            }
        }

        path.append(&mut max_path);

        (max_steam, path)
    }
}

pub fn part1(input: &str) {
    let volcano = VolcanoNetwork::from_description(input);

    println!("Actual network:");
    println!("\tUseful nodes: ");
    for id in volcano
        .non_zero_nodes
        .iter()
        .sorted_by(|left_id, right_id| left_id.cmp(right_id))
    {
        print!("{}, ", volcano.valve_graph[*id].0);
    }
    println!();

    let (max_steam, max_path) = volcano.compute_max_steam();

    println!("Maximun path [Length: {}]:", max_path.len());
    for (id, minute, steam_at_minute) in max_path.iter() {
        println!(
            "\t[{}, {}] {} - {}",
            minute, steam_at_minute, volcano.valve_graph[*id].0, volcano.valve_graph[*id].1,
        );
    }
    println!();

    println!("Max steam released: {}", max_steam);
}

/*
pub fn part2(input: &str) {
    let hills_range = HillsRange::from_grid(input);
    let shortestest = hills_range.find_shortestest_path().unwrap();

    println!("Shortestest path to the top: {}", shortestest);
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn simple_volcano() {
        let volcano = VolcanoNetwork::from_description(INPUT_STRING);
        let (max_steam, _) = volcano.compute_max_steam();

        assert_eq!(max_steam, 1651);
    }
}
