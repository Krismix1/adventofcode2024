use std::fs;

use regex::Regex;

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    let re = Regex::new(r"([\d-]+),([\d-]+)[^\d-]+([\d-]+),([\d-]+)").unwrap();
    input
        .lines()
        .map(|l| {
            let (_, [p2, p1, v2, v1]) = re.captures(l).map(|e| e.extract()).unwrap();
            (
                (p1.parse().unwrap(), p2.parse().unwrap()),
                (v1.parse().unwrap(), v2.parse().unwrap()),
            )
        })
        .collect()
}

fn move_robot(mut pos: (i64, i64), v: (i64, i64), grid: (i64, i64)) -> (i64, i64) {
    pos.0 += v.0;
    if pos.0 < 0 {
        pos.0 += grid.0;
    } else {
        pos.0 %= grid.0;
    }
    pos.1 += v.1;
    if pos.1 < 0 {
        pos.1 += grid.1;
    } else {
        pos.1 %= grid.1;
    }

    pos
}

fn close_to_tree(robots: &[Vec<i32>]) -> bool {
    let mut reversed = vec![vec![0; robots.len()]; robots[0].len()];
    for (i, l) in robots.iter().enumerate() {
        for (j, v) in l.iter().enumerate() {
            reversed[j][i] = *v;
        }
    }

    for i in 0..reversed.len() - 1 {
        for j in 0..reversed[0].len() - 4 {
            if reversed[i][j..j + 4].iter().all(|el| *el > 0)
                && reversed[i + 1][j..j + 4].iter().all(|el| *el > 0)
            {
                return true;
            }
        }
    }

    false
}

fn solve(input: String) -> i64 {
    let mut robots = parse_input(&input);
    let grid_size_x = robots.iter().map(|el| el.0 .0).max().unwrap() + 1;
    let grid_size_y = robots.iter().map(|el| el.0 .1).max().unwrap() + 1;
    let mut iterations = 0;
    loop {
        iterations += 1;
        for i in 0..robots.len() {
            let robot = robots.get_mut(i).unwrap();
            let new_pos = move_robot(robot.0, robot.1, (grid_size_x, grid_size_y));
            robot.0 = new_pos;
        }

        let mut grid = vec![vec![0; grid_size_y as usize]; grid_size_x as usize];
        for robot in robots.iter() {
            grid[robot.0 .0 as usize][robot.0 .1 as usize] += 1;
        }
        if close_to_tree(&grid) {
            for l in grid {
                for ch in l {
                    print!("{}", if ch == 0 { "." } else { "#" });
                }
                println!();
            }
            break;
        }
    }

    iterations
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
