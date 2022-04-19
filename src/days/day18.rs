use core::fmt;

use hashbrown::{HashMap, HashSet};

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

use crate::aoc_lib::jazz_data::{BinaryNode, BinaryTree};

#[derive(PartialEq)]
struct SailfishNumber {
    data: BinaryTree<u8>,
    deep_nodes: Vec<usize>,
    large_nodes: Vec<usize>,
}

impl SailfishNumber {
    pub fn new(arena: Vec<BinaryNode<u8>>, free_idx: Vec<usize>) -> Self {
        let mut new_number = Self {
            data: BinaryTree::<u8>::new(arena, free_idx),
            deep_nodes: vec![],
            large_nodes: vec![],
        };
        new_number.update_reduceable();

        new_number
    }

    pub fn sum(&self, other: &Self) -> Self {
        let updated_left = update_indices(&self.data.arena[..], 1);
        let left_max = updated_left
            .iter()
            .max_by(|node, other| node.idx.cmp(&other.idx))
            .unwrap()
            .idx;
        let updated_right = update_indices(&other.data.arena[..], left_max + 1);

        let mut new_node: BinaryNode<u8> = BinaryNode::new(0, None, 0);
        new_node.left = Some(1);
        new_node.right = Some(left_max + 1);

        let mut arena: Vec<BinaryNode<u8>> = vec![new_node];

        arena.extend(updated_left.into_iter());
        arena.extend(updated_right.into_iter());

        let mut available: Vec<usize> = self
            .data
            .get_free_nodes()
            .iter()
            .map(|idx| idx + 1)
            .collect();
        available.extend(
            other
                .data
                .get_free_nodes()
                .iter()
                .map(|idx| idx + left_max + 1),
        );

        Self::new(arena, available)
    }

    pub fn reduce(&mut self) {
        let mut reduction_steps: usize = 0;
        while !self.deep_nodes.is_empty() || !self.large_nodes.is_empty() {
            reduction_steps += 1;
            if reduction_steps > 1000 {
                panic!("What the hell!");
            }
            println!("Current number: {}", self);
            if !self.deep_nodes.is_empty() {
                self.explode(self.deep_nodes[0]);
            } else if !self.large_nodes.is_empty() {
                self.split(self.large_nodes[0]);
            }
            self.update_reduceable();
        }
    }

    fn update_reduceable(&mut self) {
        let mut exploration_stack: Vec<usize> = vec![0];
        let mut visited_nodes: HashSet<usize> = HashSet::new();

        self.deep_nodes.clear();
        self.large_nodes.clear();

        while let Some(curr_idx) = exploration_stack.pop() {
            if visited_nodes.contains(&curr_idx) {
                let fucked_nodes: Vec<&BinaryNode<_>> = self
                    .data
                    .arena
                    .iter()
                    .filter(|node| {
                        node.left.map_or(false, |idx| idx == curr_idx)
                            || node.right.map_or(false, |idx| idx == curr_idx)
                    })
                    .collect();

                println!("There's a loop somewere! {:?}", fucked_nodes);

                panic!("What the fuck!");
            }

            visited_nodes.insert(curr_idx);

            let curr_node = self.data.arena.get(curr_idx).unwrap();
            if let Some(right_idx) = curr_node.right {
                exploration_stack.push(right_idx);
            }
            if let Some(left_idx) = curr_node.left {
                exploration_stack.push(left_idx);
            }
            if let Some(val) = curr_node.value {
                if val > 9 {
                    self.large_nodes.push(curr_idx);
                }
            } else if curr_node.depth >= 4 {
                self.deep_nodes.push(curr_idx);
            }
        }
    }

