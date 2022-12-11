use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, rest};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Sum(usize),
    Mul(usize),
    Square,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Monkey {
    inspection_queue: VecDeque<usize>,
    worry_mod: Operation,
    mod_value: usize,
    next_monkeys: (usize, usize),
    items_inspected: usize,
}

impl Monkey {
    fn from_configuration(config: [&str; 5]) -> Self {
        let inspection_queue = parse_queue(config[0]).unwrap().1;
        let worry_mod = parse_operation(config[1]).unwrap().1;
        let mod_value = parse_test(config[2]).unwrap().1;
        let next_monkeys = (
            parse_next(config[3]).unwrap().1,
            parse_next(config[4]).unwrap().1,
        );

        Self {
            inspection_queue,
            worry_mod,
            mod_value,
            next_monkeys,
            items_inspected: 0,
        }
    }

    pub fn inspect(&mut self, is_worrisome: bool) -> Vec<(usize, usize)> {
        let mut inspection_results: Vec<(usize, usize)> = Vec::new();
        while let Some(next_worry) = self.inspection_queue.pop_front() {
            let mut new_worry = match self.worry_mod {
                Operation::Sum(val) => next_worry + val,
                Operation::Mul(val) => next_worry * val,
                Operation::Square => next_worry * next_worry,
            };

            if !is_worrisome {
                new_worry /= 3;
            }

            let next_monkey: usize = if { new_worry % self.mod_value } == 0 {
                self.next_monkeys.0
            } else {
                self.next_monkeys.1
            };

            inspection_results.push((next_monkey, new_worry));
        }

        self.items_inspected += inspection_results.len();

        inspection_results
    }
}

#[derive(Debug)]
struct InspectionPlant {
    inspectors: Vec<Monkey>,
    mcm: usize,
}

impl InspectionPlant {
    fn new() -> Self {
        Self {
            inspectors: Vec::new(),
            mcm: 0,
        }
    }

    pub fn add_inspector(&mut self, new_inspector: Monkey) {
        self.inspectors.push(new_inspector);
        self.update_mcm();
    }

    fn inspection_round(&mut self, is_worrisome: bool) {
        for insp_id in 0..self.inspectors.len() {
            let inspection_results = self.inspectors[insp_id].inspect(is_worrisome);
            for (next_monkey, worry) in inspection_results.into_iter() {
                self.inspectors[next_monkey]
                    .inspection_queue
                    .push_back(worry % self.mcm);
            }
        }
    }

    fn update_mcm(&mut self) {
        self.mcm = self
            .inspectors
            .iter()
            .map(|insp| insp.mod_value)
            .unique()
            .product1()
            .unwrap();
    }

    pub fn run_inspection(&mut self, rounds: usize) {
        let is_worrisome = rounds > 20;
        for _ in 0..rounds {
            self.inspection_round(is_worrisome);
        }
    }
}

fn parse_queue(input: &str) -> IResult<&str, VecDeque<usize>> {
    map(
        preceded(tag("Starting items: "), separated_list1(tag(", "), digit1)),
        |raw_values: Vec<&str>| {
            raw_values
                .into_iter()
                .map(|val| val.parse().unwrap())
                .collect()
        },
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (_, juicy_bits) = preceded(tag("Operation: new = old "), rest)(input)?;
    alt((
        map(tag("* old"), |_| Operation::Square),
        map(preceded(tag("+ "), digit1), |raw_val: &str| {
            Operation::Sum(raw_val.parse().unwrap())
        }),
        map(preceded(tag("* "), digit1), |raw_val: &str| {
            Operation::Mul(raw_val.parse().unwrap())
        }),
    ))(juicy_bits)
}

fn parse_test(input: &str) -> IResult<&str, usize> {
    map(
        preceded(tag("Test: divisible by "), digit1),
        |raw_val: &str| raw_val.parse().unwrap(),
    )(input)
}

fn parse_next(input: &str) -> IResult<&str, usize> {
    map(
        preceded(
            alt((tag("If true: "), tag("If false: "))),
            preceded(tag("throw to monkey "), digit1),
        ),
        |raw_val: &str| raw_val.parse().unwrap(),
    )(input)
}

fn initialize_inspection(input: &str) -> InspectionPlant {
    let mut plant = InspectionPlant::new();
    for config_group in &input.lines().map(|l| l.trim()).chunks(7) {
        let config_vec = config_group.skip(1).take(5).collect_vec();
        plant.add_inspector(Monkey::from_configuration([
            config_vec[0],
            config_vec[1],
            config_vec[2],
            config_vec[3],
            config_vec[4],
        ]));
    }

    plant
}

pub fn part1(input: &str) {
    let mut plant = initialize_inspection(input);
    plant.run_inspection(20);

    let most_inspections: Vec<usize> = plant
        .inspectors
        .iter()
        .map(|insp| insp.items_inspected)
        .sorted()
        .rev()
        .take(2)
        .collect_vec();

    let monkey_business = most_inspections[0] * most_inspections[1];

    println!("Monkey business after 20 turns: {}", monkey_business)
}

pub fn part2(input: &str) {
    let mut plant = initialize_inspection(input);
    plant.run_inspection(10000);

    let most_inspections: Vec<usize> = plant
        .inspectors
        .iter()
        .map(|insp| insp.items_inspected)
        .sorted()
        .rev()
        .take(2)
        .collect_vec();

    let monkey_business = most_inspections[0] * most_inspections[1];

    println!(
        "Monkey business after 10000 sweaty turns: {}",
        monkey_business
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn simple_monkeys() {
        let mut plant = initialize_inspection(INPUT_STRING);
        plant.run_inspection(20);

        let most_inspections: Vec<usize> = plant
            .inspectors
            .iter()
            .map(|insp| insp.items_inspected)
            .sorted()
            .rev()
            .take(2)
            .collect_vec();

        assert_eq!(most_inspections[0] * most_inspections[1], 10605);
    }

    #[test]
    fn long_monkeys() {
        let mut plant = initialize_inspection(INPUT_STRING);
        plant.run_inspection(10000);

        let most_inspections: Vec<usize> = plant
            .inspectors
            .iter()
            .map(|insp| insp.items_inspected)
            .sorted()
            .rev()
            .take(2)
            .collect_vec();

        assert_eq!(most_inspections[0] * most_inspections[1], 2713310158);
    }
}
