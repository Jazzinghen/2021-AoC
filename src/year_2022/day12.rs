use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};

type Point = (usize, usize);

struct HillsRange {
    hill_graph: DiGraph<char, ()>,
    rows: usize,
    columns: usize,
    start_node: NodeIndex,
    goal_node: NodeIndex,
}

impl HillsRange {
    fn new() -> Self {
        HillsRange {
            hill_graph: DiGraph::new(),
            rows: 0,
            columns: 0,
            start_node: NodeIndex::new(0),
            goal_node: NodeIndex::new(0),
        }
    }

    fn from_grid(input: &str) -> Self {
        let mut new_range = HillsRange::new();

        new_range.rows = input.lines().count();
        new_range.columns = input.lines().next().unwrap().trim().len();

        for (cell_idx, height) in input.chars().filter(|c| c.is_alphabetic()).enumerate() {
            let current_height = match height {
                'S' => {
                    new_range.start_node = NodeIndex::new(cell_idx);
                    'a'
                }
                'E' => {
                    new_range.goal_node = NodeIndex::new(cell_idx);
                    'z'
                }
                _ => height,
            };
            new_range.hill_graph.add_node(current_height);
        }

        for cell_coords in (0..new_range.rows).cartesian_product(0..new_range.columns) {
            let current_idx = NodeIndex::new(new_range.linear_id(&cell_coords).unwrap());

            if cell_coords.1 > 0 {
                let left_coord = (cell_coords.0, cell_coords.1 - 1);
                let left_idx = NodeIndex::new(new_range.linear_id(&left_coord).unwrap());
                if new_range.is_neighbour(&current_idx, &left_idx) {
                    new_range.hill_graph.add_edge(current_idx, left_idx, ());
                }
            }

            if cell_coords.1 < new_range.columns - 1 {
                let right_coord = (cell_coords.0, cell_coords.1 + 1);
                let right_idx = NodeIndex::new(new_range.linear_id(&right_coord).unwrap());
                if new_range.is_neighbour(&current_idx, &right_idx) {
                    new_range.hill_graph.add_edge(current_idx, right_idx, ());
                }
            }

            if cell_coords.0 > 0 {
                let up_coord = (cell_coords.0 - 1, cell_coords.1);
                let up_idx = NodeIndex::new(new_range.linear_id(&up_coord).unwrap());
                if new_range.is_neighbour(&current_idx, &up_idx) {
                    new_range.hill_graph.add_edge(current_idx, up_idx, ());
                }
            }

            if cell_coords.0 < new_range.rows - 1 {
                let down_coord = (cell_coords.0 + 1, cell_coords.1);
                let down_idx = NodeIndex::new(new_range.linear_id(&down_coord).unwrap());
                if new_range.is_neighbour(&current_idx, &down_idx) {
                    new_range.hill_graph.add_edge(current_idx, down_idx, ());
                }
            }
        }

        new_range
    }

    fn linear_id(&self, coords: &Point) -> Result<usize, &str> {
        if coords.1 >= self.columns || coords.0 >= self.rows {
            Err("Requested coordinate is out of bounds!")
        } else {
            Ok(coords.0 * self.columns + coords.1)
        }
    }

    fn is_neighbour(&self, start: &NodeIndex, end: &NodeIndex) -> bool {
        let start_height = self.hill_graph[*start];
        let end_height = self.hill_graph[*end];
        end_height <= start_height || end_height == char::from_u32(start_height as u32 + 1).unwrap()
    }

    pub fn find_shortest_path(&self) -> Option<usize> {
        let path_cost = dijkstra(
            &self.hill_graph,
            self.start_node,
            Some(self.goal_node),
            |_| 1,
        );

        path_cost
            .get(&self.goal_node)
            .and_then(|goal_cost| usize::try_from(*goal_cost).ok())
    }

    pub fn find_shortestest_path(&self) -> Option<usize> {
        let starting_points: Vec<NodeIndex> = self
            .hill_graph
            .node_indices()
            .filter(|idx| self.hill_graph[*idx] == 'a')
            .collect_vec();

        let mut shortestest_path: Option<usize> = None;
        for start in starting_points.into_iter() {
            let path_cost = dijkstra(&self.hill_graph, start, Some(self.goal_node), |_| 1);
            if let Some(current_shortest_path) = path_cost
                .get(&self.goal_node)
                .and_then(|goal_cost| usize::try_from(*goal_cost).ok())
            {
                if let Some(shortestest) = shortestest_path {
                    shortestest_path = Some(shortestest.min(current_shortest_path));
                } else {
                    shortestest_path = Some(current_shortest_path);
                }
            }
        }

        shortestest_path
    }
}

pub fn part1(input: &str) {
    let hills = HillsRange::from_grid(input);
    let shortest_path = hills
        .find_shortest_path()
        .expect("We should have a shortest path, man!");

    println!("Shortest path to the top: {}", shortest_path);
}

pub fn part2(input: &str) {
    let hills_range = HillsRange::from_grid(input);
    let shortestest = hills_range.find_shortestest_path().unwrap();

    println!("Shortestest path to the top: {}", shortestest);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    #[test]
    fn simple_climb() {
        let hills_range = HillsRange::from_grid(INPUT_STRING);
        let shortest = hills_range.find_shortest_path().unwrap();

        assert_eq!(shortest, 31usize);
    }

    #[test]
    fn simple_shortestest() {
        let hills_range = HillsRange::from_grid(INPUT_STRING);
        let shortestest = hills_range.find_shortestest_path().unwrap();

        assert_eq!(shortestest, 29usize);
    }
}
