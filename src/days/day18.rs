use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

use crate::aoc_lib::jazz_data::{BinaryNode, BinaryTree};

enum NodePosition {
    Left,
    Right,
}

fn update_indexes(
    subtree: HashMap<usize, BinaryNode<Option<u8>>>,
    new_position: NodePosition,
) -> HashMap<usize, BinaryNode<Option<u8>>> {
    let mut new_subtree: HashMap<usize, BinaryNode<Option<u8>>> = HashMap::new();
    let mut update_queue: VecDeque<(usize, usize)> = VecDeque::new();

    let first_step: (usize, usize) = match new_position {
        NodePosition::Left => (0, 1),
        NodePosition::Right => (0, 2),
    };

    update_queue.push_back(first_step);

    while let Some((current_idx, new_idx)) = update_queue.pop_front() {
        let previous_node = subtree.get(&current_idx).unwrap();
        let mut new_node: BinaryNode<Option<u8>> = BinaryNode::new(new_idx, previous_node.value);
        let new_left_idx = new_idx * 2 + 1;
        let new_right_idx = new_idx * 2 + 2;
        if let Some(left_idx) = previous_node.left {
            update_queue.push_back((left_idx, new_left_idx));
            new_node.left = Some(new_left_idx);
        }
        if let Some(right_idx) = previous_node.right {
            update_queue.push_back((right_idx, new_right_idx));
            new_node.right = Some(new_right_idx);
        }

        new_subtree.insert(new_idx, new_node);
    }

    new_subtree
}

fn sailfish_component(input: &str) -> IResult<&str, HashMap<usize, BinaryNode<Option<u8>>>> {
    let val_parse: IResult<&str, &str> = digit1(input);
    match val_parse {
        Ok((rem_str, value)) => {
            let new_node: BinaryNode<Option<u8>> =
                BinaryNode::new(0, Some(value.parse::<u8>().unwrap()));
            let mut subtree: HashMap<usize, BinaryNode<Option<u8>>> = HashMap::new();
            subtree.insert(0, new_node);
            Ok((rem_str, subtree))
        }
        Err(_) => {
            let (rem_str, subtree) = sailfish_tree(input)?;
            Ok((rem_str, subtree))
        }
    }
}

fn sailfish_tree(input: &str) -> IResult<&str, HashMap<usize, BinaryNode<Option<u8>>>> {
    let (remain_str, (left, right)) = preceded(
        space0,
        delimited(
            tag("["),
            separated_pair(sailfish_component, tag(","), sailfish_component),
            tag("]"),
        ),
    )(input)?;

    let mut new_node: BinaryNode<Option<u8>> = BinaryNode::new(0, None);
    new_node.left = Some(1);
    new_node.right = Some(2);

    let mut tree: HashMap<usize, BinaryNode<Option<u8>>> = HashMap::new();
    tree.insert(0, new_node);

    tree.extend(update_indexes(left, NodePosition::Left).into_iter());
    tree.extend(update_indexes(right, NodePosition::Right).into_iter());

    Ok((remain_str, tree))
}

