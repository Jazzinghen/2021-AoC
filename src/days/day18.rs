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

// Updated function to update the indices of a subtree to put them in a dense
// tree arena. Returns the largest index for further updates
fn update_indices(subtree: &mut [BinaryNode<u8>], new_root: usize) -> usize {
    for node in subtree.iter_mut() {
        node.idx += new_root;
        node.left = node.left.map(|idx| idx + new_root);
        node.right = node.right.map(|idx| idx + new_root);
    }

    subtree.last().unwrap().idx
}

fn sailfish_component(input: &str) -> IResult<&str, Vec<BinaryNode<u8>>> {
    let val_parse: IResult<&str, &str> = digit1(input);
    match val_parse {
        Ok((rem_str, value)) => {
            let new_node: BinaryNode<u8> = BinaryNode::new(0, Some(value.parse::<u8>().unwrap()));
            Ok((rem_str, vec![new_node]))
        }
        Err(_) => {
            let (rem_str, subtree) = sailfish_tree(input)?;
            Ok((rem_str, subtree))
        }
    }
}

fn sailfish_tree(input: &str) -> IResult<&str, Vec<BinaryNode<u8>>> {
    let (remain_str, (mut left, mut right)) = preceded(
        space0,
        delimited(
            tag("["),
            separated_pair(sailfish_component, tag(","), sailfish_component),
            tag("]"),
        ),
    )(input)?;

    let left_max = update_indices(&mut left[..], 1);
    let _ = update_indices(&mut right[..], left_max + 1);

    let mut new_node: BinaryNode<u8> = BinaryNode::new(0, None);
    new_node.left = Some(1);
    new_node.right = Some(left_max + 1);

    let mut tree: Vec<BinaryNode<u8>> = vec![new_node];

    tree.extend(left.into_iter());
    tree.extend(right.into_iter());

    Ok((remain_str, tree))
}

fn parse_trees(input: &str) -> Vec<BinaryTree<u8>> {
    let mut parsed_trees: Vec<BinaryTree<u8>> = Vec::new();

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

        let ref_numbers: Vec<BinaryTree<u8>> = vec![
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(2),
                    },
                    BinaryNode {
                        idx: 1,
                        value: Some(1),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(2),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        parent: None,
                        left: Some(2),
                        right: Some(3),
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(1),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(2),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(2),
                    },
                    BinaryNode {
                        idx: 1,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        parent: None,
                        left: Some(3),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(8),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(7),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        parent: None,
                        left: Some(2),
                        right: Some(3),
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(1),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: None,
                        parent: None,
                        left: Some(5),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(8),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: Some(5),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        parent: None,
                        left: Some(2),
                        right: Some(9),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        parent: None,
                        left: Some(3),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 3,
                        value: None,
                        parent: None,
                        left: Some(4),
                        right: Some(5),
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(1),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(2),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: None,
                        parent: None,
                        left: Some(7),
                        right: Some(8),
                    },
                    BinaryNode {
                        idx: 7,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 8,
                        value: Some(4),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 9,
                        value: None,
                        parent: None,
                        left: Some(10),
                        right: Some(13),
                    },
                    BinaryNode {
                        idx: 10,
                        value: None,
                        parent: None,
                        left: Some(11),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(5),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: Some(6),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        parent: None,
                        left: Some(14),
                        right: Some(15),
                    },
                    BinaryNode {
                        idx: 14,
                        value: Some(7),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(8),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        parent: None,
                        left: Some(2),
                        right: Some(7),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        parent: None,
                        left: Some(3),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: None,
                        parent: None,
                        left: Some(5),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: Some(8),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 7,
                        value: None,
                        parent: None,
                        left: Some(8),
                        right: Some(11),
                    },
                    BinaryNode {
                        idx: 8,
                        value: None,
                        parent: None,
                        left: Some(9),
                        right: Some(10),
                    },
                    BinaryNode {
                        idx: 9,
                        value: Some(0),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 10,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(6),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: None,
                        parent: None,
                        left: Some(13),
                        right: Some(20),
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        parent: None,
                        left: Some(14),
                        right: Some(17),
                    },
                    BinaryNode {
                        idx: 14,
                        value: None,
                        parent: None,
                        left: Some(15),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: Some(7),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 17,
                        value: None,
                        parent: None,
                        left: Some(18),
                        right: Some(19),
                    },
                    BinaryNode {
                        idx: 18,
                        value: Some(4),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 19,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 20,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
            BinaryTree {
                arena: vec![
                    BinaryNode {
                        idx: 0,
                        value: None,
                        parent: None,
                        left: Some(1),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        parent: None,
                        left: Some(2),
                        right: Some(9),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        parent: None,
                        left: Some(3),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 3,
                        value: None,
                        parent: None,
                        left: Some(4),
                        right: Some(5),
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(1),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: None,
                        parent: None,
                        left: Some(7),
                        right: Some(8),
                    },
                    BinaryNode {
                        idx: 7,
                        value: Some(5),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 8,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 9,
                        value: None,
                        parent: None,
                        left: Some(10),
                        right: Some(13),
                    },
                    BinaryNode {
                        idx: 10,
                        value: None,
                        parent: None,
                        left: Some(11),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(1),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        parent: None,
                        left: Some(14),
                        right: Some(15),
                    },
                    BinaryNode {
                        idx: 14,
                        value: Some(8),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(7),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: None,
                        parent: None,
                        left: Some(17),
                        right: Some(24),
                    },
                    BinaryNode {
                        idx: 17,
                        value: None,
                        parent: None,
                        left: Some(18),
                        right: Some(21),
                    },
                    BinaryNode {
                        idx: 18,
                        value: None,
                        parent: None,
                        left: Some(19),
                        right: Some(20),
                    },
                    BinaryNode {
                        idx: 19,
                        value: Some(4),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 20,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 21,
                        value: None,
                        parent: None,
                        left: Some(22),
                        right: Some(23),
                    },
                    BinaryNode {
                        idx: 22,
                        value: Some(6),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 23,
                        value: Some(9),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 24,
                        value: None,
                        parent: None,
                        left: Some(25),
                        right: Some(28),
                    },
                    BinaryNode {
                        idx: 25,
                        value: None,
                        parent: None,
                        left: Some(26),
                        right: Some(27),
                    },
                    BinaryNode {
                        idx: 26,
                        value: Some(8),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 27,
                        value: Some(2),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 28,
                        value: None,
                        parent: None,
                        left: Some(29),
                        right: Some(30),
                    },
                    BinaryNode {
                        idx: 29,
                        value: Some(7),
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 30,
                        value: Some(3),
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
        ];

        //assert_eq!(numbers, ref_numbers);

        println!("{:?}", numbers);
        panic!("Derp!");
    }
}
