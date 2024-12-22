use aoc::get_input;

fn calculate(mut number: u64, iterations: usize) -> u64 {
    for _ in 0..iterations {
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
    }

    number
}

fn solve(input: String) -> u64 {
    input
        .lines()
        .map(|l| calculate(l.parse::<u64>().unwrap(), 2000))
        .sum()
}

fn main() {
    let result = solve(get_input());
    println!("{result}");
}
