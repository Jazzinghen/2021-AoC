pub fn part1(input: String) {
    let mut line_input = input.lines().peekable();
    let mut frequencies = vec![0; line_input.peek().expect("I need at least one line, man.").len()];

    for curr_value in line_input {
        for (pos, bit) in curr_value.chars().enumerate() {
            match bit {
                '0' => {frequencies[pos] -= 1},
                '1' => {frequencies[pos] += 1},
                _ => {println!("Got character {} in a binary string", bit)}
            }
        }
    }

    println!("Final frequencies: {:?}", frequencies);

    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    for freq in frequencies {
        gamma <<= 1;
        epsilon <<= 1;
        if freq > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    let power_consumption = gamma * epsilon;
    println!("Power consumption: {}", power_consumption);
}

pub fn part2(input: String) {
    let line_input = input.lines();
    let mut forward = 0;
    let mut depth = 0;
    let mut angle = 0;

    for curr_value in line_input {
        let mut direction_val_split = curr_value.split_whitespace();

        let direction_string = direction_val_split.next().expect("Where did our direction go?").to_lowercase();
        let distance = direction_val_split.next().expect("Where did the distance go?").trim().parse::<i32>().expect("Didn't receive a number");
        match direction_string.as_str() {
            "forward" => {
                forward += distance;
                depth += distance * angle;
                if depth < 0 {
                    depth = 0;
                }
            },
            "down" => {angle += distance},
            "up" => {angle -= distance},
            _ => {println!("Provided a non-handled direction: {}", direction_string);}
        }
    }

    println!("Travel area: {}", forward * depth);
}