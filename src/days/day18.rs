use indextree::{Arena, NodeId};
use nom::bytes::complete::tag;
use nom::character::complete::{space0, u8};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

type SailfishArena = Arena<Option<u8>>;

#[derive(Clone)]
enum TempRef {
    Value(u8),
    Node(Box<TempNode>),
}

#[derive(Clone)]
struct TempNode {
    left: TempRef,
    right: TempRef,
}

enum SailfishOperation {
    Explode(NodeId),
    Split(NodeId),
}

#[derive(PartialEq)]
struct SailfishNumber {
    data: Arena<Option<u8>>,
    deep_nodes: Vec<NodeId>,
    large_nodes: Vec<NodeId>,
}

fn sum(arena: &mut SailfishArena, lhs: NodeId, rhs: NodeId) -> NodeId {
    let new_root = arena.new_node(None);
    new_root.append(lhs, arena);
    new_root.append(rhs, arena);

    reduce(arena, new_root);

    new_root
}

fn find_reducible(arena: &mut SailfishArena, root: NodeId) -> Option<SailfishOperation> {
    let mut exploration_stack: Vec<(NodeId, usize)> = vec![(root, 0)];

    let mut leftmost_deep: Option<NodeId> = None;
    let mut leftmost_big: Option<NodeId> = None;

    while let Some((curr_idx, curr_depth)) = exploration_stack.pop() {
        let curr_node = arena.get(curr_idx).unwrap();

        if leftmost_deep.is_none() && curr_depth > 4 && curr_node.get().is_some() {
            leftmost_deep = Some(arena.get(curr_idx).unwrap().parent().unwrap());
        }

        if leftmost_deep.is_some() {
            exploration_stack.clear();
        } else if let Some(value) = curr_node.get() {
            assert!(
                curr_node.first_child().is_none(),
                "Value nodes should be leaves, not intermediate nodes!"
            );
            if leftmost_big.is_none() && *value > 9 {
                leftmost_big = Some(curr_idx);
            };
        } else {
            assert_eq!(
                curr_idx.children(arena).count(),
                2,
                "Somehow we don't have 2 children in a non-leaf node of a binary tree!"
            );

            exploration_stack.push((curr_node.last_child().unwrap(), curr_depth + 1));
            exploration_stack.push((curr_node.first_child().unwrap(), curr_depth + 1));
        }
    }

    if let Some(deep) = leftmost_deep {
        return Some(SailfishOperation::Explode(deep));
    }
    if let Some(big) = leftmost_big {
        return Some(SailfishOperation::Split(big));
    }

    None
}

fn reduce(arena: &mut SailfishArena, root: NodeId) {
    while let Some(next_reduction) = find_reducible(arena, root) {
        match next_reduction {
            SailfishOperation::Explode(node) => explode(arena, node),
            SailfishOperation::Split(node) => split(arena, node),
        };
    }
}

fn explode(arena: &mut SailfishArena, deep_node: NodeId) {
    let start_parent = arena.get(deep_node).unwrap().parent();
    let mut parent = start_parent;
    let mut prev_node = deep_node;
    let mut left_branch: Option<NodeId> = None;

    while left_branch.is_none() && parent.is_some() {
        let curr_node = arena.get(parent.unwrap()).unwrap();
        let left_node = curr_node.first_child().unwrap();
        if left_node != prev_node {
            left_branch = Some(left_node);
        } else {
            prev_node = parent.unwrap();
            parent = curr_node.parent();
        }
    }

    if let Some(left) = left_branch {
        let left_id = arena.get(deep_node).unwrap().first_child().unwrap();
        let left_val = arena.get(left_id).unwrap().get().unwrap();

        let mut curr_node = arena.get_mut(left).unwrap();
        while curr_node.get().is_none() {
            let curr_idx = curr_node.last_child().unwrap();
            curr_node = arena.get_mut(curr_idx).unwrap();
        }
        let new_val = curr_node.get().unwrap() + left_val;
        let _ = curr_node.get_mut().insert(new_val);
    };

    parent = start_parent;

    let mut right_branch: Option<NodeId> = None;

    while right_branch.is_none() && parent.is_some() {
        let curr_node = arena.get(parent.unwrap()).unwrap();
        let right_node = curr_node.last_child().unwrap();
        if right_node != prev_node {
            right_branch = Some(right_node);
        } else {
            prev_node = parent.unwrap();
            parent = curr_node.parent();
        }
    }

    if let Some(right) = right_branch {
        let right_id = arena.get(deep_node).unwrap().last_child().unwrap();
        let right_val = arena.get(right_id).unwrap().get().unwrap();

        let mut curr_node = arena.get_mut(right).unwrap();
        while curr_node.get().is_none() {
            let curr_idx = curr_node.first_child().unwrap();
            curr_node = arena.get_mut(curr_idx).unwrap();
        }
        let new_val = curr_node.get().unwrap() + right_val;
        let _ = curr_node.get_mut().insert(new_val);
    };

    #[allow(clippy::needless_collect)]
    let children: Vec<NodeId> = deep_node.children(arena).collect();
    for child in children.into_iter() {
        child.remove_subtree(arena);
    }
    let _ = arena.get_mut(deep_node).unwrap().get_mut().insert(0);
}

