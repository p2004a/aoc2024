use enum_map::{Enum, EnumMap};
use std::cmp::min;
use std::collections::binary_heap::BinaryHeap;
use std::io;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Enum, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    East,
    West,
    North,
    South,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn adv(self, dir: Dir) -> Pos {
        let d = match dir {
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
            Dir::North => (0, -1),
            Dir::South => (0, 1),
        };
        Pos {
            x: (self.x as isize + d.0) as usize,
            y: (self.y as isize + d.1) as usize,
        }
    }
}

struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

fn find_start(grid: &Grid<u8>) -> Option<Pos> {
    for y in 0..grid.0.len() {
        for x in 0..grid.0[y].len() {
            let pos = Pos { x, y };
            if grid[pos] == b'S' {
                return Some(pos);
            }
        }
    }
    None
}

fn find_path(grid: &Grid<u8>, start: Pos) -> Option<u32> {
    let mut dist: Grid<EnumMap<Dir, u32>> = Grid(
        grid.0
            .iter()
            .map(|v| {
                v.iter()
                    .map(|_| EnumMap::from_fn(|_| std::u32::MAX))
                    .collect()
            })
            .collect(),
    );

    let mut queue: BinaryHeap<(u32, Pos, Dir)> = BinaryHeap::new();
    dist[start][Dir::East] = 0;
    queue.push((0, start, Dir::East));

    let mut end_cost = std::u32::MAX;

    while let Some((cost, pos, dir)) = queue.pop() {
        if grid[pos] == b'E' {
            end_cost = min(end_cost, cost);
            continue;
        }
        if cost > dist[pos][dir] {
            continue;
        }

        for next_dir in [Dir::East, Dir::West, Dir::South, Dir::North] {
            let edge_cost = if next_dir == dir { 1 } else { 1001 };
            let next_pos = pos.adv(next_dir);
            let next_cost = cost + edge_cost;
            if grid[next_pos] != b'#' && next_cost < dist[next_pos][next_dir] {
                dist[next_pos][next_dir] = next_cost;
                queue.push((next_cost, next_pos, next_dir));
            }
        }
    }

    if end_cost < std::u32::MAX {
        Some(end_cost)
    } else {
        None
    }
}

fn main() {
    let grid: Grid<u8> = Grid(
        io::read_to_string(io::stdin())
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|v| v.bytes().collect())
            .collect(),
    );
    let start = find_start(&grid).unwrap();
    let path_cost = find_path(&grid, start).unwrap();
    println!("{path_cost}");
}
