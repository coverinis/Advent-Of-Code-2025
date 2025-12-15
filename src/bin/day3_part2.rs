use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "day3_input.txt"
    };

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut total_joltage: u128 = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        // Greedy approach to maximize 12-digit number
        let mut result_digits = Vec::new();
        let mut current_idx = 0;
        let n = digits.len();
        let digits_to_pick = 12;

        for k in 0..digits_to_pick {
            let remaining_needed = digits_to_pick - 1 - k;
            // We can pick from current_idx up to (N - remaining_needed - 1)
            // Example: pick 1st digit (k=0), remaining=11. Range end = N - 11 - 1 = N - 12 (inclusive)
            let search_end = n - remaining_needed;

            let mut best_digit = 0;
            let mut best_idx = current_idx;

            // Search for the largest digit in the valid range
            // If ties, prefer the first occurrence (smallest index) to maximize future options
            for i in current_idx..search_end {
                if digits[i] > best_digit {
                    best_digit = digits[i];
                    best_idx = i;
                }
                if best_digit == 9 {
                    break; // Can't beat 9, take it immediately
                }
            }

            result_digits.push(best_digit);
            current_idx = best_idx + 1;
        }

        // Construct the number from digits
        let mut bank_value: u128 = 0;
        for d in result_digits {
            bank_value = bank_value * 10 + d as u128;
        }

        total_joltage += bank_value;
    }

    println!("Total output joltage: {}", total_joltage);

    Ok(())
}