fn split(arena: &mut SailfishArena, big_node: NodeId) {
    let curr_val = arena.get_mut(big_node).unwrap().get_mut().take().unwrap();
    let (new_left, new_right) = if curr_val % 2 == 0 {
        (curr_val / 2, curr_val / 2)
    } else {
        (curr_val / 2, curr_val / 2 + 1)
    };

    let new_left_node = arena.new_node(Some(new_left));
    let new_right_node = arena.new_node(Some(new_right));

    big_node.append(new_left_node, arena);
    big_node.append(new_right_node, arena);
}

fn compute_magnitude(arena: &SailfishArena, node_idx: NodeId) -> u64 {
    let current_node = arena.get(node_idx).unwrap();
    match current_node.get() {
        Some(val) => u64::from(*val),
        None => {
            let left_val: u64 = compute_magnitude(arena, current_node.first_child().unwrap()) * 3;
            let right_val: u64 = compute_magnitude(arena, current_node.last_child().unwrap()) * 2;
            left_val + right_val
        }
    }
}

fn sailfish_component(input: &str) -> IResult<&str, TempRef> {
    let val_parse: IResult<&str, u8> = u8(input);
    match val_parse {
        Ok((rem_str, value)) => Ok((rem_str, TempRef::Value(value))),
        Err(_) => {
            let (rem_str, subtree) = sailfish_tree(input)?;
            Ok((rem_str, TempRef::Node(Box::new(subtree))))
        }
    }
}

fn sailfish_tree(input: &str) -> IResult<&str, TempNode> {
    let (remain_str, (left, right)) = preceded(
        space0,
        delimited(
            tag("["),
            separated_pair(sailfish_component, tag(","), sailfish_component),
            tag("]"),
        ),
    )(input)?;

    Ok((remain_str, TempNode { left, right }))
}

fn parse_numbers(input: &str, arena: &mut SailfishArena) -> Vec<NodeId> {
    let mut roots: Vec<NodeId> = Vec::new();
    for line in input.lines() {
        let (_, new_number) = sailfish_tree(line).unwrap();
        roots.push(convert_tree(new_number, arena));
    }

    roots
}

fn convert_tree(from: TempNode, arena: &mut SailfishArena) -> NodeId {
    let new_root = arena.new_node(None);
    let mut conversion_stack: Vec<(TempNode, NodeId)> = vec![(from, new_root)];

    while let Some((curr_from, curr_to)) = conversion_stack.pop() {
        let left_node = curr_from.left;
        let right_node = curr_from.right;

        let to_right = match right_node {
            TempRef::Value(val) => arena.new_node(Some(val)),
            TempRef::Node(next_right) => {
                let right = arena.new_node(None);
                conversion_stack.push((*next_right, right));
                right
            }
        };

        let to_left = match left_node {
            TempRef::Value(val) => arena.new_node(Some(val)),
            TempRef::Node(next_left) => {
                let left = arena.new_node(None);
                conversion_stack.push((*next_left, left));
                left
            }
        };

        curr_to.append(to_left, arena);
        curr_to.append(to_right, arena);
    }

    new_root
}

pub fn part1(input: &str) {
    let mut arena: SailfishArena = Arena::new();
    let numbers = parse_numbers(input, &mut arena);

    let mut total_idx = sum(&mut arena, numbers[0], numbers[1]);
    for next_root in numbers.into_iter().skip(2) {
        total_idx = sum(&mut arena, total_idx, next_root);
    }

    println!(
        "Final magnitude of the sum: {}",
        compute_magnitude(&arena, total_idx)
    );
}

