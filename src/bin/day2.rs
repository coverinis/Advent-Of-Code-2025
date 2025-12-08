use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "day2_input.txt"
    };

    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let input = input.trim();

    let mut total_invalid_sum: u64 = 0;

    // Split by comma to get ranges
    for range_str in input.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = range_str.split('-').collect();
        let start: u64 = parts[0].parse().expect("Invalid start of range");
        let end: u64 = parts[1].parse().expect("Invalid end of range");

        for id in start..=end {
            if is_invalid(id) {
                total_invalid_sum += id;
            }
        }
    }

    println!("Sum of invalid IDs: {}", total_invalid_sum);
}

fn is_invalid(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    let first_half = &s[0..mid];
    let second_half = &s[mid..len];

    first_half == second_half
}
