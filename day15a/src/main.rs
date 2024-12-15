use itertools::Itertools;
use std::io::{self, Write};

type Pos = (usize, usize);

fn find_robot(grid: &Vec<Vec<u8>>) -> Option<Pos> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'@' {
                return Some((x, y));
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    East,
    West,
    North,
    South,
}

impl Dir {
    fn from_char(c: char) -> Option<Dir> {
        match c {
            '<' => Some(Dir::West),
            '>' => Some(Dir::East),
            '^' => Some(Dir::North),
            'v' => Some(Dir::South),
            _ => None,
        }
    }

    // We take adventage of the fact that grid in this task is very nice
    // and has border so we can use simple function like this 🎉.
    fn adv(&self, pos: Pos) -> Pos {
        let d = match self {
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
            Dir::North => (0, -1),
            Dir::South => (0, 1),
        };
        (
            (pos.0 as isize + d.0) as usize,
            (pos.1 as isize + d.1) as usize,
        )
    }
}

fn try_move(grid: &mut Vec<Vec<u8>>, pos: Pos, dir: Dir) -> bool {
    if grid[pos.1][pos.0] == b'.' {
        return true;
    } else if grid[pos.1][pos.0] == b'#' {
        return false;
    }

    let next_pos = dir.adv(pos);
    if try_move(grid, next_pos, dir) {
        grid[next_pos.1][next_pos.0] = grid[pos.1][pos.0];
        grid[pos.1][pos.0] = b'.';
        true
    } else {
        false
    }
}

fn print_grid(grid: &Vec<Vec<u8>>) -> io::Result<()> {
    let mut out = io::stdout().lock();
    for row in grid {
        out.write_all(row)?;
        out.write_all(&[b'\n'])?;
    }
    Ok(())
}

fn compute_result(grid: &Vec<Vec<u8>>) -> usize {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'O' {
                result += y * 100 + x;
            }
        }
    }
    result
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let (grid_str, steps_str) = input.split("\n\n").collect_tuple().unwrap();

    let mut grid: Vec<Vec<u8>> = grid_str
        .trim()
        .split_whitespace()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let steps: Vec<Dir> = steps_str.chars().filter_map(Dir::from_char).collect();

    let mut pos = find_robot(&grid).unwrap();
    for s in steps {
        if try_move(&mut grid, pos, s) {
            pos = s.adv(pos)
        }
    }

    println!("{}", compute_result(&grid));
}
