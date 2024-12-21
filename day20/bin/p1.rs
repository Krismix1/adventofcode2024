use std::collections::HashMap;

use aoc::{a_star, get_input, shortest};

fn get_neighbours(current: (usize, usize), grid: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    dirs.iter()
        .filter_map(|dir| {
            let next_position = (
                current.0.wrapping_add_signed(dir.0),
                current.1.wrapping_add_signed(dir.1),
            );
            if next_position.0 < grid.len()
                && next_position.1 < grid[0].len()
                && grid[next_position.0][next_position.1] == 1
            {
                Some(next_position)
            } else {
                None
            }
        })
        .collect()
}

fn print_grid(
    grid: &[Vec<i32>],
    start: (usize, usize),
    end: (usize, usize),
    pos_1: (usize, usize),
    pos_2: (usize, usize),
) {
    for (i, row) in grid.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if (i, j) == pos_1 {
                print!("1");
            } else if (i, j) == pos_2 {
                print!("2");
            } else if (i, j) == start {
                print!("S");
            } else if (i, j) == end {
                print!("E");
            } else if *el == 1 {
                print!("#");
            } else if *el == 0 {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(input: String) -> usize {
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.bytes()
                .enumerate()
                .map(|(j, el)| {
                    if el == b'#' {
                        1
                    } else {
                        if el == b'S' {
                            start_position = (i, j);
                        } else if el == b'E' {
                            end_position = (i, j);
                        }
                        0
                    }
                })
                .collect()
        })
        .collect();

    let base_path = a_star(&grid, start_position, end_position).unwrap();
    let base_picoseconds = base_path.len() - 1;
    let mut sum = 0;
    for (count, (i, j)) in base_path.iter().enumerate() {
        println!("Checking {}/{}", count + 1, base_path.len());
        let (i, j) = (*i, *j);
        for neighbour in get_neighbours((i, j), &grid) {
            let prev = grid[neighbour.0][neighbour.1];

            grid[neighbour.0][neighbour.1] = 0;
            if let Some(next) = base_path.get(count + 1) {
                grid[next.0][next.1] = 1;
                if let Some(new_path) = a_star(&grid, (i, j), end_position) {
                    let new_picoseconds = new_path.len() - 1;
                    let diff = (base_picoseconds - count) - new_picoseconds;
                    if diff >= 100 {
                        sum += 1;
                    }
                }
                grid[next.0][next.1] = 0;
            }

            grid[neighbour.0][neighbour.1] = prev;
        }
    }

    sum
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
