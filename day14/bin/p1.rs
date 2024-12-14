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

fn move_robot(mut pos: (i64, i64), v: (i64, i64), iterations: i64, grid: (i64, i64)) -> (i64, i64) {
    pos.0 = (pos.0 + iterations * v.0).rem_euclid(grid.0);
    pos.1 = (pos.1 + iterations * v.1).rem_euclid(grid.1);

    pos
}

fn solve(input: String) -> i64 {
    let mut robots = parse_input(&input);
    let grid_size_x = robots.iter().map(|el| el.0 .0).max().unwrap() + 1;
    let grid_size_y = robots.iter().map(|el| el.0 .1).max().unwrap() + 1;
    for i in 0..robots.len() {
        let robot = robots.get_mut(i).unwrap();
        let new_pos = move_robot(robot.0, robot.1, 100, (grid_size_x, grid_size_y));
        robot.0 = new_pos;
    }

    let mut tl_q = 0;
    let mut tr_q = 0;
    let mut bl_q = 0;
    let mut br_q = 0;
    for rob in robots {
        if rob.0 .0 < grid_size_x / 2 {
            match rob.0 .1.cmp(&(grid_size_y / 2)) {
                std::cmp::Ordering::Less => tl_q += 1,
                std::cmp::Ordering::Greater => tr_q += 1,
                _ => {}
            }
            continue;
        }
        if rob.0 .0 > grid_size_x / 2 {
            match rob.0 .1.cmp(&(grid_size_y / 2)) {
                std::cmp::Ordering::Less => bl_q += 1,
                std::cmp::Ordering::Greater => br_q += 1,
                _ => {}
            }
            continue;
        }
    }

    tl_q * tr_q * bl_q * br_q
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
