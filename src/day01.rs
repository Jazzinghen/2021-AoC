pub fn part1(input: String) {
    let mut line_input = input.split('\n');

    let mut increase_amount = 0;
    let mut last_value: u32 = line_input.next().expect("An empty input, really?").trim().parse::<u32>().expect("The first line is not an integer!");

    for curr_value in line_input {
        let num_opt = curr_value.trim().parse::<u32>();
        let num_value = match num_opt {
            Ok(val) => {val}
            Err(e) => {
                println!("String {} doesn't seem to contain a number! Error: {}", curr_value, e);
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