    fn explode(&mut self, idx: usize) {
        let mut parent = self.data.arena[idx].parent;
        let mut left_branch: Option<usize> = None;
        let mut left_visited: HashSet<usize> = HashSet::new();
        left_visited.insert(idx);
        while let Some(next_parent) = parent {
            let curr_node = self.data.arena.get(next_parent).unwrap();
            left_visited.insert(curr_node.idx);
            parent = curr_node.parent;
            if let Some(left_idx) = curr_node.left {
                if !left_visited.contains(&left_idx) {
                    parent = None;
                    left_branch = curr_node.left;
                }
            };
        }

        let left_idx_delete = self.data.arena[idx].left.unwrap();
        if let Some(left) = left_branch {
            let left_value = self.data.arena[left_idx_delete].value.unwrap();
            let mut curr_node = self.data.arena.get_mut(left).unwrap();
            while curr_node.value.is_none() {
                let right_idx = curr_node.right.unwrap();
                curr_node = self.data.arena.get_mut(right_idx).unwrap();
            }
            curr_node.value = Some(curr_node.value.unwrap() + left_value);
        };

        parent = self.data.arena[idx].parent;

        let mut right_branch: Option<usize> = None;
        let mut right_visited: HashSet<usize> = HashSet::new();
        right_visited.insert(idx);
        while let Some(next_parent) = parent {
            let curr_node = &self.data.arena[next_parent];
            right_visited.insert(curr_node.idx);
            parent = curr_node.parent;
            if let Some(right_idx) = curr_node.right {
                if !right_visited.contains(&right_idx) {
                    right_branch = curr_node.right;
                    parent = None;
                }
            }
        }

        let right_idx_delete = self.data.arena[idx].right.unwrap();

        if let Some(right) = right_branch {
            let right_value = self.data.arena[right_idx_delete].value.unwrap();
            let mut curr_node = self.data.arena.get_mut(right).unwrap();
            while curr_node.value.is_none() {
                let left_idx = curr_node.left.unwrap();
                curr_node = self.data.arena.get_mut(left_idx).unwrap();
            }
            curr_node.value = Some(curr_node.value.unwrap() + right_value);
        };

        self.data.remove_node(left_idx_delete);
        self.data.remove_node(right_idx_delete);
        self.data.arena[idx].value = Some(0);
    }

    fn split(&mut self, idx: usize) {
        let curr_val = self.data.arena[idx].value.unwrap();
        let (new_left, new_right) = if curr_val % 2 == 0 {
            (curr_val / 2, curr_val / 2)
        } else {
            (curr_val / 2, curr_val / 2 + 1)
        };
        let next_depth = self.data.arena[idx].depth + 1;

        let left_new_node: BinaryNode<u8> = BinaryNode {
            idx: 0,
            value: Some(new_left),
            depth: next_depth,
            parent: Some(idx),
            left: None,
            right: None,
        };
        let new_left_idx = self.data.add_new_node(left_new_node);

        let right_new_node: BinaryNode<u8> = BinaryNode {
            idx: 0,
            value: Some(new_right),
            depth: next_depth,
            parent: Some(idx),
            left: None,
            right: None,
        };
        let new_right_idx = self.data.add_new_node(right_new_node);

        self.data.arena[idx].value = None;
        self.data.arena[idx].left = Some(new_left_idx);
        self.data.arena[idx].right = Some(new_right_idx);
    }
}

impl fmt::Display for SailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_number(&self.data, 0))
    }
}

fn print_number(subtree: &BinaryTree<u8>, node_idx: usize) -> String {
    let current_node = subtree.arena.get(node_idx).unwrap();
    match current_node.value {
        Some(val) => val.to_string(),
        None => {
            let left_str = print_number(subtree, current_node.left.unwrap());
            let right_str = print_number(subtree, current_node.right.unwrap());
            format!("[{},{}]", left_str, right_str)
        }
    }
}

// Updated function to update the indices of a subtree to put them in a dense
// tree arena. Returns the largest index for further updates
fn update_indices(subtree: &[BinaryNode<u8>], new_root: usize) -> Vec<BinaryNode<u8>> {
    let mut updated_data: Vec<BinaryNode<u8>> = Vec::new();
    let mut parent_update: HashMap<usize, usize> = HashMap::new();
    for node in subtree.iter() {
        let mut new_node: BinaryNode<u8> =
            BinaryNode::new(node.idx + new_root, node.value, node.depth + 1);

        new_node.parent = Some(*parent_update.get(&node.idx).unwrap_or(&0));

        if let Some(left_child) = node.left {
            new_node.left = Some(left_child + new_root);
            parent_update.insert(left_child, new_node.idx);
        };
        if let Some(right_child) = node.right {
            new_node.right = Some(right_child + new_root);
            parent_update.insert(right_child, new_node.idx);
        };

        updated_data.push(new_node);
    }

    updated_data
}

