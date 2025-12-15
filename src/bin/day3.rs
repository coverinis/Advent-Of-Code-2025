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

    let mut total_joltage = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut max_bank_joltage = 0;

        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let joltage = digits[i] * 10 + digits[j];
                if joltage > max_bank_joltage {
                    max_bank_joltage = joltage;
                }
            }
        }

        total_joltage += max_bank_joltage;
    }

    println!("Total output joltage: {}", total_joltage);

    Ok(())
}
