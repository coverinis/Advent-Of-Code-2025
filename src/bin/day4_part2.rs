use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "day4_input.txt"
    };

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        println!("Grid is empty");
        return;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed_count = 0;

    // Directions: (row_offset, col_offset) for 8 neighbors
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        let mut to_remove = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    let mut paper_neighbors = 0;

                    for &(dr, dc) in &directions {
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;

                        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                            if grid[nr as usize][nc as usize] == '@' {
                                paper_neighbors += 1;
                            }
                        }
                    }

                    if paper_neighbors < 4 {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        total_removed_count += to_remove.len();

        for &(r, c) in &to_remove {
            grid[r][c] = '.'; // Remove the paper roll
        }
    }

    let result_str = format!("Total accessible paper rolls: {}", total_removed_count);
    println!("{}", result_str);
}
