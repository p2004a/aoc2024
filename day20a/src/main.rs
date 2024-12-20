mod grid;
use enum_iterator::{all, Sequence};
use grid::{Grid, Pos};
use std::{collections::VecDeque, io};

#[derive(Sequence)]
enum Dir {
    West,
    East,
    North,
    South,
}

impl grid::Dir for Dir {
    fn vector(&self) -> (i32, i32) {
        match self {
            Dir::West => (-1, 0),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::North => (0, -1),
        }
    }
}

#[inline]
fn add_vec(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

#[derive(Sequence)]
enum SkipDir {
    EE,
    NE,
    NN,
    NW,
    SE,
    SS,
    SW,
    WW,
}

impl grid::Dir for SkipDir {
    fn vector(&self) -> (i32, i32) {
        match self {
            SkipDir::EE => add_vec(Dir::East.vector(), Dir::East.vector()),
            SkipDir::NE => add_vec(Dir::North.vector(), Dir::East.vector()),
            SkipDir::NN => add_vec(Dir::North.vector(), Dir::North.vector()),
            SkipDir::NW => add_vec(Dir::North.vector(), Dir::West.vector()),
            SkipDir::SE => add_vec(Dir::South.vector(), Dir::East.vector()),
            SkipDir::SS => add_vec(Dir::South.vector(), Dir::South.vector()),
            SkipDir::SW => add_vec(Dir::South.vector(), Dir::West.vector()),
            SkipDir::WW => add_vec(Dir::West.vector(), Dir::West.vector()),
        }
    }
}

fn bfs(grid: &Grid<u8>, start: Pos) -> Grid<u32> {
    let mut dist = Grid::new(grid.width(), grid.height(), std::u32::MAX);
    dist[start] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        for npos in grid.neighbours(all::<Dir>(), pos) {
            if grid[npos] != b'#' && dist[pos] + 1 < dist[npos] {
                dist[npos] = dist[pos] + 1;
                queue.push_back(npos);
            }
        }
    }

    dist
}

fn main() {
    let grid = Grid::from_str(io::read_to_string(io::stdin()).unwrap().as_str());
    let start = grid.iter().filter(|p| grid[*p] == b'S').next().unwrap();
    let end = grid.iter().filter(|p| grid[*p] == b'E').next().unwrap();
    let start_dist = bfs(&grid, start);
    let end_dist = bfs(&grid, end);

    let full_dist = start_dist[end];
    let mut count = 0;
    for pos in grid.iter().filter(|pos| grid[*pos] != b'#') {
        for npos in grid
            .neighbours(all::<SkipDir>(), pos)
            .filter(|npos| grid[*npos] != b'#')
        {
            let new_dist = start_dist[pos] + end_dist[npos] + 2;
            if new_dist <= full_dist - 100 {
                count += 1;
            }
        }
    }
    println!("{count}");
}
