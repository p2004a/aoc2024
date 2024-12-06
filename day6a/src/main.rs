use std::io;
use std::ops::{Index, IndexMut};

struct Grid {
    elems: Vec<u8>,
    width: isize,
    height: isize,
}

type GridPos = (isize, isize);

impl Grid {
    fn new(width: isize, height: isize) -> Self {
        let mut elems = Vec::new();
        elems.resize((width * height) as usize, 0);
        Grid {
            elems,
            width,
            height,
        }
    }

    fn in_bounds(&self, pos: GridPos) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    fn get(&self, pos: GridPos) -> Option<u8> {
        if self.in_bounds(pos) {
            return Some(self.elems[(pos.1 * self.width + pos.0) as usize]);
        }
        None
    }
}

impl Index<GridPos> for Grid {
    type Output = u8;

    fn index(&self, idx: GridPos) -> &Self::Output {
        assert!(self.in_bounds(idx));
        &self.elems[(idx.1 * self.width + idx.0) as usize]
    }
}

impl IndexMut<GridPos> for Grid {
    fn index_mut(&mut self, idx: GridPos) -> &mut Self::Output {
        assert!(self.in_bounds(idx));
        &mut self.elems[(idx.1 * self.width + idx.0) as usize]
    }
}

fn read_grid() -> io::Result<Grid> {
    let grid_str = io::read_to_string(io::stdin())?;
    let lines: Vec<&str> = grid_str.trim().split("\n").collect();
    let mut g = Grid::new(lines[0].len() as isize, lines.len() as isize);
    for (y, l) in lines.iter().enumerate() {
        assert!(l.len() == g.width as usize);
        for (x, b) in l.as_bytes().iter().enumerate() {
            g[(x as isize, y as isize)] = *b;
        }
    }
    Ok(g)
}

fn rot_right(dir: (isize, isize)) -> (isize, isize) {
    (-dir.1, dir.0)
}

fn move_pos(pos: GridPos, dir: (isize, isize)) -> GridPos {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn main() {
    let mut g = read_grid().unwrap();
    let mut pos = (0, 0);

    // Find start
    for y in 0..g.height {
        for x in 0..g.width {
            if g[(x, y)] == b'^' {
                pos = (x, y);
            }
        }
    }

    // Just naively simulate
    let mut dir = (0, -1); // up
    while g.in_bounds(pos) {
        while g.get(move_pos(pos, dir)) == Some(b'#') {
            dir = rot_right(dir);
        }
        g[pos] = b'X';
        pos = move_pos(pos, dir);
    }

    // Count number of visited positions
    let mut result = 0;
    for y in 0..g.height {
        for x in 0..g.width {
            if g[(x, y)] == b'X' {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
