use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};
fn h(current: (usize, usize), goal: (usize, usize)) -> u64 {
    (goal.0.abs_diff(current.0) + goal.1.abs_diff(current.1)) as u64
}

fn d(node: (usize, usize), neighbour: (usize, usize)) -> u64 {
    // d(current,neighbor) is the weight of the edge from current to neighbor
    (node.0.abs_diff(neighbour.0) + node.1.abs_diff(neighbour.1)) as u64
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    mut current: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut total_path = vec![current];
    while let Some(c) = came_from.get(&current) {
        current = *c;
        total_path.insert(0, current);
    }

    total_path
}

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
                && grid[next_position.0][next_position.1] == 0
            {
                Some(next_position)
            } else {
                None
            }
        })
        .collect()
}

pub fn a_star(
    grid: &[Vec<i32>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse(start));

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, h(start, goal));

    while let Some(current) = open_set.pop() {
        if current.0 == goal {
            return Some(reconstruct_path(came_from, current.0));
        }
        // open_set.pop();

        let neighbour_of_current = get_neighbours(current.0, grid);
        for neighbour in neighbour_of_current {
            let tentative_g_score =
                g_score.get(&current.0).unwrap_or(&u64::MAX) + d(current.0, neighbour);
            if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&u64::MAX) {
                came_from.insert(neighbour, current.0);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + h(neighbour, goal));
                if !open_set.clone().into_vec().contains(&Reverse(neighbour)) {
                    open_set.push(Reverse(neighbour));
                }
            }
        }
    }

    None
}

fn search(
    grid: &[Vec<i32>],
    start: (usize, usize),
    end: (usize, usize),
    mut seen: HashSet<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    if seen.contains(&start) {
        return vec![];
    }

    if start == end {
        return vec![vec![start]];
    }

    seen.insert(start);
    let (i, j) = start;

    let mut paths = vec![];

    if i < grid.len() - 1 && grid[i + 1][j] == 0 {
        let mut res = search(grid, (i + 1, j), end, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }
    if j < grid[i].len() - 1 && grid[i][j + 1] == 0 {
        let mut res = search(grid, (i, j + 1), end, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }

    if i > 0 && grid[i - 1][j] == 0 {
        let mut res = search(grid, (i - 1, j), end, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }
    if j > 0 && grid[i][j - 1] == 0 {
        let mut res = search(grid, (i, j - 1), end, seen.clone());
        if !res.is_empty() {
            res.iter_mut().for_each(|el| el.push((i, j)));
            paths.extend(res);
        }
    }

    paths
}

pub fn shortest(grid: &[Vec<i32>], start: (usize, usize), end: (usize, usize)) -> usize {
    let paths = search(grid, start, end, HashSet::new());
    paths.iter().map(|el| el.len()).min().unwrap()
}

pub fn get_input() -> String {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        fs::read_to_string("input.txt").unwrap()
    } else {
        fs::read_to_string("example.txt").unwrap()
    }
}
