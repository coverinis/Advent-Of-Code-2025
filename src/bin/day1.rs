use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "day1_input.txt"
    };

    let input = fs::read_to_string(filename).expect("Failed to read input file");

    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        match direction {
            'L' => {
                position = (position - amount).rem_euclid(100);
            }
            'R' => {
                position = (position + amount).rem_euclid(100);
            }
            _ => panic!("Unknown direction: {}", direction),
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    println!("Password: {}", zero_count);
}
