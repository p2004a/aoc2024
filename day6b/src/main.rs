use std::io;
use std::ops::{Index, IndexMut};

struct Grid<T> {
    elems: Vec<T>,
    width: isize,
    height: isize,
}

type GridPos = (isize, isize);

impl<T: Copy> Grid<T> {
    fn new(width: isize, height: isize, initial: T) -> Self {
        let mut elems = Vec::new();
        elems.resize((width * height) as usize, initial);
        Grid {
            elems,
            width,
            height,
        }
    }

    fn in_bounds(&self, pos: GridPos) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    fn get(&self, pos: GridPos) -> Option<T> {
        if self.in_bounds(pos) {
            return Some(self.elems[(pos.1 * self.width + pos.0) as usize]);
        }
        None
    }
}

impl<T: Copy> Index<GridPos> for Grid<T> {
    type Output = T;

    fn index(&self, idx: GridPos) -> &Self::Output {
        assert!(self.in_bounds(idx));
        &self.elems[(idx.1 * self.width + idx.0) as usize]
    }
}

impl<T: Copy> IndexMut<GridPos> for Grid<T> {
    fn index_mut(&mut self, idx: GridPos) -> &mut Self::Output {
        assert!(self.in_bounds(idx));
        &mut self.elems[(idx.1 * self.width + idx.0) as usize]
    }
}

fn read_grid() -> io::Result<Grid<u8>> {
    let grid_str = io::read_to_string(io::stdin())?;
    let lines: Vec<&str> = grid_str.trim().split("\n").collect();
    let mut g = Grid::new(lines[0].len() as isize, lines.len() as isize, b' ');
    for (y, l) in lines.iter().enumerate() {
        assert!(l.len() == g.width as usize);
        for (x, b) in l.as_bytes().iter().enumerate() {
            g[(x as isize, y as isize)] = *b;
        }
    }
    Ok(g)
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn rot_right(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn bit(self) -> u8 {
        match self {
            Dir::Up => 1 << 0,
            Dir::Right => 1 << 1,
            Dir::Down => 1 << 2,
            Dir::Left => 1 << 3,
        }
    }
}

#[derive(Clone, Copy)]
struct DirSet(u8);

impl DirSet {
    fn new() -> Self {
        DirSet(0)
    }

    fn set(&mut self, dir: Dir) {
        self.0 = self.0 | dir.bit();
    }

    fn has(self, dir: Dir) -> bool {
        self.0 & dir.bit() != 0
    }
}

fn move_pos(pos: GridPos, dir: Dir) -> GridPos {
    let change = match dir {
        Dir::Up => (0, -1),
        Dir::Right => (1, 0),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
    };
    (pos.0 + change.0, pos.1 + change.1)
}

fn has_loop(grid: &Grid<u8>, start: GridPos) -> bool {
    let mut pos = start;
    let mut dir = Dir::Up;
    let mut visited = Grid::new(grid.width, grid.height, DirSet::new());
    while grid.in_bounds(pos) {
        while grid.get(move_pos(pos, dir)) == Some(b'#') {
            dir = dir.rot_right();
        }
        if visited[pos].has(dir) {
            return true;
        }
        visited[pos].set(dir);
        pos = move_pos(pos, dir);
    }
    false
}

fn main() {
    let mut g = read_grid().unwrap();

    // Find start
    let mut start_pos = (0, 0);
    for y in 0..g.height {
        for x in 0..g.width {
            if g[(x, y)] == b'^' {
                start_pos = (x, y);
            }
        }
    }

    let mut result = 0;
    for y in 0..g.height {
        for x in 0..g.width {
            let obstacle = (x, y);
            if g[obstacle] != b'.' {
                continue;
            }
            g[obstacle] = b'#';
            if has_loop(&g, start_pos) {
                result += 1;
            }
            g[obstacle] = b'.';
        }
    }
    println!("{}", result);
}
