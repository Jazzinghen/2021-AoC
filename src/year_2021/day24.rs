use itertools::Itertools;

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply(i32, i32),
    Divide(i32, i32),
}

fn parse_operations(input: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();

    for instructions_chunk in input.lines().chunks(18).into_iter() {
        let mut operation: bool = false;
        let mut cmp_bias: i32 = i32::MIN;
        let mut offset: i32 = i32::MIN;
        for (line, data) in instructions_chunk.enumerate() {
            match line {
                4 => {
                    let raw_op = data.split_ascii_whitespace().nth(2).unwrap();
                    operation = raw_op.parse::<i32>().unwrap() == 1;
                }
                5 => {
                    let raw_bias = data.split_ascii_whitespace().nth(2).unwrap();
                    cmp_bias = raw_bias.parse::<i32>().unwrap();
                }
                15 => {
                    let raw_off = data.split_ascii_whitespace().nth(2).unwrap();
                    offset = raw_off.parse::<i32>().unwrap();
                }
                _ => {}
            };
        }

        assert_ne!(cmp_bias, i32::MIN);
        assert_ne!(offset, i32::MIN);

        if operation {
            operations.push(Operation::Multiply(cmp_bias, offset))
        } else {
            operations.push(Operation::Divide(cmp_bias, offset))
        }
    }

    operations
}

fn find_codes(instructions: &[Operation]) -> ([u8; 14], [u8; 14]) {
    let mut min_code: [u8; 14] = [10; 14];
    let mut max_code: [u8; 14] = [0; 14];

    let mut op_stack: Vec<(usize, &Operation)> = Vec::new();

    for (op_id, op) in instructions.iter().enumerate() {
        match op {
            Operation::Multiply(..) => op_stack.push((op_id, op)),
            Operation::Divide(bias, _) => {
                let (prev_id, prev_op) = op_stack.pop().unwrap();

                if let Operation::Multiply(_, prev_offset) = prev_op {
                    let target_diff = prev_offset + bias;

                    let (min, max) = if target_diff > 0 {
                        ((1i32, 1 + target_diff), (9 - target_diff, 9i32))
                    } else {
                        ((1 - target_diff, 1i32), (9i32, 9 + target_diff))
                    };

                    min_code[prev_id] = u8::try_from(min.0).unwrap();
                    max_code[prev_id] = u8::try_from(max.0).unwrap();

                    min_code[op_id] = u8::try_from(min.1).unwrap();
                    max_code[op_id] = u8::try_from(max.1).unwrap();
                }
            }
        }
    }

    (min_code, max_code)
}

pub fn only_part(input: &str) {
    let operations = parse_operations(input);
    let (min, max) = find_codes(&operations);

    println!("Smallest code: {}", min.iter().join(""));
    println!("Largest code: {}", max.iter().join(""));
}

#[cfg(test)]
mod tests {
    use super::*;

    // It's not really useful, just fun
    fn execute(instructions: &str, input: &str) -> [i32; 4] {
        let mut registers: [i32; 4] = [0; 4];

        let mut current_input = input
            .chars()
            .map(|digit| i32::try_from(digit.to_digit(10).unwrap()).unwrap());

        for command in instructions.lines().map(|line| line.trim()) {
            let parts = command.split_ascii_whitespace().collect_vec();
            let dst_idx: usize = match parts[1].to_ascii_lowercase().as_str() {
                "w" => 0,
                "x" => 1,
                "y" => 2,
                "z" => 3,
                _ => panic!(
                    "Got a strange register: \"{}\"",
                    parts[1].to_ascii_lowercase()
                ),
            };

            if parts[0].to_ascii_lowercase().as_str() == "inp" {
                registers[dst_idx] = current_input.next().unwrap();
            } else {
                let src_data = match parts[2].to_ascii_lowercase().as_str() {
                    "w" => registers[0],
                    "x" => registers[1],
                    "y" => registers[2],
                    "z" => registers[3],
                    _ => parts[2].to_ascii_lowercase().parse::<i32>().unwrap(),
                };
                match parts[0].to_ascii_lowercase().as_str() {
                    "add" => registers[dst_idx] += src_data,
                    "mul" => registers[dst_idx] *= src_data,
                    "div" => registers[dst_idx] /= src_data,
                    "mod" => registers[dst_idx] %= src_data,
                    "eql" => registers[dst_idx] = (registers[dst_idx] == src_data) as i32,
                    _ => panic!(
                        "Got a strange instruction: \"{}\"",
                        parts[0].to_ascii_lowercase()
                    ),
                }
            }
        }

        registers
    }

    #[test]
    fn negate() {
        let instructions = "inp x
        mul x -1";
        let input = "9";

        let registers = execute(instructions, input);

        assert_eq!(registers[1], -9);
    }

    #[test]
    fn is_three_times() {
        let instructions = "inp z
        inp x
        mul z 3
        eql z x";

        let input = "39";

        let registers = execute(instructions, input);

        assert_eq!(registers[3], 1);
    }

    #[test]
    fn is_not_three_times() {
        let instructions = "inp z
        inp x
        mul z 3
        eql z x";

        let input = "49";

        let registers = execute(instructions, input);

        assert_eq!(registers[3], 0);
    }

    #[test]
    fn binary_conversion() {
        let instructions = "inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2";

        let input = "7";

        let registers = execute(instructions, input);

        assert_eq!(registers, [0, 1, 1, 1]);
    }
}
