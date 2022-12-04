use std::convert::From;

use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rucksack {
    full_contents: HashSet<char>,
    front: HashSet<char>,
    back: HashSet<char>,
    shared_stuff: HashSet<char>,
}

impl Rucksack {
    fn new(raw_data: &str) -> Self {
        let contents_amount = raw_data.chars().count();
        assert_eq!(contents_amount % 2, 0);
        let half_data = contents_amount / 2;
        let mut front: HashSet<char> = HashSet::new();

        for content in raw_data.chars().take(half_data) {
            front.insert(content);
        }

        let mut back: HashSet<char> = HashSet::new();
        for content in raw_data.chars().skip(half_data) {
            back.insert(content);
        }

        let shared_stuff: HashSet<char> = find_shared_contents(&front, &back);

        Self {
            full_contents: front.union(&back).copied().collect(),
            front,
            back,
            shared_stuff,
        }
    }

    pub fn compute_priority(&self) -> u64 {
        self.shared_stuff.iter().map(compute_priority_score).sum()
    }
}

fn compute_priority_score(content: &char) -> u64 {
    if content.is_lowercase() {
        let base_value: u64 = 'a'.into();
        u64::from(*content) - base_value + 1
    } else {
        let base_value: u64 = 'A'.into();
        u64::from(*content) - base_value + 27
    }
}

fn find_shared_contents(first: &HashSet<char>, second: &HashSet<char>) -> HashSet<char> {
    first.intersection(second).copied().collect()
}

fn find_group_priority(rucks: [&Rucksack; 3]) -> char {
    let first_intersection = find_shared_contents(&rucks[0].full_contents, &rucks[1].full_contents);
    let total_intersection = find_shared_contents(&first_intersection, &rucks[2].full_contents);

    assert_eq!(total_intersection.len(), 1);
    total_intersection.into_iter().next().unwrap()
}

pub fn part1(input: &str) {
    let rucks = input
        .lines()
        .map(|line| Rucksack::new(line.trim()))
        .collect_vec();
    let priority_score = rucks.iter().fold(0u64, |acc, r| acc + r.compute_priority());
    println!("Sum of the priorities: {}", priority_score);
}

pub fn part2(input: &str) {
    let rucks = input
        .lines()
        .map(|line| Rucksack::new(line.trim()))
        .collect_vec();

    let mut group_scores = 0u64;

    for group in &rucks.iter().chunks(3) {
        let group_rucks = group.collect_vec();
        let shared_content = find_group_priority([group_rucks[0], group_rucks[1], group_rucks[2]]);
        group_scores += compute_priority_score(&shared_content);
    }
    println!("Group scores: {}", group_scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn simple_priority() {
        let rucks = INPUT_STRING
            .lines()
            .map(|line| Rucksack::new(line.trim()))
            .collect_vec();
        let priority_score = rucks.iter().fold(0u64, |acc, r| acc + r.compute_priority());

        assert_eq!(priority_score, 157u64);
    }

    #[test]
    fn group_priorities() {
        let rucks = INPUT_STRING
            .lines()
            .map(|line| Rucksack::new(line.trim()))
            .collect_vec();

        let mut group_scores = 0u64;

        for group in &rucks.iter().chunks(3) {
            let group_rucks = group.collect_vec();
            let shared_content =
                find_group_priority([group_rucks[0], group_rucks[1], group_rucks[2]]);
            group_scores += compute_priority_score(&shared_content);
        }

        assert_eq!(group_scores, 70u64);
    }
}
