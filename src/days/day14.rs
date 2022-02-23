use std::collections::{HashMap};

use itertools::{Itertools, MinMaxResult};

use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1, space0};
use nom::sequence::{separated_pair, preceded};
use nom::{IResult};

// Polymer evolution parser
fn insertion_rule(input: &str) -> IResult<&str, (&str, &str)> {
    preceded(space0, separated_pair(
        alpha1,
        tag(" -> "),
        alpha1
    ))(input)
}

#[derive(Debug)]
struct PolymerData {
    current_polymer: String,
    insertion_rules: HashMap<String, char>
}

impl PolymerData {
    pub fn new(input: &str) -> PolymerData {
        let mut polymer_lines = input.lines();
        let polymer_template = polymer_lines.next().unwrap();

        let mut rules: HashMap<String, char> = HashMap::new();
        for line in polymer_lines.skip(1) {
            let (_, (pair, result)) = insertion_rule(line).unwrap();
            rules.insert(pair.to_string(), result.chars().next().unwrap());
        }

        PolymerData{current_polymer: polymer_template.to_string(), insertion_rules: rules}
    }

    fn step(&mut self) {
        let mut step_result: String = String::new();
        step_result.push(self.current_polymer.chars().next().unwrap());
        for (first, second) in self.current_polymer.chars().tuple_windows() {
            let pair: String = vec![first, second].iter().collect();
            if let Some(result) = self.insertion_rules.get(&pair) {
                step_result.push(*result);
            }
            step_result.push(second);
        }
        self.current_polymer = step_result;
    }

    pub fn evolve_polymer(&mut self, steps: u64) {
        for _ in 0..steps {
            self.step()
        }
    }

    pub fn compute_elements_delta(self) -> u64 {
        let mut frequencies: HashMap<char, u64> = HashMap::new();

        for element in self.current_polymer.chars() {
            if let Some(freq) = frequencies.get_mut(&element) {
                *freq += 1u64;
            } else {
                frequencies.insert(element, 1u64);
            }
        }

        if let MinMaxResult::MinMax(min, max) = frequencies.into_values().minmax() {
            return max - min;
        } else {
            panic!("For some reason we didn't find a min and max in the values!");
        }
    }
}

pub fn part1(input: &str) {
    let mut data = PolymerData::new(input);
    data.evolve_polymer(10);
    let elements_delta = data.compute_elements_delta();
    println!("Delta between highest and lowest frequency elements: {}", elements_delta);
}

pub fn _part2(_input: &str) {
    // let mut data = ActivationData::new(input);
    // data.fold_all();
    // println!("Activation paper after folding:");
    // println!();
    // data.print_activation();
    // println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_polymer_step() {
        let input_string = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let mut data = PolymerData::new(input_string);
        data.evolve_polymer(1);

        assert_eq!(data.current_polymer, "NCNBCHB");
    }

    #[test]
    fn four_polymer_steps() {
        let input_string = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let mut data = PolymerData::new(input_string);
        data.evolve_polymer(4);

        assert_eq!(data.current_polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn ten_polymer_steps_delta() {
        let input_string = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let mut data = PolymerData::new(input_string);
        data.evolve_polymer(10);

        assert_eq!(data.compute_elements_delta(), 1588u64);
    }
}