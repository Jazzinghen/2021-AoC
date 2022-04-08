use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

use crate::aoc_lib::jazz_data::{BinaryNode, BinaryTree};

// Updated function to update the indices of a subtree to put them in a dense
// tree arena. Returns the largest index for further updates
fn update_indices(subtree: &[BinaryNode<u8>], new_root: usize) -> Vec<BinaryNode<u8>> {
    let mut updated_data: Vec<BinaryNode<u8>> = Vec::new();
    for node in subtree.iter() {
        let mut new_node: BinaryNode<u8> =
            BinaryNode::new(node.idx + new_root, node.value, node.depth + 1);

        new_node.left = node.left.map(|idx| idx + new_root);
        new_node.right = node.right.map(|idx| idx + new_root);
        updated_data.push(new_node);
    }

    updated_data
}

fn sailfish_component(input: &str) -> IResult<&str, BinaryTree<u8>> {
    let val_parse: IResult<&str, &str> = digit1(input);
    match val_parse {
        Ok((rem_str, value)) => {
            let new_node: BinaryNode<u8> =
                BinaryNode::new(0, Some(value.parse::<u8>().unwrap()), 0);
            Ok((
                rem_str,
                BinaryTree::<u8> {
                    arena: vec![new_node],
                },
            ))
        }
        Err(_) => {
            let (rem_str, subtree) = sailfish_tree(input)?;
            Ok((rem_str, subtree))
        }
    }
}

fn sailfish_tree(input: &str) -> IResult<&str, BinaryTree<u8>> {
    let (remain_str, (left, right)) = preceded(
        space0,
        delimited(
            tag("["),
            separated_pair(sailfish_component, tag(","), sailfish_component),
            tag("]"),
        ),
    )(input)?;

    Ok((remain_str, numbers_sum(&left, &right)))
}

fn parse_trees(input: &str) -> Vec<BinaryTree<u8>> {
    let mut parsed_trees: Vec<BinaryTree<u8>> = Vec::new();

    for line in input.lines() {
        let (_, new_tree) = sailfish_tree(line).unwrap();
        parsed_trees.push(new_tree);
    }

    parsed_trees
}

fn numbers_sum(left: &BinaryTree<u8>, right: &BinaryTree<u8>) -> BinaryTree<u8> {
    let updated_left = update_indices(&left.arena[..], 1);
    let left_max = updated_left.last().unwrap().idx;
    let updated_right = update_indices(&right.arena[..], left_max + 1);

    let mut new_node: BinaryNode<u8> = BinaryNode::new(0, None, 0);
    new_node.left = Some(1);
    new_node.right = Some(left_max + 1);

    let mut arena: Vec<BinaryNode<u8>> = vec![new_node];

    arena.extend(updated_left.into_iter());
    arena.extend(updated_right.into_iter());

    BinaryTree { arena }
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

        let numbers = parse_trees(input_string);

        // It's pretty dumb, but it's the ony way I know to do this test
        let ref_numbers: Vec<BinaryTree<u8>> = vec![
            BinaryTree {
                arena: vec![
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
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(2),
                        depth: 1,
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
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(2),
                        right: Some(3),
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(1),
                        depth: 2,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(2),
                        depth: 2,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(3),
                        depth: 1,
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
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(2),
                    },
                    BinaryNode {
                        idx: 1,
                        value: Some(9),
                        depth: 1,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(3),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(8),
                        depth: 2,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(7),
                        depth: 2,
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
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(2),
                        right: Some(3),
                    },
                    BinaryNode {
                        idx: 2,
                        value: Some(1),
                        depth: 2,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(9),
                        depth: 2,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(5),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(8),
                        depth: 2,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: Some(5),
                        depth: 2,
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
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(2),
                        right: Some(9),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(3),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 3,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(4),
                        right: Some(5),
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(1),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(2),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(7),
                        right: Some(8),
                    },
                    BinaryNode {
                        idx: 7,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 8,
                        value: Some(4),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 9,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(10),
                        right: Some(13),
                    },
                    BinaryNode {
                        idx: 10,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(11),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(5),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: Some(6),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(14),
                        right: Some(15),
                    },
                    BinaryNode {
                        idx: 14,
                        value: Some(7),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(8),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: Some(9),
                        depth: 1,
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
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(2),
                        right: Some(7),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(3),
                        right: Some(4),
                    },
                    BinaryNode {
                        idx: 3,
                        value: Some(9),
                        depth: 3,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 4,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(5),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: Some(8),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 7,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(8),
                        right: Some(11),
                    },
                    BinaryNode {
                        idx: 8,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(9),
                        right: Some(10),
                    },
                    BinaryNode {
                        idx: 9,
                        value: Some(0),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 10,
                        value: Some(9),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(6),
                        depth: 3,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(13),
                        right: Some(20),
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(14),
                        right: Some(17),
                    },
                    BinaryNode {
                        idx: 14,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(15),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: Some(7),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 17,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(18),
                        right: Some(19),
                    },
                    BinaryNode {
                        idx: 18,
                        value: Some(4),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 19,
                        value: Some(9),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 20,
                        value: Some(3),
                        depth: 2,
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
                        depth: 0,
                        parent: None,
                        left: Some(1),
                        right: Some(16),
                    },
                    BinaryNode {
                        idx: 1,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(2),
                        right: Some(9),
                    },
                    BinaryNode {
                        idx: 2,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(3),
                        right: Some(6),
                    },
                    BinaryNode {
                        idx: 3,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(4),
                        right: Some(5),
                    },
                    BinaryNode {
                        idx: 4,
                        value: Some(1),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 5,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 6,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(7),
                        right: Some(8),
                    },
                    BinaryNode {
                        idx: 7,
                        value: Some(5),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 8,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 9,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(10),
                        right: Some(13),
                    },
                    BinaryNode {
                        idx: 10,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(11),
                        right: Some(12),
                    },
                    BinaryNode {
                        idx: 11,
                        value: Some(1),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 12,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 13,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(14),
                        right: Some(15),
                    },
                    BinaryNode {
                        idx: 14,
                        value: Some(8),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 15,
                        value: Some(7),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 16,
                        value: None,
                        depth: 1,
                        parent: None,
                        left: Some(17),
                        right: Some(24),
                    },
                    BinaryNode {
                        idx: 17,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(18),
                        right: Some(21),
                    },
                    BinaryNode {
                        idx: 18,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(19),
                        right: Some(20),
                    },
                    BinaryNode {
                        idx: 19,
                        value: Some(4),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 20,
                        value: Some(9),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 21,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(22),
                        right: Some(23),
                    },
                    BinaryNode {
                        idx: 22,
                        value: Some(6),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 23,
                        value: Some(9),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 24,
                        value: None,
                        depth: 2,
                        parent: None,
                        left: Some(25),
                        right: Some(28),
                    },
                    BinaryNode {
                        idx: 25,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(26),
                        right: Some(27),
                    },
                    BinaryNode {
                        idx: 26,
                        value: Some(8),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 27,
                        value: Some(2),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 28,
                        value: None,
                        depth: 3,
                        parent: None,
                        left: Some(29),
                        right: Some(30),
                    },
                    BinaryNode {
                        idx: 29,
                        value: Some(7),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                    BinaryNode {
                        idx: 30,
                        value: Some(3),
                        depth: 4,
                        parent: None,
                        left: None,
                        right: None,
                    },
                ],
            },
        ];

        assert_eq!(numbers, ref_numbers);
    }
}
