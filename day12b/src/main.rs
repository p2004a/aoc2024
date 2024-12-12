use enum_map::{Enum, EnumMap};
use std::io;

#[derive(Debug, Enum, PartialEq, Clone, Copy)]
enum Dir {
    West,
    East,
    North,
    South,
}

fn dfs(
    map: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
    sides: &mut EnumMap<Dir, Vec<(usize, usize)>>,
    p: (usize, usize),
) -> u32 {
    visited[p.1][p.0] = true;
    let mut area = 1;
    for (np, side) in [
        ((p.0.wrapping_sub(1), p.1), Dir::West),
        ((p.0 + 1, p.1), Dir::East),
        ((p.0, p.1.wrapping_sub(1)), Dir::North),
        ((p.0, p.1 + 1), Dir::South),
    ] {
        if map.get(np.1).and_then(|r| r.get(np.0)) == Some(&map[p.1][p.0]) {
            if !visited[np.1][np.0] {
                area += dfs(map, visited, sides, np);
            }
        } else {
            sides[side].push(if side == Dir::North || side == Dir::South {
                (p.1, p.0)
            } else {
                p
            });
        }
    }
    area
}

fn count_sides(mut sides: EnumMap<Dir, Vec<(usize, usize)>>) -> u32 {
    for s in sides.values_mut() {
        s.sort();
    }
    let mut num_sides = 0;
    for s in sides.values() {
        let mut prev = (std::usize::MAX - 1, std::usize::MAX - 1);
        for p in s {
            if prev.0 != p.0 || prev.1 + 1 != p.1 {
                num_sides += 1;
            }
            prev = *p;
        }
    }
    num_sides
}

fn main() {
    let map: Vec<Vec<u8>> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| x.bytes().collect())
        .collect();
    let (mw, mh) = (map[0].len(), map.len());

    let mut visited: Vec<Vec<bool>> = Vec::new();
    visited.resize_with(mh, || {
        let mut v = Vec::new();
        v.resize(mw, false);
        v
    });
    let mut total_price = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited[y][x] {
                let mut sides = EnumMap::from_fn(|_| Vec::new());
                let area = dfs(&map, &mut visited, &mut sides, (x, y));
                total_price += area * count_sides(sides);
            }
        }
    }
    println!("{total_price}");
}
