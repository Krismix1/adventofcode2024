use std::iter::zip;

use aoc::{get_input, parse_input};

fn solve(input: String) -> u64 {
    let (mut left, mut right) = parse_input(&mut input.as_str());
    left.sort();
    right.sort();
    zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
