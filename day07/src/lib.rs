use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{preceded, repeat, terminated};
use winnow::{PResult, Parser};

pub fn parse_number(input: &mut &str) -> PResult<i64> {
    // take_while(1.., '0'..='9').parse_next(input)
    digit1.try_map(|s: &str| s.parse::<i64>()).parse_next(input)
}

pub fn parse_operator_delim(input: &mut &str) -> PResult<char> {
    ' '.parse_next(input)
}

pub fn parse_operators(input: &mut &str) -> PResult<Vec<i64>> {
    let operator_parser = preceded(parse_operator_delim, parse_number);
    repeat(2.., operator_parser).parse_next(input)
}

pub fn parse_line(input: &mut &str) -> PResult<(i64, Vec<i64>)> {
    let test_value = parse_number.parse_next(input)?;
    ':'.parse_next(input)?;

    let operators = parse_operators.parse_next(input)?;
    Ok((test_value, operators))
}

pub fn parse_input(input: &mut &str) -> PResult<Vec<(i64, Vec<i64>)>> {
    repeat(1.., terminated(parse_line, line_ending)).parse_next(input)
}
