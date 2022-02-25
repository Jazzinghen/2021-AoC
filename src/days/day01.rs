use itertools::Itertools;

pub fn part1(input: &str) {
    let mut increase_amount = 0;
    let mut line_input = input.split('\n');
    let mut last_value: u32 = line_input
        .next()
        .expect("An empty input, really?")
        .trim()
        .parse::<u32>()
        .expect("The first line is not an integer!");

    for curr_value in line_input {
        let num_opt = curr_value.trim().parse::<u32>();
        let num_value = match num_opt {
            Ok(val) => val,
            Err(e) => {
                println!(
                    "String {} doesn't seem to contain a number! Error: {}",
                    curr_value, e
                );
                last_value
            }
        };

        if num_value > last_value {
            increase_amount += 1;
        }
        last_value = num_value;
    }

    println!("Amount of increases: {}", increase_amount);
}

fn sum_window(tuple: (&str, &str, &str)) -> Option<i32> {
    let mut final_sum = 0;
    for value in [tuple.0, tuple.1, tuple.2].iter() {
        let num_opt = value.trim().parse::<i32>();
        match num_opt {
            Ok(val) => {
                final_sum += val;
            }
            Err(e) => {
                println!(
                    "String {} doesn't seem to contain a number! Error: {}",
                    value, e
                );
                return None;
            }
        }
    }

    Some(final_sum)
}

pub fn part2(input: &str) {
    let mut line_input = input.split('\n').tuple_windows::<(_, _, _)>();
    //let mut
    let mut last_full_value =
        sum_window(line_input.next().unwrap()).expect("I am expecting more than three values");
    let mut increases = 0;

    for curr_value in line_input {
        let curr_num = sum_window(curr_value);
        if let Some(num) = curr_num {
            if num > last_full_value {
                increases += 1;
            }
            last_full_value = num;
        }
    }

    println!("Amount of increases: {}", increases);
}
