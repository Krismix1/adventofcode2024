use std::fs;

use aoc::parse_input;
use winnow::Parser;

type OpFn = fn(i64, i64) -> i64;
fn add_op(a: i64, b: i64) -> i64 {
    a + b
}

fn mul_op(a: i64, b: i64) -> i64 {
    a * b
}

fn generate_operations(count: usize) -> Vec<Vec<OpFn>> {
    if count == 0 {
        return vec![vec![], vec![]];
    }

    let mut ops = generate_operations(count - 1);
    let mut ops1 = ops.to_vec();
    ops.iter_mut().for_each(|v| v.push(add_op));
    ops1.iter_mut().for_each(|v| v.push(mul_op));
    ops.append(&mut ops1);
    ops
}

fn verify_input(test_value: i64, operators: &[i64]) -> bool {
    generate_operations(operators.len()).iter().any(|ops| {
        let value = operators
            .iter()
            .skip(1)
            .enumerate()
            .fold(operators[0], |acc, el| {
                let op = &ops[el.0];
                op(acc, *el.1)
            });
        value == test_value
    })
}

fn parse(input: String) -> i64 {
    let data = parse_input.parse_next(&mut input.as_str()).unwrap();
    data.iter()
        .filter(|v| verify_input(v.0, &v.1))
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