fn parse_trees(input: &str) -> Vec<BinaryTree<Option<u8>>> {
    let mut parsed_trees: Vec<BinaryTree<Option<u8>>> = Vec::new();

    for line in input.lines() {
        let (_, parsed_value) = sailfish_tree(line).unwrap();
        let new_tree = BinaryTree {
            arena: parsed_value,
        };
        parsed_trees.push(new_tree);
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
    use super::*;

    #[test]
    fn input_parsing() {
        let input_string = "[1,2]
        [[1,2],3]
        [9,[8,7]]
        [[1,9],[8,5]]
        [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
        [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
        [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

        let numbers = parse_trees(input_string);

        let ref_numbers: Vec<BinaryTree<Option<u8>>> = vec![
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: Some(1),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: Some(2),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: None,
                            parent: None,
                            left: Some(3),
                            right: Some(4),
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        3,
                        BinaryNode {
                            idx: 3,
                            value: Some(1),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        4,
                        BinaryNode {
                            idx: 4,
                            value: Some(2),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: None,
                            parent: None,
                            left: Some(5),
                            right: Some(6),
                        },
                    ),
                    (
                        5,
                        BinaryNode {
                            idx: 5,
                            value: Some(8),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        6,
                        BinaryNode {
                            idx: 6,
                            value: Some(7),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: None,
                            parent: None,
                            left: Some(3),
                            right: Some(4),
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: None,
                            parent: None,
                            left: Some(5),
                            right: Some(6),
                        },
                    ),
                    (
                        3,
                        BinaryNode {
                            idx: 3,
                            value: Some(1),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        4,
                        BinaryNode {
                            idx: 4,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        5,
                        BinaryNode {
                            idx: 5,
                            value: Some(8),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        6,
                        BinaryNode {
                            idx: 6,
                            value: Some(5),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: None,
                            parent: None,
                            left: Some(3),
                            right: Some(4),
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        3,
                        BinaryNode {
                            idx: 3,
                            value: None,
                            parent: None,
                            left: Some(7),
                            right: Some(8),
                        },
                    ),
                    (
                        4,
                        BinaryNode {
                            idx: 4,
                            value: None,
                            parent: None,
                            left: Some(9),
                            right: Some(10),
                        },
                    ),
                    (
                        7,
                        BinaryNode {
                            idx: 7,
                            value: None,
                            parent: None,
                            left: Some(15),
                            right: Some(16),
                        },
                    ),
                    (
                        8,
                        BinaryNode {
                            idx: 8,
                            value: None,
                            parent: None,
                            left: Some(17),
                            right: Some(18),
                        },
                    ),
                    (
                        9,
                        BinaryNode {
                            idx: 9,
                            value: None,
                            parent: None,
                            left: Some(19),
                            right: Some(20),
                        },
                    ),
                    (
                        10,
                        BinaryNode {
                            idx: 10,
                            value: None,
                            parent: None,
                            left: Some(21),
                            right: Some(22),
                        },
                    ),
                    (
                        15,
                        BinaryNode {
                            idx: 15,
                            value: Some(1),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        16,
                        BinaryNode {
                            idx: 16,
                            value: Some(2),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        17,
                        BinaryNode {
                            idx: 17,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        18,
                        BinaryNode {
                            idx: 18,
                            value: Some(4),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        19,
                        BinaryNode {
                            idx: 19,
                            value: Some(5),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        20,
                        BinaryNode {
                            idx: 20,
                            value: Some(6),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        21,
                        BinaryNode {
                            idx: 21,
                            value: Some(7),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        22,
                        BinaryNode {
                            idx: 22,
                            value: Some(8),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: None,
                            parent: None,
                            left: Some(3),
                            right: Some(4),
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: None,
                            parent: None,
                            left: Some(5),
                            right: Some(6),
                        },
                    ),
                    (
                        3,
                        BinaryNode {
                            idx: 3,
                            value: None,
                            parent: None,
                            left: Some(7),
                            right: Some(8),
                        },
                    ),
                    (
                        4,
                        BinaryNode {
                            idx: 4,
                            value: None,
                            parent: None,
                            left: Some(9),
                            right: Some(10),
                        },
                    ),
                    (
                        5,
                        BinaryNode {
                            idx: 5,
                            value: None,
                            parent: None,
                            left: Some(11),
                            right: Some(12),
                        },
                    ),
                    (
                        6,
                        BinaryNode {
                            idx: 6,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        7,
                        BinaryNode {
                            idx: 7,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        8,
                        BinaryNode {
                            idx: 8,
                            value: None,
                            parent: None,
                            left: Some(17),
                            right: Some(18),
                        },
                    ),
                    (
                        9,
                        BinaryNode {
                            idx: 9,
                            value: None,
                            parent: None,
                            left: Some(19),
                            right: Some(20),
                        },
                    ),
                    (
                        10,
                        BinaryNode {
                            idx: 10,
                            value: Some(6),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        11,
                        BinaryNode {
                            idx: 11,
                            value: None,
                            parent: None,
                            left: Some(23),
                            right: Some(24),
                        },
                    ),
                    (
                        12,
                        BinaryNode {
                            idx: 12,
                            value: None,
                            parent: None,
                            left: Some(25),
                            right: Some(26),
                        },
                    ),
                    (
                        17,
                        BinaryNode {
                            idx: 17,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        18,
                        BinaryNode {
                            idx: 18,
                            value: Some(8),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        19,
                        BinaryNode {
                            idx: 19,
                            value: Some(0),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        20,
                        BinaryNode {
                            idx: 20,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        23,
                        BinaryNode {
                            idx: 23,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        24,
                        BinaryNode {
                            idx: 24,
                            value: Some(7),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        25,
                        BinaryNode {
                            idx: 25,
                            value: Some(4),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        26,
                        BinaryNode {
                            idx: 26,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
            BinaryTree {
                arena: HashMap::from([
                    (
                        0,
                        BinaryNode {
                            idx: 0,
                            value: None,
                            parent: None,
                            left: Some(1),
                            right: Some(2),
                        },
                    ),
                    (
                        1,
                        BinaryNode {
                            idx: 1,
                            value: None,
                            parent: None,
                            left: Some(3),
                            right: Some(4),
                        },
                    ),
                    (
                        2,
                        BinaryNode {
                            idx: 2,
                            value: None,
                            parent: None,
                            left: Some(5),
                            right: Some(6),
                        },
                    ),
                    (
                        3,
                        BinaryNode {
                            idx: 3,
                            value: None,
                            parent: None,
                            left: Some(7),
                            right: Some(8),
                        },
                    ),
                    (
                        4,
                        BinaryNode {
                            idx: 4,
                            value: None,
                            parent: None,
                            left: Some(9),
                            right: Some(10),
                        },
                    ),
                    (
                        5,
                        BinaryNode {
                            idx: 5,
                            value: None,
                            parent: None,
                            left: Some(11),
                            right: Some(12),
                        },
                    ),
                    (
                        6,
                        BinaryNode {
                            idx: 6,
                            value: None,
                            parent: None,
                            left: Some(13),
                            right: Some(14),
                        },
                    ),
                    (
                        7,
                        BinaryNode {
                            idx: 7,
                            value: None,
                            parent: None,
                            left: Some(15),
                            right: Some(16),
                        },
                    ),
                    (
                        8,
                        BinaryNode {
                            idx: 8,
                            value: None,
                            parent: None,
                            left: Some(17),
                            right: Some(18),
                        },
                    ),
                    (
                        9,
                        BinaryNode {
                            idx: 9,
                            value: None,
                            parent: None,
                            left: Some(19),
                            right: Some(20),
                        },
                    ),
                    (
                        10,
                        BinaryNode {
                            idx: 10,
                            value: None,
                            parent: None,
                            left: Some(21),
                            right: Some(22),
                        },
                    ),
                    (
                        11,
                        BinaryNode {
                            idx: 11,
                            value: None,
                            parent: None,
                            left: Some(23),
                            right: Some(24),
                        },
                    ),
                    (
                        12,
                        BinaryNode {
                            idx: 12,
                            value: None,
                            parent: None,
                            left: Some(25),
                            right: Some(26),
                        },
                    ),
                    (
                        13,
                        BinaryNode {
                            idx: 13,
                            value: None,
                            parent: None,
                            left: Some(27),
                            right: Some(28),
                        },
                    ),
                    (
                        14,
                        BinaryNode {
                            idx: 14,
                            value: None,
                            parent: None,
                            left: Some(29),
                            right: Some(30),
                        },
                    ),
                    (
                        15,
                        BinaryNode {
                            idx: 15,
                            value: Some(1),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        16,
                        BinaryNode {
                            idx: 16,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        17,
                        BinaryNode {
                            idx: 17,
                            value: Some(5),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        18,
                        BinaryNode {
                            idx: 18,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        19,
                        BinaryNode {
                            idx: 19,
                            value: Some(1),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        20,
                        BinaryNode {
                            idx: 20,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        21,
                        BinaryNode {
                            idx: 21,
                            value: Some(8),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        22,
                        BinaryNode {
                            idx: 22,
                            value: Some(7),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        23,
                        BinaryNode {
                            idx: 23,
                            value: Some(4),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        24,
                        BinaryNode {
                            idx: 24,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        25,
                        BinaryNode {
                            idx: 25,
                            value: Some(6),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        26,
                        BinaryNode {
                            idx: 26,
                            value: Some(9),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        27,
                        BinaryNode {
                            idx: 27,
                            value: Some(8),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        28,
                        BinaryNode {
                            idx: 28,
                            value: Some(2),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        29,
                        BinaryNode {
                            idx: 29,
                            value: Some(7),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                    (
                        30,
                        BinaryNode {
                            idx: 30,
                            value: Some(3),
                            parent: None,
                            left: None,
                            right: None,
                        },
                    ),
                ]),
            },
        ];

        assert_eq!(numbers, ref_numbers);
    }
}
