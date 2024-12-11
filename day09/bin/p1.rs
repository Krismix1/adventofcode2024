use std::fs;

fn parse(input: String) -> u64 {
    let mut disk = Vec::with_capacity(input.len());
    for (idx, ch) in input.trim().bytes().enumerate() {
        let mut to_add = None;
        if idx % 2 == 0 {
            to_add = Some(idx / 2);
        }
        let count = ch - b'0';
        for _ in 0..count {
            disk.push(to_add);
        }
    }
    let mut i = 0usize;
    let mut j = disk.len() - 1;
    let mut sum = 0u64;
    while i <= j {
        // println!("{i}, {j}, {sum} {:?} {:?}", disk[i], disk[j]);
        match disk[i] {
            Some(v) => {
                sum += (v * i) as u64;
                i += 1;
            }
            None => match disk[j] {
                Some(v) => {
                    sum += (v * i) as u64;
                    j -= 1;
                    i += 1;
                }
                None => j -= 1,
            },
        }
    }

    sum
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