fn sailfish_component(input: &str) -> IResult<&str, SailfishNumber> {
    let val_parse: IResult<&str, &str> = digit1(input);
    match val_parse {
        Ok((rem_str, value)) => {
            let new_node: BinaryNode<u8> =
                BinaryNode::new(0, Some(value.parse::<u8>().unwrap()), 0);
            Ok((rem_str, SailfishNumber::new(vec![new_node], vec![])))
        }
        Err(_) => {
            let (rem_str, subtree) = sailfish_tree(input)?;
            Ok((rem_str, subtree))
        }
    }
}

fn sailfish_tree(input: &str) -> IResult<&str, SailfishNumber> {
    let (remain_str, (left, right)) = preceded(
        space0,
        delimited(
            tag("["),
            separated_pair(sailfish_component, tag(","), sailfish_component),
            tag("]"),
        ),
    )(input)?;

    Ok((remain_str, left.sum(&right)))
}

fn parse_numbers(input: &str) -> Vec<SailfishNumber> {
    let mut parsed_trees: Vec<SailfishNumber> = Vec::new();

    for line in input.lines() {
        let (_, new_number) = sailfish_tree(line).unwrap();
        parsed_trees.push(new_number);
    }

    parsed_trees
}

pub fn part1(input: &str) {
    // let (_, target_trench) = target(input).unwrap();
    // let start_v = target_trench.coolest_speed();
    // let max_height: i32 = start_v.1 * (start_v.1 + 1i32) / 2i32;
    //  println!("Maximum height for provided trench: {}", max_height);
}

