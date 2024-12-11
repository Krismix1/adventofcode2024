use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_antinodes(
    grid_size: (usize, usize),
    antennas: &[(usize, usize)],
) -> HashSet<(usize, usize)> {
    let max_lines: i64 = grid_size.0.try_into().unwrap();
    let max_columns: i64 = grid_size.1.try_into().unwrap();

    let mut res = HashSet::new();
    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i == j {
                continue;
            }

            let ll = antennas[i];
            let rr = antennas[j];
            let i_offset = ll.0 as i64 - rr.0 as i64;
            let j_offset = ll.1 as i64 - rr.1 as i64;

            let mut new_i = ll.0 as i64 + i_offset;
            let mut new_j = ll.1 as i64 + j_offset;
            while !(new_i < 0 || new_i > max_lines - 1 || new_j < 0 || new_j > max_columns - 1) {
                res.insert((new_i.try_into().unwrap(), new_j.try_into().unwrap()));
                new_i += i_offset;
                new_j += j_offset;
            }

            let mut new_i = ll.0 as i64 - i_offset;
            let mut new_j = ll.1 as i64 - j_offset;
            while !(new_i < 0 || new_i > max_lines - 1 || new_j < 0 || new_j > max_columns - 1) {
                res.insert((new_i.try_into().unwrap(), new_j.try_into().unwrap()));
                new_i -= i_offset;
                new_j -= j_offset;
            }
        }
    }
    res
}

fn parse(input: String) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let max_lines = lines.len();
    let max_columns = lines[0].len();
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch != '.' {
                map.entry(ch).or_default().push((i, j));
            }
        }
    }
    let mut all_antinodes: HashSet<(usize, usize)> = HashSet::new();
    for values in map.into_values() {
        let antinodes = get_antinodes((max_lines, max_columns), &values);
        all_antinodes.extend(antinodes);
    }

    // let lines: Vec<_> = input.lines().collect();
    // for (i, line) in lines.into_iter().enumerate() {
    //     for (j, ch) in line.bytes().enumerate() {
    //         if all_antinodes.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!("{}", ch as char);
    //         }
    //     }
    //     println!();
    // }

    all_antinodes.len().try_into().unwrap()
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
