use std::collections::HashMap;

use aoc::{get_input, parse_input};

fn solve(input: String) -> u64 {
    let (left, right) = parse_input(&mut input.as_str());
    let frequency: HashMap<u64, u64> = right.into_iter().fold(HashMap::new(), |mut acc, el| {
        *acc.entry(el).or_default() += 1;
        acc
    });

    left.into_iter()
        .map(|el| el * frequency.get(&el).unwrap_or(&0))
        .sum()
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
