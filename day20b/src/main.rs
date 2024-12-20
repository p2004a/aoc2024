mod grid;
use enum_iterator::{all, Sequence};
use grid::{Grid, Pos};
use std::{collections::VecDeque, io, iter};

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

fn circle(size: i32) -> impl Iterator<Item = (i32, i32)> {
    let mut x = -size;
    let mut y = 0;
    iter::from_fn(move || {
        while x <= size {
            while y <= (size - x.abs()) {
                let ry = y;
                y += 1;
                if x != 0 || ry != 0 {
                    return Some((x, ry));
                }
            }
            x += 1;
            y = -(size - x.abs());
        }
        None
    })
}

fn distance(a: Pos, b: Pos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn bfs(grid: &Grid<u8>, start: Pos) -> Grid<i32> {
    let mut dist = Grid::new(grid.width(), grid.height(), std::i32::MAX);
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
            .neighbours(circle(20), pos)
            .filter(|npos| grid[*npos] != b'#')
        {
            let new_dist = start_dist[pos] + end_dist[npos] + distance(pos, npos);
            if new_dist <= full_dist - 100 {
                count += 1;
            }
        }
    }
    println!("{count}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circle_test() {
        let mut v: Vec<(i32, i32)> = circle(2).collect();
        let mut expected = vec![
            (-2, 0),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -2),
            (0, -1),
            (0, 1),
            (0, 2),
            (1, -1),
            (1, 0),
            (1, 1),
            (2, 0),
        ];
        v.sort();
        expected.sort();
        assert_eq!(v, expected);
    }
}
