use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1};
use nom::sequence::separated_pair;
use nom::combinator::{map};
use nom::IResult;

// Edge parser
fn edge(input: &str) -> IResult<&str, (&str, &str)> {
    let parser = separated_pair(
        alphanumeric1,
        tag("-"),
        alphanumeric1
    );
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        (s.0, s.1)
    })
    (input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CaveType {
    Start,
    End,
    Small(String),
    Big(String)
}

#[derive(Debug)]
struct CaveNetwork {
    edges: HashMap<String, Vec<CaveType>>,
    small_caves: HashSet<String>
}

impl CaveNetwork {
    pub fn new(input: &str) -> CaveNetwork {
        let mut edge_map: HashMap<String, Vec<CaveType>> = HashMap::new();
        let mut small_set: HashSet<String> = HashSet::new();

        for line in input.lines() {
            let (_, (start, end)) = edge(line.trim()).expect("Something went super wrong!");
            let end_type = if end.eq("start") {
                CaveType::Start
            } else if end.eq("end") {
                CaveType::End
            } else if end.chars().fold(true, |is_lowercase, c| is_lowercase && c.is_lowercase()) {
                small_set.insert(end.to_string());
                CaveType::Small(end.to_owned())
            } else {
                CaveType::Big(end.to_owned())
            };

            if end_type != CaveType::Start {
                match edge_map.get_mut(start) {
                    Some(map_entry) => {
                        map_entry.push(end_type);
                    },
                    None => {
                        edge_map.insert(start.to_owned(), vec![end_type]);
                    }
                }
            }

            let start_type = if start.eq("start") {
                CaveType::Start
            } else if start.eq("end") {
                CaveType::End
            } else if start.chars().fold(true, |is_lowercase, c| is_lowercase && c.is_lowercase()) {
                small_set.insert(start.to_string());
                CaveType::Small(start.to_owned())
            } else {
                CaveType::Big(start.to_owned())
            };

            if start_type != CaveType::Start {
                match edge_map.get_mut(end) {
                    Some(map_entry) => {
                        map_entry.push(start_type);
                    },
                    None => {
                        edge_map.insert(end.to_owned(), vec![start_type]);
                    }
                }
            }
        }

        return CaveNetwork{edges: edge_map, small_caves: small_set};
    }

    fn find_unique_paths(&self, repeatable_cave: &str) -> HashSet<String> {
        let mut path_small_caves_visit: Vec<HashSet<String>> = Vec::new();
        let mut path_repeated_cave: Vec<bool> = Vec::new();
        let mut debug_path: Vec<Vec<String>> = Vec::new();
        let mut visit_queue: VecDeque<(usize, CaveType)> = VecDeque::new();
        visit_queue.push_back((0, CaveType::Start));

        let mut found_paths: HashSet<String> = HashSet::new();

        while let Some(current_cave) = visit_queue.pop_front() {
            match current_cave {
                (_, CaveType::Start) => {
                    let first_caves = self.edges.get("start").unwrap();
                    for (path, next_cave) in first_caves.iter().enumerate() {
                        assert_eq!(path_small_caves_visit.len(), path);
                        path_small_caves_visit.push(HashSet::new());
                        path_repeated_cave.push(false);
                        debug_path.push(vec!["start".to_string()]);
                        visit_queue.push_back((path, next_cave.to_owned()));
                    }
                },
                (path, CaveType::End) => {
                    let completed_path = debug_path.get_mut(path).unwrap();
                    completed_path.push("end".to_string());
                    let mut path_string = String::new();
                    for (idx, cave) in completed_path.iter().enumerate() {
                        path_string.push_str(cave);
                        if idx < completed_path.len() - 1 {
                            path_string.push_str("=>");
                        }
                    }
                    found_paths.insert(path_string);
                },
                (path, CaveType::Small(cave_name)) => {
                    let current_visit_path = path_small_caves_visit.get_mut(path).unwrap();
                    if !current_visit_path.contains(&cave_name) {
                        if !cave_name.eq(repeatable_cave) {
                            current_visit_path.insert(cave_name.to_owned());
                        } else if *path_repeated_cave.get(path).unwrap() {
                            current_visit_path.insert(cave_name.to_owned());
                        } else {
                            *path_repeated_cave.get_mut(path).unwrap() = true;
                        }

                        let next_caves = self.edges.get(&cave_name).unwrap();
                        visit_queue.push_back((path, next_caves.first().unwrap().clone()));
                        debug_path.get_mut(path).unwrap().push(cave_name);

                        for next_cave in next_caves.iter().skip(1) {
                            let new_path_id = path_small_caves_visit.len();
                            visit_queue.push_back((new_path_id, next_cave.to_owned()));
                            path_small_caves_visit.push(path_small_caves_visit.get(path).unwrap().clone());
                            debug_path.push(debug_path.get(path).unwrap().clone());
                            path_repeated_cave.push(*path_repeated_cave.get(path).unwrap());
                        }
                    } else {
                        debug_path.get_mut(path).unwrap().push("fail".to_string());
                    }
                },
                (path, CaveType::Big(cave_name)) => {
                    let next_caves = self.edges.get(&cave_name).unwrap();
                    visit_queue.push_back((path, next_caves.first().unwrap().clone()));
                    debug_path.get_mut(path).unwrap().push(cave_name);

                    for next_cave in next_caves.iter().skip(1) {
                        let new_path_id = path_small_caves_visit.len();
                        visit_queue.push_back((new_path_id, next_cave.to_owned()));
                        path_small_caves_visit.push(path_small_caves_visit.get(path).unwrap().clone());
                        debug_path.push(debug_path.get(path).unwrap().clone());
                        path_repeated_cave.push(*path_repeated_cave.get(path).unwrap());
                    }
                }
            };
        }

        /*
        println!("Found paths ({} lead to the end): ", found_paths.len());
        for (path_id, cave_list) in debug_path.iter().enumerate() {
            print!("Path {}: ", path_id);
            for (idx, cave) in cave_list.iter().enumerate() {
                if idx < cave_list.len() - 1 {
                    print!("{} -> ", cave);
                } else {
                    println!("{}", cave);
                }
            }
        }
        */

        return found_paths;
    }

    pub fn find_paths(&self, allow_repetition: bool) -> u64 {
        let mut total_paths: HashSet<String> = HashSet::new();
        if allow_repetition {
            for cave in self.small_caves.iter() {
                total_paths.extend(self.find_unique_paths(&cave).iter().map(|a| a.clone()));
            };
        } else {
            total_paths = self.find_unique_paths("");
        }
        return u64::try_from(total_paths.len()).unwrap();
    }
}

pub fn part1(input: &str) {
    let cave_net = CaveNetwork::new(input);
    let path_count = cave_net.find_paths(false);
    println!("Amount of unique paths to the exit: {}", path_count);
}

pub fn part2(input: &str) {
    let cave_net = CaveNetwork::new(input);
    let path_count = cave_net.find_paths(true);
    println!("Amount of unique paths to the exit considering repeating caves: {}", path_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_network() {
        let input_string = "start-A
                        start-b
                        A-c
                        A-b
                        b-d
                        A-end
                        b-end";

        let cave_net = CaveNetwork::new(input_string);
        let path_count = cave_net.find_paths(false);

        assert_eq!(path_count, 10u64);
    }

    #[test]
    fn shallow_network_repetition() {
        let input_string = "start-A
                        start-b
                        A-c
                        A-b
                        b-d
                        A-end
                        b-end";

        let cave_net = CaveNetwork::new(input_string);
        let path_count = cave_net.find_paths(true);

        assert_eq!(path_count, 36u64);
    }

    #[test]
    fn deeper_network() {
        let input_string = "dc-end
                        HN-start
                        start-kj
                        dc-start
                        dc-HN
                        LN-dc
                        HN-end
                        kj-sa
                        kj-HN
                        kj-dc";

        let cave_net = CaveNetwork::new(input_string);
        let path_count = cave_net.find_paths(false);

        assert_eq!(path_count, 19u64);
    }

    #[test]
    fn deeper_network_repetition() {
        let input_string = "dc-end
                        HN-start
                        start-kj
                        dc-start
                        dc-HN
                        LN-dc
                        HN-end
                        kj-sa
                        kj-HN
                        kj-dc";

        let cave_net = CaveNetwork::new(input_string);
        let path_count = cave_net.find_paths(true);

        assert_eq!(path_count, 103u64);
    }

    #[test]
    fn deepest_network() {
        let input_string = "fs-end
                        he-DX
                        fs-he
                        start-DX
                        pj-DX
                        end-zg
                        zg-sl
                        zg-pj
                        pj-he
                        RW-he
                        fs-DX
                        pj-RW
                        zg-RW
                        start-pj
                        he-WI
                        zg-he
                        pj-fs
                        start-RW";

        let cave_net = CaveNetwork::new(input_string);
        let path_count = cave_net.find_paths(false);

        assert_eq!(path_count, 226u64);
    }

    #[test]
    fn deepest_network_repetition() {
        let input_string = "fs-end
                        he-DX
                        fs-he
                        start-DX
                        pj-DX
                        end-zg
                        zg-sl
                        zg-pj
                        pj-he
                        RW-he
                        fs-DX
                        pj-RW
                        zg-RW
                        start-pj
                        he-WI
                        zg-he
                        pj-fs
                        start-RW";

        let cave_net = CaveNetwork::new(input_string);
        let path_count = cave_net.find_paths(true);

        assert_eq!(path_count, 3509u64);
    }
}