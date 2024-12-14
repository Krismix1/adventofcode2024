use std::fs;

use regex::Regex;

fn parse_number_line(input: &str) -> (i64, i64) {
    let re = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
    let (_, [x, y]) = re.captures(input).map(|caps| caps.extract()).unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn solve_prize(btn_a: (i64, i64), btn_b: (i64, i64), prize: (i64, i64)) -> i64 {
    let (x1, x2) = btn_a;
    let (y1, y2) = btn_b;
    let (z1, z2) = prize;
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return 0;
    }
    a * 3 + b
}

fn solve(input: String) -> i64 {
    let mut line_iter = input.lines();
    let mut sum = 0;
    loop {
        let l1 = line_iter.next().unwrap();
        let l2 = line_iter.next().unwrap();
        let l3 = line_iter.next().unwrap();
        let btn_a = parse_number_line(l1);
        let btn_b = parse_number_line(l2);
        let prize = parse_number_line(l3);
        sum += solve_prize(btn_a, btn_b, (prize.0, prize.1));

        if line_iter.next().is_none() {
            break;
        }
    }
    sum
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
