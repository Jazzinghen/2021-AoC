pub fn part1(input: &str) {
    let mut line_input = input.lines().peekable();
    let mut frequencies = vec![
        0;
        line_input
            .peek()
            .expect("I need at least one line, man.")
            .len()
    ];

    for curr_value in line_input {
        for (pos, bit) in curr_value.chars().enumerate() {
            match bit {
                '0' => frequencies[pos] -= 1,
                '1' => frequencies[pos] += 1,
                _ => {
                    println!("Got character {} in a binary string", bit)
                }
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

pub fn check_for_one(binary_value: &str, bit_pos: usize) -> Option<bool> {
    match binary_value.chars().nth(bit_pos).unwrap_or('x') {
        '1' => Some(true),
        '0' => Some(false),
        _ => None,
    }
}

pub fn part2(input: &str) {
    let line_input = input.lines();

    let (one_data, zero_data): (Vec<&str>, Vec<&str>) = line_input.partition(|line| {
        check_for_one(line, 0)
            .unwrap_or_else(|| panic!("Didn't get a proper binary string! Got {}", line))
    });

    let (mut oxygen_data, mut carbon_data) = if one_data.len() >= zero_data.len() {
        (one_data, zero_data)
    } else {
        (zero_data, one_data)
    };

    let mut curr_bit = 1;

    while oxygen_data.len() > 1 {
        let (one, zero): (Vec<&str>, Vec<&str>) = oxygen_data.iter().partition(|line| {
            check_for_one(line, curr_bit)
                .unwrap_or_else(|| panic!("Didn't get a proper binary string! Got {}", line))
        });
        curr_bit += 1;
        oxygen_data = if one.len() >= zero.len() { one } else { zero };
    }

    let oxygen_score = match isize::from_str_radix(oxygen_data[0], 2) {
        Ok(score) => score,
        Err(e) => {
            panic!(
                "Couldn't parse the value of {}, got error: {}",
                oxygen_data[0], e
            )
        }
    };

    curr_bit = 1;

    while carbon_data.len() > 1 {
        let (one, zero): (Vec<&str>, Vec<&str>) = carbon_data.iter().partition(|line| {
            check_for_one(line, curr_bit)
                .unwrap_or_else(|| panic!("Didn't get a proper binary string! Got {}", line))
        });
        curr_bit += 1;
        carbon_data = if one.len() >= zero.len() { zero } else { one };
    }

    let carbon_score = match isize::from_str_radix(carbon_data[0], 2) {
        Ok(score) => score,
        Err(e) => {
            panic!(
                "Couldn't parse the value of {}, got error: {}",
                carbon_data[0], e
            )
        }
    };

    println!("Life support rating: {}", oxygen_score * carbon_score);
}
