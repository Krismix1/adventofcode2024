use std::fs;

use aoc::{a_star, get_input, parse_input};

fn solve(input: String) -> u64 {
    let mut grid = vec![vec![0; 71]; 71];
    let bytes = parse_input(&input);

    for byte in bytes.iter().take(1024) {
        grid[byte.0][byte.1] = 1;
    }
    let path = a_star(&grid, (0, 0), (70, 70)).unwrap();

    for (i, line) in grid.iter().enumerate() {
        for (j, el) in line.iter().enumerate() {
            print!(
                "{}",
                if *el == 1 {
                    '#'
                } else if path.contains(&(i, j)) {
                    'O'
                } else {
                    '.'
                }
            );
        }
        println!();
    }

    path.len() as u64 - 1
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
