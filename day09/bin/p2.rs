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
    let mut j = disk.len() - 1;
    let mut sum = 0u64;
    let mut fs_end = disk.len();
    while j > 0 {
        if disk[j].is_none() {
            fs_end = j;
            j -= 1;
            continue;
        }

        if disk[j - 1] != disk[j] {
            // TODO: maybe have to check for j=0||1
            // println!("{:?}", disk[j..fs_end].iter().collect::<Vec<_>>());
            let mut i_start = 0;
            let mut i_end = i_start;
            while fs_end - j > i_end - i_start && i_start < j {
                while disk[i_start].is_some() {
                    i_start += 1;
                    i_end = i_start + 1;
                }
                while disk[i_end].is_none() && i_end < j {
                    i_end += 1;
                }
                if fs_end - j > i_end - i_start {
                    i_start = i_end; // may be out of boundary?
                }
            }
            i_start = j.min(i_start);
            sum += ((i_start + (fs_end - j - 1 + i_start)) * disk[j].unwrap()) as u64
                * (fs_end - j) as u64
                / 2;

            disk.iter_mut()
                .take(i_start + fs_end - j)
                .skip(i_start)
                .for_each(|el| *el = Some(0));
            // println!("{sum}");
            // disk.chunk_by(pred)
            fs_end = j;
        }
        j -= 1;
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
