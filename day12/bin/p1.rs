use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs,
};

fn search(grid: &[Vec<u8>], i: isize, j: isize, seen: &mut HashSet<(isize, isize)>) {
    if !seen.insert((i, j)) {
        return;
    }

    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];
    for dir in directions.iter() {
        let next_i = i + dir.0;
        let next_j = j + dir.1;
        if next_i < 0
            || next_j < 0
            || next_i as usize >= grid.len()
            || next_j as usize >= grid[0].len()
        {
            continue;
        }
        if grid[i as usize][j as usize] == grid[next_i as usize][next_j as usize] {
            search(grid, next_i, next_j, seen);
        }
    }
}

fn get_regions(grid: &[Vec<u8>]) -> Vec<(u8, HashSet<(isize, isize)>)> {
    let mut regions: HashMap<u8, Vec<HashSet<(isize, isize)>>> = HashMap::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            match regions.entry(*letter) {
                Entry::Occupied(mut en) => {
                    if en
                        .get()
                        .iter()
                        .any(|set| set.contains(&(i as isize, j as isize)))
                    {
                        // this cell is already part of another region
                        continue;
                    }
                    let mut cells = HashSet::new();
                    search(grid, i as isize, j as isize, &mut cells);
                    en.get_mut().push(cells);
                }
                Entry::Vacant(_) => {
                    let mut cells = HashSet::new();
                    search(grid, i as isize, j as isize, &mut cells);
                    regions.insert(*letter, vec![cells]);
                }
            }
        }
    }

    regions
        .into_iter()
        .fold(vec![], |mut acc, (letter, regions)| {
            for reg in regions {
                acc.push((letter, reg));
            }
            acc
        })
}

fn get_area<T>(region: &HashSet<T>) -> u64 {
    region.len() as u64
}
fn get_perimeter(region: &HashSet<(isize, isize)>) -> u64 {
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];
    let mut perimeter = 0;
    for cell in region {
        for dir in directions.iter() {
            if !region.contains(&(cell.0 + dir.0, cell.1 + dir.1)) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn solve(input: String) -> u64 {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let regions = get_regions(&grid);

    regions
        .iter()
        .map(|region| get_area(&region.1) * get_perimeter(&region.1))
        .sum()
}

fn main() {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        let real_input = fs::read_to_string("input.txt").unwrap();
        let result = solve(real_input);
        println!("{result}");
    } else {
        let example_input = fs::read_to_string("example.txt").unwrap();
        let result = solve(example_input.clone());
        println!("{result}");
    }
}
