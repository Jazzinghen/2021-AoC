use std::collections::{HashMap};

fn syntax_line_check(line: &str) -> Option<char> {
    let parentheses_combo = HashMap::from([('(', ')'), ('<', '>'), ('{', '}'), ('[', ']')]);
    let mut parentheses_stack: Vec<char> = Vec::new();
    for par in line.chars() {
        if parentheses_combo.contains_key(&par) {
            parentheses_stack.push(par);
        } else {
            if let Some(pot_open) = parentheses_stack.pop() {
                let paired_character = parentheses_combo.get(&pot_open).expect("Found an invalid character in the syntax");
                if par != *paired_character {
                    return Some(par);
                }
            } else {
                return Some(par);
            }
        }
    }
    None
}

fn compute_syntax_error_score(input: &str) -> u64 {
    let error_score = HashMap::from([(')', 3u64), ('>', 25137u64), ('}', 1197u64), (']', 57u64)]);
    let mut score = 0u64;
    for syntax_line in input.split_whitespace() {
        if let Some(wrong_char) = syntax_line_check(syntax_line) {
            println!("Found a non-matched character: {}!", wrong_char);
            score += error_score.get(&wrong_char).unwrap();
        }
    }

    return score;
}

pub fn part1(input: &str) {
    //let min_consumption = min_crab_fuel(input, simple_delta);
    //println!("Estimated minimum cost: {}", min_consumption);
}

pub fn part2(input: &str) {
    //let min_consumption = min_crab_fuel(input, linear_delta);
    //println!("Estimated minimum geometric cost: {}", min_consumption);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_mismatch_parentheses() {
        let input_string = "[({(<(())[]>[[{[]{<()<>>
                            [(()[<>])]({[<{<<[]>>(
                            {([(<{}[<>[]}>{[]{[(<()>
                            (((({<>}<{<{<>}{[]{[]{}
                            [[<[([]))<([[{}[[()]]]
                            [{[{({}]{}}([{[{{{}}([]
                            {<[[]]>}<{[{[{[]{()[[[]
                            [<(<(<(<{}))><([]([]()
                            <{([([[(<>()){}]>(<<{{
                            <{([{{}}[<[[[<>{}]]]>[]]";

        let syntax_error_score = compute_syntax_error_score(input_string);

        assert_eq!(syntax_error_score, 26397u64);
    }
}