pub fn part1(input: String) {
    let line_input = input.lines();
    let mut forward = 0;
    let mut depth = 0;

    for curr_value in line_input {
        let mut direction_val_split = curr_value.split_whitespace();

        let direction_string = direction_val_split.next().expect("Where did our direction go?").to_lowercase();
        let distance = direction_val_split.next().expect("Where did the distance go?").trim().parse::<i32>().expect("Didn't receive a number");
        match direction_string.as_str() {
            "forward" => {forward += distance},
            "down" => {depth += distance},
            "up" => {
                depth -= distance;
                if depth < 0 {
                    depth = 0;
                }
            },
            _ => {println!("Provided a non-handled direction: {}", direction_string);}
        }
    }

    println!("Travel area: {}", forward * depth);
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