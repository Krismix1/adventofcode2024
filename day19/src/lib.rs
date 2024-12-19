use std::fs;

pub fn get_input() -> String {
    let is_real = std::env::var("REAL").is_ok();
    if is_real {
        fs::read_to_string("input.txt").unwrap()
    } else {
        fs::read_to_string("example.txt").unwrap()
    }
}
