use std::collections::HashMap;

use aoc::get_input;

fn count(patterns: &[String], design: &str, cache: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(h) = cache.get(design) {
        return *h;
    }
    let n = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .map(|pattern| count(patterns, &design[pattern.len()..], cache))
        .sum();

    cache.insert(design.to_string(), n);

    n
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let patterns: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    lines.next();

    let designs: Vec<String> = lines.map(ToOwned::to_owned).collect();

    (patterns, designs)
}

fn solve(input: String) -> u64 {
    let (patterns, designs) = parse_input(&input);
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| count(&patterns, design, &mut cache))
        .sum()
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
