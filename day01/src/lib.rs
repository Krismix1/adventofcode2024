use std::fs;
use winnow::{
    ascii::{digit1, newline},
    combinator::{repeat, terminated},
    PResult, Parser,
};

fn parse_number(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

pub fn parse_input(input: &mut &str) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<(u64, u64)> = repeat(
        1..,
        (
            terminated(parse_number, "   "),
            terminated(parse_number, newline),
        ),
    )
    .parse_next(input)
    .unwrap();

    lines.into_iter().fold((vec![], vec![]), |mut acc, el| {
        acc.0.push(el.0);
        acc.1.push(el.1);
        acc
    })
}

pub fn get_input() -> String {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        fs::read_to_string("input.txt").unwrap()
    } else {
        fs::read_to_string("example.txt").unwrap()
    }
}
