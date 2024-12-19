use std::collections::HashMap;

use aoc::get_input;

fn count(patterns: &[String], design: &str, cache: &mut HashMap<String, u64>) -> u64 {
    if let Some(h) = cache.get(design) {
        return *h;
    }
    let mut prefixes = vec![];

    for patt in patterns {
        if design.starts_with(patt) {
            prefixes.push((patt, design.strip_prefix(patt).unwrap()));
        }
    }

    let mut sum = 0;

    for (_, suffix) in prefixes {
        if suffix.is_empty() {
            sum += 1;
        } else {
            sum += count(patterns, suffix, cache);
        }
    }

    cache.insert(design.to_string(), sum);

    sum
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
