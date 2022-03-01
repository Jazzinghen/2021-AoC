use std::collections::HashMap;

enum CheckResult {
    Wrong(char),
    Incomlete(Vec<char>),
}

fn syntax_line_check(line: &str) -> CheckResult {
    let parentheses_combo = HashMap::from([('(', ')'), ('<', '>'), ('{', '}'), ('[', ']')]);
    let mut parentheses_stack: Vec<char> = Vec::new();
    for par in line.chars() {
        if parentheses_combo.contains_key(&par) {
            parentheses_stack.push(par);
        } else if let Some(pot_open) = parentheses_stack.pop() {
            let paired_character = parentheses_combo
                .get(&pot_open)
                .expect("Found an invalid character in the syntax");
            if par != *paired_character {
                return CheckResult::Wrong(par);
            }
        } else {
            return CheckResult::Wrong(par);
        }
    }
    CheckResult::Incomlete(parentheses_stack)
}

// This function takes the remaining, incomplete, part of a syntax line and computes the autocompletion score
fn compute_autocomplete_cost(input: &[char]) -> u64 {
    let autocomplete_costs = HashMap::from([('(', 1u64), ('[', 2u64), ('{', 3u64), ('<', 4u64)]);
    let autocomplete_stack = input.iter().rev();
    let mut autocomplete_cost = 0u64;

    for curr_open in autocomplete_stack {
        autocomplete_cost *= 5;
        autocomplete_cost += autocomplete_costs.get(curr_open).unwrap();
    }

    autocomplete_cost
}

fn compute_syntax_scores(input: &str) -> (u64, u64) {
    let error_score = HashMap::from([(')', 3u64), ('>', 25137u64), ('}', 1197u64), (']', 57u64)]);
    let mut syntax_score = 0u64;
    let mut autocomplete_costs = Vec::new();
    for syntax_line in input.split_whitespace() {
        match syntax_line_check(syntax_line) {
            CheckResult::Wrong(wrong_char) => syntax_score += error_score.get(&wrong_char).unwrap(),
            CheckResult::Incomlete(remaining_string) => {
                autocomplete_costs.push(compute_autocomplete_cost(&remaining_string));
            }
        }
    }

    autocomplete_costs.sort_unstable();

    (
        syntax_score,
        autocomplete_costs[autocomplete_costs.len() / 2],
    )
}

pub fn part1(input: &str) {
    let (syntax_error_score, autocomplete_cost) = compute_syntax_scores(input);
    println!(
        "Syntax error score: {}; Autocomplete cost: {}",
        syntax_error_score, autocomplete_cost
    );
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

        let (syntax_error_score, _) = compute_syntax_scores(input_string);

        assert_eq!(syntax_error_score, 26397u64);
    }

    #[test]
    fn complete_lines() {
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

        let (_, autocomplete_cost) = compute_syntax_scores(input_string);

        assert_eq!(autocomplete_cost, 288957u64);
    }
}
