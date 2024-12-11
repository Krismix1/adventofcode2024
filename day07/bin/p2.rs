use std::fs;

use aoc::parse_input;
use winnow::Parser;

fn is_valid(target: i64, ns: &[i64], n: i64) -> bool {
    if ns.is_empty() || n > target {
        return n == target;
    }
    (is_valid(target, &ns[1..], n * 10i64.pow(ns[0].ilog10() + 1) + ns[0]))
        || is_valid(target, &ns[1..], n + ns[0])
        || is_valid(target, &ns[1..], n * ns[0])
}

fn parse(input: String) -> i64 {
    let data = parse_input.parse_next(&mut input.as_str()).unwrap();
    data.iter()
        .filter(|(target, nums)| is_valid(*target, &nums[1..], nums[0]))
        .map(|v| v.0)
        .sum()
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
