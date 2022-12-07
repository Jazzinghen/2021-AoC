use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, rest};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

enum CLILogLine {
    Entry(FSType),
    Command(CLICommand),
}

enum FSType {
    File((usize, String)),
    Directory(String),
}

enum CLICommand {
    Cd(String),
    List,
}

// File parses
fn file_entry(input: &str) -> IResult<&str, FSType> {
    map(
        separated_pair(digit1, char(' '), rest),
        |(file_size, file_name): (&str, &str)| {
            FSType::File((file_size.parse().unwrap(), file_name.to_string()))
        },
    )(input)
}

fn folder_entry(input: &str) -> IResult<&str, FSType> {
    map(preceded(tag("dir "), rest), |folder: &str| {
        FSType::Directory(folder.to_string())
    })(input)
}

fn cd_command(input: &str) -> IResult<&str, CLICommand> {
    map(preceded(tag("cd "), rest), |folder: &str| {
        CLICommand::Cd(folder.to_string())
    })(input)
}

fn ls_command(input: &str) -> IResult<&str, CLICommand> {
    let (rem_input, _) = tag("ls")(input)?;

    Ok((rem_input, CLICommand::List))
}

fn cli_line(input: &str) -> IResult<&str, CLILogLine> {
    if let Ok((rem_input, command)) = preceded(tag("$ "), alt((ls_command, cd_command)))(input) {
        return Ok((rem_input, CLILogLine::Command(command)));
    }

    map(alt((file_entry, folder_entry)), |entry: FSType| {
        CLILogLine::Entry(entry)
    })(input)
}

fn build_fs_tree(input: &str) -> DiGraph<(String, usize), ()> {
    let mut fs_tree: DiGraph<(String, usize), ()> = DiGraph::new();

    let mut tree_stack: Vec<(String, NodeIndex)> = Vec::new();
    let mut roots: Vec<NodeIndex> = Vec::new();

    for line in input.lines().map(|l| l.trim()) {
        if let Ok((_, log_line)) = cli_line(line) {
            match log_line {
                CLILogLine::Command(CLICommand::Cd(path)) => {
                    if path == ".." {
                        let _ = tree_stack.pop();
                    } else {
                        let new_dir = fs_tree.add_node((path.clone(), 0));
                        if let Some((_, parent_folder)) = tree_stack.last() {
                            fs_tree.add_edge(*parent_folder, new_dir, ());
                        } else {
                            roots.push(new_dir);
                        }
                        tree_stack.push((path, new_dir));
                    }
                }
                CLILogLine::Entry(FSType::File((size, name))) => {
                    let (_, curr_dir) = tree_stack.last().unwrap();
                    let new_file = fs_tree.add_node((name, size));
                    fs_tree.add_edge(*curr_dir, new_file, ());
                }
                _ => {}
            }
        }
    }

    for root in roots.into_iter() {
        let mut dfs_post = DfsPostOrder::new(&fs_tree, root);
        while let Some(node) = dfs_post.next(&fs_tree) {
            if fs_tree.neighbors(node).count() > 0 {
                let mut node_size: usize = 0;
                for child in fs_tree.neighbors(node) {
                    node_size += fs_tree[child].1;
                }
                fs_tree[node].1 = node_size;
            }
        }
    }

    fs_tree
}

fn small_folders_sum(fs_tree: &DiGraph<(String, usize), ()>) -> usize {
    let mut sum: usize = 0;
    for node in fs_tree.node_indices() {
        if fs_tree.neighbors(node).count() > 0 {
            let folder_size = fs_tree[node].1;
            if folder_size <= 100000 {
                sum += folder_size;
            }
        }
    }

    sum
}

fn find_folder_to_delete(
    fs_tree: &DiGraph<(String, usize), ()>,
    update_size: usize,
    total_space: usize,
) -> Result<usize, &str> {
    let mut folders_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut used_space: usize = 0;

    for node in fs_tree.node_indices() {
        if fs_tree.neighbors(node).count() > 0 {
            folders_heap.push(Reverse(fs_tree[node].1));
            used_space = used_space.max(fs_tree[node].1);
        }
    }

    while let Some(next_size) = folders_heap.pop() {
        let new_free_space = used_space - next_size.0;
        if (total_space - new_free_space) >= update_size {
            return Ok(next_size.0);
        }
    }

    Err("Couldn't find a folder big enough to free enough space. Format the system.")
}

pub fn part1(input: &str) {
    let tree = build_fs_tree(input);
    let smol_sum = small_folders_sum(&tree);

    println!("Sum of all the folders with size <= 100k: {}", smol_sum);
}

pub fn part2(input: &str) {
    let tree = build_fs_tree(input);
    let smallest_folder_to_delete = find_folder_to_delete(&tree, 30000000, 70000000).unwrap();

    println!(
        "Size of the smallest folder to delete if we want to install update: {}",
        smallest_folder_to_delete
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";

    #[test]
    fn simple_folder_size() {
        let tree = build_fs_tree(INPUT_STRING);
        let smol_sum = small_folders_sum(&tree);

        assert_eq!(smol_sum, 95437usize);
    }

    #[test]
    fn update_removal_size() {
        let tree = build_fs_tree(INPUT_STRING);
        let smallest_folder_to_delete = find_folder_to_delete(&tree, 30000000, 70000000).unwrap();

        assert_eq!(smallest_folder_to_delete, 24933642usize);
    }
}