pub fn _part2(_input: &str) {
    // let (_, target_trench) = target(input).unwrap();
    // let initial_velocities: HashSet<Point> = target_trench.compute_initial_velocities();
    // println!("Amount of initial velocities: {}", initial_velocities.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trim_whitespace(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        s.split_whitespace().for_each(|w| {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(w);
        });
        result
    }

    fn print_number(arena: &SailfishArena, node_idx: NodeId) -> String {
        let current_node = arena.get(node_idx).unwrap();
        match current_node.get() {
            Some(val) => val.to_string(),
            None => {
                let left_str = print_number(arena, current_node.first_child().unwrap());
                let right_str = print_number(arena, current_node.last_child().unwrap());
                format!("[{},{}]", left_str, right_str)
            }
        }
    }

    /*
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
    */

    #[test]
    fn input_parsing() {
        let input_string = "[1,2]
        [[1,2],3]
        [9,[8,7]]
        [[1,9],[8,5]]
        [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
        [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
        [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

        let mut test_arena: SailfishArena = Arena::new();
        let numbers = parse_numbers(input_string, &mut test_arena);

        let input_lines: Vec<String> = input_string.lines().map(trim_whitespace).collect();

        let printed_trees: Vec<String> = numbers
            .into_iter()
            .map(|root| print_number(&test_arena, root))
            .collect();

        assert_eq!(printed_trees, input_lines, "Wrong parse!");
    }

    #[test]
    fn explosions() {
        let input_string = "[[[[[9,8],1],2],3],4]
        [7,[6,[5,[4,[3,2]]]]]
        [[6,[5,[4,[3,2]]]],1]
        [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";

        let mut test_arena: SailfishArena = Arena::new();
        let numbers = parse_numbers(input_string, &mut test_arena);

        let reduced_numbers: Vec<String> = numbers
            .into_iter()
            .map(|num| {
                reduce(&mut test_arena, num);
                print_number(&test_arena, num)
            })
            .collect();

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

        let mut test_arena: SailfishArena = Arena::new();
        let numbers = parse_numbers(input_string, &mut test_arena);
        let reduced_numbers: Vec<String> = numbers
            .into_iter()
            .map(|num| {
                reduce(&mut test_arena, num);
                print_number(&test_arena, num)
            })
            .collect();

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

        let mut test_arena: SailfishArena = Arena::new();
        let mut sum_roots: Vec<NodeId> = Vec::new();

        for group in input_strings.into_iter() {
            let numbers = parse_numbers(group, &mut test_arena);
            let mut total_root = sum(&mut test_arena, numbers[0], numbers[1]);
            for num in numbers.into_iter().skip(2) {
                total_root = sum(&mut test_arena, total_root, num);
            }

            sum_roots.push(total_root);
        }

        let reduced_numbers: Vec<String> = sum_roots
            .into_iter()
            .map(|root| print_number(&test_arena, root))
            .collect();

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
        let mut test_arena: SailfishArena = Arena::new();
        let numbers = parse_numbers(input_string, &mut test_arena);
        let mut total_idx = sum(&mut test_arena, numbers[0], numbers[1]);
        assert_eq!(
            print_number(&test_arena, total_idx),
            ref_steps.next().unwrap()
        );
        for next_root in numbers.into_iter().skip(2) {
            total_idx = sum(&mut test_arena, total_idx, next_root);
            assert_eq!(
                print_number(&test_arena, total_idx),
                ref_steps.next().unwrap()
            );
        }
    }

    #[test]
    fn magnitude_computation() {
        let input_string = "[[1,2],[[3,4],5]]
            [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
            [[[[1,1],[2,2]],[3,3]],[4,4]]
            [[[[3,0],[5,3]],[4,4]],[5,5]]
            [[[[5,0],[7,4]],[5,5]],[6,6]]
            [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let mut test_arena: SailfishArena = Arena::new();
        let numbers = parse_numbers(input_string, &mut test_arena);

        let magnitudes: Vec<u64> = numbers
            .into_iter()
            .map(|r| compute_magnitude(&test_arena, r))
            .collect();

        let ref_magnitudes: Vec<u64> = vec![143, 1384, 445, 791, 1137, 3488];

        assert_eq!(magnitudes, ref_magnitudes);
    }

    #[test]
    fn string_to_magnitude_fullstack() {
        let input_string = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        let mut test_arena: SailfishArena = Arena::new();
        let numbers = parse_numbers(input_string, &mut test_arena);

        let mut total_idx = sum(&mut test_arena, numbers[0], numbers[1]);
        for next_root in numbers.into_iter().skip(2) {
            total_idx = sum(&mut test_arena, total_idx, next_root);
        }

        assert_eq!(
            print_number(&test_arena, total_idx),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(compute_magnitude(&test_arena, total_idx), 4140);
    }
}
