use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "day4_input.txt"
    };

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        println!("Grid is empty");
        return;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_count = 0;

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
                    accessible_count += 1;
                }
            }
        }
    }

    println!("Total accessible paper rolls: {}", accessible_count);
}
