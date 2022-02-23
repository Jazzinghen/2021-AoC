use std::collections::{HashMap};
use std::hash::Hash;

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
    template_polymer: String,
    insertion_rules: HashMap<String, char>,
    pair_frequencies: HashMap<String, u64>
}

impl PolymerData {
    pub fn new(input: &str) -> PolymerData {
        let mut polymer_lines = input.lines();
        let polymer_template = polymer_lines.next().unwrap();

        let mut initial_frequencies: HashMap<String, u64> = HashMap::new();
        for (first, second) in polymer_template.chars().tuple_windows() {
            let pair: String = vec![first, second].iter().collect();
            if let Some(freq) = initial_frequencies.get_mut(&pair) {
                *freq += 1;
            } else {
                initial_frequencies.insert(pair, 1u64);
            }
        }

        let mut rules: HashMap<String, char> = HashMap::new();
        for line in polymer_lines.skip(1) {
            let (_, (pair, result)) = insertion_rule(line).unwrap();
            rules.insert(pair.to_string(), result.chars().next().unwrap());
        }

        PolymerData{template_polymer: polymer_template.to_string(), insertion_rules: rules, pair_frequencies: initial_frequencies}
    }

    fn step(&mut self) {
        let mut step_result: HashMap<String, u64> = HashMap::new();

        for (pair, freq) in self.pair_frequencies.iter() {
            if let Some(evo) = self.insertion_rules.get(pair) {
                let (first_char, second_char) = pair.chars().next_tuple().unwrap();
                let first_pair: String = vec![first_char, *evo].iter().collect();
                let second_pair: String = vec![*evo, second_char].iter().collect();
                if let Some(first_freq) = step_result.get_mut(&first_pair) {
                    *first_freq += freq;
                } else {
                    step_result.insert(first_pair, *freq);
                }
                if let Some(second_freq) = step_result.get_mut(&second_pair) {
                    *second_freq += freq;
                } else {
                    step_result.insert(second_pair, *freq);
                }
            }
        }
        self.pair_frequencies = step_result;
    }

    pub fn evolve_polymer(&mut self, steps: u64) {
        for _ in 0..steps {
            self.step()
        }
    }

    pub fn compute_elements_delta(self) -> (u64, HashMap<char, u64>) {
        let mut frequencies: HashMap<char, u64> = HashMap::new();

        for (pair, freq) in self.pair_frequencies.iter() {
            let (element, _) = pair.chars().next_tuple().unwrap();
            if let Some(elem_freq) = frequencies.get_mut(&element) {
                *elem_freq += *freq;
            } else {
                frequencies.insert(element, *freq);
            }
        }

        let last_element = self.template_polymer.chars().last().unwrap();
        if let Some(last_freq) = frequencies.get_mut(&last_element) {
            *last_freq += 1u64;
        } else {
            frequencies.insert(last_element, 1u64);
        }

        let mut delta: u64 = 0u64;
        if let MinMaxResult::MinMax(min, max) = frequencies.values().minmax() {
            delta = max - min;
        } else {
            panic!("For some reason we didn't find a min and max in the values!");
        }

        return (delta, frequencies);
    }
}

pub fn part1(input: &str) {
    let mut data = PolymerData::new(input);
    data.evolve_polymer(10);
    let (elements_delta, _) = data.compute_elements_delta();
    println!("Delta between highest and lowest frequency elements after 10 steps: {}", elements_delta);
}

pub fn part2(input: &str) {
    let mut data = PolymerData::new(input);
    data.evolve_polymer(40);
    let (elements_delta, _) = data.compute_elements_delta();
    println!("Delta between highest and lowest frequency elements after 40 steps: {}", elements_delta);
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

        // Resulting polymer: NCNBCHB
        // Frequencies:
        //   - N: 2
        //   - C: 2
        //   - B: 2
        //   - H: 1
        let (delta, freqs) = data.compute_elements_delta();
        assert_eq!(delta, 1u64);

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

        // Resulting polymer: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
        let (delta, freqs) = data.compute_elements_delta();
        assert_eq!(delta, 18u64);
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

        let (delta, freqs) = data.compute_elements_delta();
        assert_eq!(delta, 1588u64);
    }
}