use std::fs;

use aoc::{a_star, get_input, parse_input};

fn solve(input: String) -> Option<(usize, usize)> {
    let mut grid = vec![vec![0; 71]; 71];
    let bytes = parse_input(&input);

    let min_steps = 1024;
    for byte in bytes.iter().take(min_steps) {
        grid[byte.0][byte.1] = 1;
    }
    for byte in bytes.iter().skip(min_steps + 1) {
        grid[byte.0][byte.1] = 1;

        let path = a_star(&grid, (0, 0), (grid.len() - 1, grid.len() - 1));
        if path.is_none() {
            return Some((byte.1, byte.0));
        }
    }

    None
}

fn main() {
    let result = solve(get_input());
    println!("{result:?}");
}
