use std::{collections::HashMap, fs};

use winnow::{
    ascii::digit1,
    combinator::{alt, repeat, terminated},
    PResult, Parser,
};

fn _parse_input(input: &mut &str) -> PResult<Vec<u64>> {
    repeat(1.., terminated(digit1.parse_to::<u64>(), alt((" ", "\n")))).parse_next(input)
}

pub fn solve(input: String, mut iterations: u64) -> u64 {
    let numbers = _parse_input.parse_next(&mut input.as_str()).unwrap();
    let mut map: HashMap<u64, u64> = HashMap::new();
    for n in numbers {
        *map.entry(n).or_default() += 1;
    }
    while iterations > 0 {
        iterations -= 1;
        let mut new_map: HashMap<u64, u64> = HashMap::new();
        for (&n, &count) in map.iter() {
            if n == 0 {
                *new_map.entry(1).or_default() += count;
                continue;
            }

            let digits_count = n.ilog10() + 1;
            if (digits_count) % 2 == 0 {
                let lhs: u64 = n / 10u64.pow(digits_count / 2);
                let rhs: u64 = n % 10u64.pow(digits_count / 2);
                *new_map.entry(lhs).or_default() += count;
                *new_map.entry(rhs).or_default() += count;
                continue;
            }
            *new_map.entry(n * 2024).or_default() += count;
        }
        map = new_map;
    }

    map.values().sum()
}

pub fn get_input() -> String {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        fs::read_to_string("input.txt").unwrap()
    } else {
        fs::read_to_string("example.txt").unwrap()
    }
}
