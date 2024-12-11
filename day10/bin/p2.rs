use std::{collections::HashSet, fs, vec};

fn search(
    grid: &[Vec<u8>],
    i: usize,
    j: usize,
    mut seen: HashSet<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    if seen.contains(&(i, j)) {
        return vec![];
    }

    let n = grid[i][j];

    if n == 9 {
        return vec![vec![(i, j)]];
    }

    seen.insert((i, j));

    let mut paths = vec![];

    if i < grid.len() - 1 && grid[i + 1][j] == n + 1 {
        let mut res = search(grid, i + 1, j, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }
    if j < grid[i].len() - 1 && grid[i][j + 1] == n + 1 {
        let mut res = search(grid, i, j + 1, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }

    if i > 0 && grid[i - 1][j] == n + 1 {
        let mut res = search(grid, i - 1, j, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }
    if j > 0 && grid[i][j - 1] == n + 1 {
        let mut res = search(grid, i, j - 1, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }

    paths
}

fn parse(input: String) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();

    let mut sum = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, n) in line.iter().enumerate() {
            if *n == 0 {
                let paths = search(&grid, i, j, HashSet::new());
                sum += paths.len();
            }
        }
    }
    sum
}

fn main() {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        let real_input = fs::read_to_string("input.txt").unwrap();
        let result = parse(real_input);
        println!("{result}");
    } else {
        let example_input = fs::read_to_string("example.txt").unwrap();
        let result = parse(example_input.clone());
        println!("{result}");
    }
}
