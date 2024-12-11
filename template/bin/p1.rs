use std::fs;

fn solve(input: String) -> i64 {
    0
}

fn main() {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        let real_input = fs::read_to_string("input.txt").unwrap();
        let result = solve(real_input);
        println!("{result}");
    } else {
        let example_input = fs::read_to_string("example.txt").unwrap();
        let result = solve(example_input.clone());
        println!("{result}");
    }
}

