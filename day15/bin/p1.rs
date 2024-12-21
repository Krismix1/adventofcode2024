use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pieces {
    Wall,
    Box,
    Empty,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (usize, usize);

fn parse_input(input: &str) -> (Vec<Vec<Pieces>>, Position, Vec<Direction>) {
    let mut grid: Vec<String> = vec![];
    let mut parses_grid = true;
    let mut movements: Vec<Direction> = vec![];
    // for line in input.lines().skip(1) {
    for line in input.lines() {
        if line.is_empty() {
            parses_grid = false;
            continue;
        }
        if parses_grid {
            grid.push(line.to_string());
        } else {
            movements.extend(line.bytes().map(|el| match el {
                b'^' => Direction::Up,
                b'>' => Direction::Right,
                b'<' => Direction::Left,
                b'v' => Direction::Down,
                _ => panic!("Invalid direction {el}"),
            }));
        }
    }
    // grid.pop();

    let mut robot_position: Position = (0, 0);

    let mut new_grid = vec![vec![Pieces::Empty; grid[0].len()]; grid.len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, el) in row.bytes().enumerate() {
            match el {
                b'@' => {
                    robot_position = (i, j);
                }
                b'.' => {
                    continue;
                }
                b'#' => {
                    new_grid[i][j] = Pieces::Wall;
                }
                b'O' => {
                    new_grid[i][j] = Pieces::Box;
                }
                _ => {
                    panic!("Invalid {el} char");
                }
            }
        }
    }

    (new_grid, robot_position, movements)
}

fn execute_move(grid: &mut [Vec<Pieces>], position: Position, direction: Direction) -> Position {
    let vel: (isize, isize) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    let mut next_position = (
        position.0.checked_add_signed(vel.0).unwrap(),
        position.1.checked_add_signed(vel.1).unwrap(),
    );
    let mut next_piece = grid[next_position.0][next_position.1];
    if next_piece == Pieces::Empty {
        return next_position;
    }

    if next_piece == Pieces::Wall {
        return position;
    }

    let mut next_empty_space = None;
    while next_position.0 < grid.len() && next_position.1 < grid[0].len() {
        next_piece = grid[next_position.0][next_position.1];
        if next_piece == Pieces::Empty {
            next_empty_space = Some(next_position);
            break;
        }
        if next_piece == Pieces::Wall {
            break;
        }

        next_position = (
            next_position.0.checked_add_signed(vel.0).unwrap(),
            next_position.1.checked_add_signed(vel.1).unwrap(),
        );
    }
    let Some(next_empty_space) = next_empty_space else {
        return position;
    };

    let mut last_piece = grid[position.0][position.1];
    let (mut i, mut j) = position;
    while (i, j) != next_empty_space {
        let next_position = (
            i.checked_add_signed(vel.0).unwrap(),
            j.checked_add_signed(vel.1).unwrap(),
        );

        std::mem::swap(&mut grid[next_position.0][next_position.1], &mut last_piece);
        (i, j) = next_position;
    }

    (
        position.0.checked_add_signed(vel.0).unwrap(),
        position.1.checked_add_signed(vel.1).unwrap(),
    )
}

fn print_grid(grid: &[Vec<Pieces>], robot_position: Position) {
    for (i, line) in grid.iter().enumerate() {
        for (j, el) in line.iter().enumerate() {
            if (i, j) == robot_position {
                print!("@");
                continue;
            }
            match el {
                Pieces::Box => print!("O"),
                Pieces::Wall => print!("#"),
                Pieces::Empty => print!("."),
            }
        }
        println!();
    }
}

fn solve(input: String) -> u64 {
    let (mut grid, mut robot_position, movements) = parse_input(&input);

    for movement in movements.iter() {
        robot_position = execute_move(&mut grid, robot_position, *movement);
    }

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == Pieces::Box {
                sum += 100 * i + j;
            }
        }
    }

    sum.try_into().unwrap()
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