pub fn part2(input: &str) {
    // let (_, target_trench) = target(input).unwrap();
    // let initial_velocities: HashSet<Point> = target_trench.compute_initial_velocities();
    // println!("Amount of initial velocities: {}", initial_velocities.len());
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use std::vec;

    use itertools::Itertools;

    use super::*;

    impl fmt::Debug for SailfishNumber {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "SailfishNumber {{")?;
            writeln!(f, "data: BinaryTree::new(")?;
            writeln!(f, "vec![")?;
            for entry in self.data.arena.iter() {
                writeln!(f, "{:?}", entry)?;
                writeln!(f, ",")?;
            }
            writeln!(f, "]),")?;
            writeln!(f, "deep_nodes: vec![")?;
            for deep in self.deep_nodes.iter() {
                writeln!(f, "{}, ", deep)?;
            }
            writeln!(f, "],")?;
            writeln!(f, "large_nodes: vec![")?;
            for large in self.large_nodes.iter() {
                writeln!(f, "{}, ", large)?;
            }
            writeln!(f, "],")?;
            writeln!(f, "}}")
        }
    }

    #[test]
    fn input_parsing() {
        let input_string = "[1,2]
        [[1,2],3]
        [9,[8,7]]
        [[1,9],[8,5]]
        [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
        [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
        [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

        let numbers = parse_numbers(input_string);

        println!("{:?}", numbers);

        /*
        // It's pretty dumb, but it's the ony way I know to do this test
        let ref_numbers: Vec<SailfishNumber> = vec![
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(2),
                    },
                    BinaryNode {
                        idx: 1,
                        value: Some(1),
                        depth: 1,
                        parent: Some(0),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(2),
                        depth: 1,
                        parent: Some(0),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(2),
                        right: Some(3),
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(1),
                        depth: 2,
                        parent: Some(1),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(2),
                        depth: 2,
                        parent: Some(1),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(3),
                        depth: 1,
                        parent: Some(0),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(2),
                    },
                    BinaryNode {
                        idx: 1,
                        value: Some(9),
                        depth: 1,
                        parent: Some(0),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(3),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(8),
                        depth: 2,
                        parent: Some(2),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(7),
                        depth: 2,
                        parent: Some(2),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(2),
                        right: Some(3),
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(1),
                        depth: 2,
                        parent: Some(1),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(9),
                        depth: 2,
                        parent: Some(1),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(5),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(8),
                        depth: 2,
                        parent: Some(4),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: Some(5),
                        depth: 2,
                        parent: Some(4),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(2),
                        right: Some(9),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 2,
                        parent: Some(1),
                        left: Some(3),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 3,
                        value: None,
                        depth: 3,
                        parent: Some(2),
                        left: Some(4),
                        right: Some(5),
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(1),
                        depth: 4,
                        parent: Some(3),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(2),
                        depth: 4,
                        parent: Some(3),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: None,
                        depth: 3,
                        parent: Some(2),
                        left: Some(7),
                        right: Some(8),
                    },
                    BinaryNode {
                        idx: 7,
                        value: Some(3),
                        depth: 4,
                        parent: Some(6),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 8,
                        value: Some(4),
                        depth: 4,
                        parent: Some(6),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 9,
                        value: None,
                        depth: 2,
                        parent: Some(1),
                        left: Some(10),
                        right: Some(13),
                    },
                    BinaryNode {
                        idx: 10,
                        value: None,
                        depth: 3,
                        parent: Some(9),
                        left: Some(11),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(5),
                        depth: 4,
                        parent: Some(10),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: Some(6),
                        depth: 4,
                        parent: Some(10),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        depth: 3,
                        parent: Some(9),
                        left: Some(14),
                        right: Some(15),
                    },
                    BinaryNode {
                        idx: 14,
                        value: Some(7),
                        depth: 4,
                        parent: Some(13),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(8),
                        depth: 4,
                        parent: Some(13),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: Some(9),
                        depth: 1,
                        parent: Some(0),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(2),
                        right: Some(7),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 2,
                        parent: Some(1),
                        left: Some(3),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(9),
                        depth: 3,
                        parent: Some(2),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: None,
                        depth: 3,
                        parent: Some(2),
                        left: Some(5),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(3),
                        depth: 4,
                        parent: Some(4),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: Some(8),
                        depth: 4,
                        parent: Some(4),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 7,
                        value: None,
                        depth: 2,
                        parent: Some(1),
                        left: Some(8),
                        right: Some(11),
                    },
                    BinaryNode {
                        idx: 8,
                        value: None,
                        depth: 3,
                        parent: Some(7),
                        left: Some(9),
                        right: Some(10),
                    },
                    BinaryNode {
                        idx: 9,
                        value: Some(0),
                        depth: 4,
                        parent: Some(8),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 10,
                        value: Some(9),
                        depth: 4,
                        parent: Some(8),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(6),
                        depth: 3,
                        parent: Some(7),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(13),
                        right: Some(20),
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        depth: 2,
                        parent: Some(12),
                        left: Some(14),
                        right: Some(17),
                    },
                    BinaryNode {
                        idx: 14,
                        value: None,
                        depth: 3,
                        parent: Some(13),
                        left: Some(15),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(3),
                        depth: 4,
                        parent: Some(14),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: Some(7),
                        depth: 4,
                        parent: Some(14),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 17,
                        value: None,
                        depth: 3,
                        parent: Some(13),
                        left: Some(18),
                        right: Some(19),
                    },
                    BinaryNode {
                        idx: 18,
                        value: Some(4),
                        depth: 4,
                        parent: Some(17),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 19,
                        value: Some(9),
                        depth: 4,
                        parent: Some(17),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 20,
                        value: Some(3),
                        depth: 2,
                        parent: Some(12),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
            SailfishNumber {
                data: BinaryTree::new(vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(2),
                        right: Some(9),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 2,
                        parent: Some(1),
                        left: Some(3),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 3,
                        value: None,
                        depth: 3,
                        parent: Some(2),
                        left: Some(4),
                        right: Some(5),
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(1),
                        depth: 4,
                        parent: Some(3),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(3),
                        depth: 4,
                        parent: Some(3),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: None,
                        depth: 3,
                        parent: Some(2),
                        left: Some(7),
                        right: Some(8),
                    },
                    BinaryNode {
                        idx: 7,
                        value: Some(5),
                        depth: 4,
                        parent: Some(6),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 8,
                        value: Some(3),
                        depth: 4,
                        parent: Some(6),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 9,
                        value: None,
                        depth: 2,
                        parent: Some(1),
                        left: Some(10),
                        right: Some(13),
                    },
                    BinaryNode {
                        idx: 10,
                        value: None,
                        depth: 3,
                        parent: Some(9),
                        left: Some(11),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(1),
                        depth: 4,
                        parent: Some(10),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: Some(3),
                        depth: 4,
                        parent: Some(10),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        depth: 3,
                        parent: Some(9),
                        left: Some(14),
                        right: Some(15),
                    },
                    BinaryNode {
                        idx: 14,
                        value: Some(8),
                        depth: 4,
                        parent: Some(13),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(7),
                        depth: 4,
                        parent: Some(13),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: None,
                        depth: 1,
                        parent: Some(0),
                        left: Some(17),
                        right: Some(24),
                    },
                    BinaryNode {
                        idx: 17,
                        value: None,
                        depth: 2,
                        parent: Some(16),
                        left: Some(18),
                        right: Some(21),
                    },
                    BinaryNode {
                        idx: 18,
                        value: None,
                        depth: 3,
                        parent: Some(17),
                        left: Some(19),
                        right: Some(20),
                    },
                    BinaryNode {
                        idx: 19,
                        value: Some(4),
                        depth: 4,
                        parent: Some(18),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 20,
                        value: Some(9),
                        depth: 4,
                        parent: Some(18),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 21,
                        value: None,
                        depth: 3,
                        parent: Some(17),
                        left: Some(22),
                        right: Some(23),
                    },
                    BinaryNode {
                        idx: 22,
                        value: Some(6),
                        depth: 4,
                        parent: Some(21),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 23,
                        value: Some(9),
                        depth: 4,
                        parent: Some(21),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 24,
                        value: None,
                        depth: 2,
                        parent: Some(16),
                        left: Some(25),
                        right: Some(28),
                    },
                    BinaryNode {
                        idx: 25,
                        value: None,
                        depth: 3,
                        parent: Some(24),
                        left: Some(26),
                        right: Some(27),
                    },
                    BinaryNode {
                        idx: 26,
                        value: Some(8),
                        depth: 4,
                        parent: Some(25),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 27,
                        value: Some(2),
                        depth: 4,
                        parent: Some(25),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 28,
                        value: None,
                        depth: 3,
                        parent: Some(24),
                        left: Some(29),
                        right: Some(30),
                    },
                    BinaryNode {
                        idx: 29,
                        value: Some(7),
                        depth: 4,
                        parent: Some(28),
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 30,
                        value: Some(3),
                        depth: 4,
                        parent: Some(28),
                        left: None,
                        right: None,
                    },
                ]),
                deep_nodes: vec![],
                large_nodes: vec![],
            },
        ];
        */

        // assert_eq!(numbers, ref_numbers);
    }

    #[test]
    fn explosions() {
        let input_string = "[[[[[9,8],1],2],3],4]
        [7,[6,[5,[4,[3,2]]]]]
        [[6,[5,[4,[3,2]]]],1]
        [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";

        let mut numbers = parse_numbers(input_string);
        let reduced_numbers: Vec<String> = numbers
            .iter_mut()
            .map(|num| {
                num.reduce();
                print_number(&num.data, 0)
            })
            .collect_vec();

        let ref_reductions = vec![
            "[[[[0,9],2],3],4]",
            "[7,[6,[5,[7,0]]]]",
            "[[6,[5,[7,0]]],3]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ];

        assert_eq!(reduced_numbers, ref_reductions);
    }

    #[test]
    fn full_reduction() {
        let input_string = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";

        let mut numbers = parse_numbers(input_string);
        let reduced_numbers: Vec<String> = numbers
            .iter_mut()
            .map(|num| {
                num.reduce();
                print_number(&num.data, 0)
            })
            .collect_vec();

        let ref_reductions = vec!["[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"];

        assert_eq!(reduced_numbers, ref_reductions);
    }

    #[test]
    fn simple_sums() {
        let input_strings = vec![
            "[1,1]
             [2,2]
             [3,3]
             [4,4]",
            "[1,1]
             [2,2]
             [3,3]
             [4,4]
             [5,5]",
            "[1,1]
             [2,2]
             [3,3]
             [4,4]
             [5,5]
             [6,6]",
        ];

        let mut reduced_numbers: Vec<String> = Vec::new();

        for group in input_strings.into_iter().map(parse_numbers) {
            let mut total = group[0].sum(&group[1]);
            total.reduce();
            for num in group.iter().skip(2) {
                total = total.sum(num);
                total.reduce();
            }

            reduced_numbers.push(total.to_string());
        }

        let reference_results = vec![
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        ];

        assert_eq!(reduced_numbers, reference_results);
    }

    #[test]
    fn longer_sum() {
        let input_string = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]";

        let mut ref_steps = vec![
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ]
        .into_iter();
        let numbers = parse_numbers(input_string);
        let mut total = numbers[0].sum(&numbers[1]);
        total.reduce();
        assert_eq!(total.to_string(), ref_steps.next().unwrap());
        for num in numbers.iter().skip(2) {
            println!("Current sum: {}", total);
            total = total.sum(num);
            total.reduce();
            assert_eq!(total.to_string(), ref_steps.next().unwrap());
        }
    }
}
