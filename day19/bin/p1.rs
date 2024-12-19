use aoc::get_input;
use regex::Regex;

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

fn solve(input: String) -> usize {
    let (patterns, designs) = parse_input(&input);
    let pattern = Regex::new(&format!("^({})+$", patterns.join("|"))).unwrap();
    designs
        .iter()
        .filter(|design| pattern.is_match(design))
        .count()
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
