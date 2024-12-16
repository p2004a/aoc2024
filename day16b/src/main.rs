use enum_map::{Enum, EnumMap};
use std::cmp::min;
use std::collections::binary_heap::BinaryHeap;
use std::collections::{HashSet, VecDeque};
use std::io;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Enum, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    East,
    West,
    North,
    South,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<T> Grid<T> {
    fn copy<F, Y>(&self, mut f: F) -> Grid<F::Output>
    where
        F: FnMut() -> Y,
    {
        Grid(
            self.0
                .iter()
                .map(|v| v.iter().map(|_| f()).collect())
                .collect(),
        )
    }
}

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

fn good_seats(grid: &Grid<u8>, start: Pos) -> usize {
    let mut dist: Grid<EnumMap<Dir, u32>> = grid.copy(|| EnumMap::from_fn(|_| std::u32::MAX));
    let mut prev: Grid<EnumMap<Dir, Vec<(Pos, Dir)>>> =
        grid.copy(|| EnumMap::from_fn(|_| Vec::new()));

    let mut queue = BinaryHeap::new();
    dist[start][Dir::East] = 0;
    queue.push((0, start, Dir::East));

    let mut end_cost = std::u32::MAX;
    let mut end_pos = start;
    while let Some((cost, pos, dir)) = queue.pop() {
        if grid[pos] == b'E' {
            end_pos = pos;
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
            if grid[next_pos] != b'#' {
                if next_cost < dist[next_pos][next_dir] {
                    dist[next_pos][next_dir] = next_cost;
                    queue.push((next_cost, next_pos, next_dir));
                    prev[next_pos][next_dir].clear();
                }
                if next_cost <= dist[next_pos][next_dir] {
                    prev[next_pos][next_dir].push((pos, dir));
                }
            }
        }
    }

    assert!(end_cost < std::u32::MAX);

    let mut prev_queue: VecDeque<(Pos, Dir)> = VecDeque::new();
    let mut visited: HashSet<(Pos, Dir)> = HashSet::new();

    for (dir, cost) in dist[end_pos] {
        if cost == end_cost {
            prev_queue.push_back((end_pos, dir));
            visited.insert((end_pos, dir));
        }
    }

    while let Some((pos, dir)) = prev_queue.pop_front() {
        for pd in prev[pos][dir].iter() {
            if !visited.contains(&pd) {
                visited.insert(*pd);
                prev_queue.push_back(*pd);
            }
        }
    }

    let unique_pos: HashSet<Pos> = HashSet::from_iter(visited.iter().map(|(pos, _dir)| *pos));
    unique_pos.len()
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
    let good_seats = good_seats(&grid, start);
    println!("{good_seats}");
}
