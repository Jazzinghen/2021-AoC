use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::sequence::{pair, preceded};
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Addx(i64),
    Nop,
}

fn asm_op(input: &str) -> IResult<&str, Operation> {
    alt((
        map(tag("noop"), |_| Operation::Nop),
        map(
            preceded(tag("addx "), pair(opt(char::<&str, _>('-')), digit1)),
            |(sign, value)| {
                let numeric_value: i64 = value.parse().unwrap();
                let numeric_sign: i64 = match sign {
                    Some(_) => -1,
                    None => 1,
                };
                Operation::Addx(numeric_value * numeric_sign)
            },
        ),
    ))(input)
}

struct SimpleCpu {
    unrolled_executions: Vec<Operation>,
    rax: i64,
    signal_sum: i64,
    crt_out: [bool; 240],
}

impl SimpleCpu {
    fn new(program: &str) -> Self {
        let mut unrolled_executions: Vec<Operation> = Vec::new();
        for line in program.lines().map(|l| l.trim()) {
            let (_, op) = asm_op(line).unwrap();
            unrolled_executions.push(Operation::Nop);
            if op != Operation::Nop {
                unrolled_executions.push(op);
            }
        }

        Self {
            unrolled_executions,
            rax: 1,
            signal_sum: 0,
            crt_out: [false; 240],
        }
    }

    pub fn execute_program(&mut self) {
        for (clock, op) in self.unrolled_executions.iter().enumerate() {
            let crt_row_delta = self.rax.abs_diff(((clock) % 40).try_into().unwrap());
            if crt_row_delta <= 1 {
                self.crt_out[clock] = true;
            }

            let signal_clock = clock + 1;
            if signal_clock >= 20 && ((signal_clock - 20) % 40) == 0 {
                self.signal_sum += self.rax * i64::try_from(signal_clock).unwrap();
            }

            if let Operation::Addx(val) = op {
                self.rax += val;
            }
        }
    }
}

impl fmt::Display for SimpleCpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for crt_line in &self.crt_out.iter().chunks(40) {
            let output_line: String = crt_line.map(|on| if *on { '#' } else { '.' }).collect();
            writeln!(f, "{}", output_line)?;
        }
        writeln!(f)
    }
}

pub fn only_part(input: &str) {
    let mut cpu = SimpleCpu::new(input);
    cpu.execute_program();

    println!("Signal pulse sum: {}", cpu.signal_sum);
    println!("CRT out: ");
    println!("{}", cpu);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";

    #[test]
    fn simple_signals() {
        let mut cpu = SimpleCpu::new(INPUT_STRING);
        cpu.execute_program();

        assert_eq!(cpu.signal_sum, 13140);
    }
}
