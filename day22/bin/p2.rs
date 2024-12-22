use std::collections::{HashMap, HashSet};

use aoc::get_input;

fn calculate(mut number: u64) -> u64 {
    // step 1
    let mut n = number * 64;
    number ^= n;
    number %= 16777216;

    // step 2
    n = number / 32;
    number ^= n;
    number %= 16777216;

    // step 3
    n = number * 2048;
    number ^= n;
    number %= 16777216;

    number
}

type MPair = (u64, i64);
fn get_differences(mut number: u64, iterations: usize) -> Vec<MPair> {
    let mut differences = Vec::with_capacity(iterations);
    let mut prev_number = number;
    for _ in 0..iterations {
        number = calculate(number);
        let diff = (number % 10) as i64 - (prev_number % 10) as i64;
        differences.push((number % 10, diff));
        prev_number = number;
    }

    differences
}

fn group<E: Clone + Copy>(elements: &[E]) -> Vec<(E, E, E, E)> {
    let mut groups = vec![];
    for i in 0..elements.len() - 4 {
        groups.push((
            elements[i],
            elements[i + 1],
            elements[i + 2],
            elements[i + 3],
        ));
    }
    groups
}

fn group_to_map(elements: &[(MPair, MPair, MPair, MPair)]) -> HashMap<(i64, i64, i64, i64), u64> {
    let mut map = HashMap::new();
    for el in elements {
        let key = (el.0 .1, el.1 .1, el.2 .1, el.3 .1);
        let value = el.3 .0;
        map.entry(key).or_insert(value); // must keep first seen value
    }

    map
}

fn solve(input: String) -> u64 {
    let monkeys = input
        .lines()
        .map(|l| group_to_map(&group(&get_differences(l.parse().unwrap(), 2000))))
        .collect::<Vec<_>>();

    let combos = monkeys
        .iter()
        .flat_map(|m| m.keys())
        .collect::<HashSet<_>>();

    combos
        .iter()
        .map(|combo| monkeys.iter().map(|m| m.get(combo).unwrap_or(&0)).sum())
        .max()
        .unwrap()
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
