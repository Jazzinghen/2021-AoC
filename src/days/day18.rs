use core::fmt::{Debug, Formatter, Result};

use hashbrown::{HashMap, HashSet};

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
    pub fn new(arena: Vec<BinaryNode<u8>>) -> Self {
        let deep_nodes: Vec<usize> = arena
            .iter()
            .filter(|node| node.depth >= 4 && node.value.is_none())
            .map(|node| node.idx)
            .rev()
            .collect();
        let large_nodes: Vec<usize> = arena
            .iter()
            .filter(|node| {
                if let Some(val) = node.value {
                    val > 9
                } else {
                    false
                }
            })
            .map(|node| node.idx)
            .rev()
            .collect();
        Self {
            data: BinaryTree::<u8>::new(arena),
            deep_nodes,
            large_nodes,
        }
    }

    pub fn sum(&self, other: &Self) -> Self {
        let updated_left = update_indices(&self.data.arena[..], 1);
        let left_max = updated_left.last().unwrap().idx;
        let updated_right = update_indices(&other.data.arena[..], left_max + 1);

        let mut new_node: BinaryNode<u8> = BinaryNode::new(0, None, 0);
        new_node.left = Some(1);
        new_node.right = Some(left_max + 1);

        let mut arena: Vec<BinaryNode<u8>> = vec![new_node];

        arena.extend(updated_left.into_iter());
        arena.extend(updated_right.into_iter());

        Self::new(arena)
    }

    pub fn reduce(&mut self) {
        while !self.deep_nodes.is_empty() || !self.large_nodes.is_empty() {
            if let Some(deep) = self.deep_nodes.pop() {
                let large_results = self.explode(deep);
                self.large_nodes.extend(large_results.into_iter());
            } else if let Some(large) = self.large_nodes.pop() {
                self.split(large);
                if self.data.arena[large].depth >= 4 {
                    self.deep_nodes.push(large);
                }
            }
        }
    }

    fn explode(&mut self, idx: usize) -> Vec<usize> {
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

        let mut large_nodes: Vec<usize> = Vec::new();

        let left_idx_delete = self.data.arena[idx].left.unwrap();
        let left_value = self.data.arena[left_idx_delete].value.unwrap();
        if let Some(left) = left_branch {
            let mut curr_node = self.data.arena.get_mut(left).unwrap();
            while curr_node.value.is_none() {
                let right_idx = curr_node.right.unwrap();
                curr_node = self.data.arena.get_mut(right_idx).unwrap();
            }
            curr_node.value = Some(curr_node.value.unwrap() + left_value);

            if curr_node.value.unwrap() > 9 {
                large_nodes.push(curr_node.idx);
            }
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
        let right_value = self.data.arena[right_idx_delete].value.unwrap();

        if let Some(right) = right_branch {
            let mut curr_node = self.data.arena.get_mut(right).unwrap();
            while curr_node.value.is_none() {
                let left_idx = curr_node.left.unwrap();
                curr_node = self.data.arena.get_mut(left_idx).unwrap();
            }
            curr_node.value = Some(curr_node.value.unwrap() + right_value);
            if curr_node.value.unwrap() > 9 {
                large_nodes.push(curr_node.idx);
            }
        };

        self.data.remove_node(left_idx_delete);
        self.data.remove_node(right_idx_delete);
        self.data.arena[idx].value = Some(0);

        large_nodes
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

// Updated function to update the indices of a subtree to put them in a dense
// tree arena. Returns the largest index for further updates
fn update_indices(subtree: &[BinaryNode<u8>], new_root: usize) -> Vec<BinaryNode<u8>> {
    let mut updated_data: Vec<BinaryNode<u8>> = Vec::new();
    let mut parent_update: HashMap<usize, usize> = HashMap::new();
    for node in subtree.iter() {
        let mut new_node: BinaryNode<u8> =
            BinaryNode::new(node.idx + new_root, node.value, node.depth + 1);

        new_node.parent = Some(*parent_update.get(&node.idx).unwrap_or(&0));

        new_node.left = node.left.map(|idx| idx + new_root);
        new_node.right = node.right.map(|idx| idx + new_root);

        if let Some(left_child) = node.left {
            parent_update.insert(left_child, new_node.idx);
        };
        if let Some(right_child) = node.right {
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
            Ok((rem_str, SailfishNumber::new(vec![new_node])))
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
    use std::vec;

    use itertools::Itertools;

    use super::*;

    impl Debug for SailfishNumber {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

        assert_eq!(numbers, ref_numbers);
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
}
