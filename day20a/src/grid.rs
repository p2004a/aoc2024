use std::{
    fmt::Display,
    iter::FusedIterator,
    ops::{Index, IndexMut},
};

pub struct Grid<T> {
    width: i32,
    height: i32,
    elems: Vec<T>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    #[allow(dead_code)]
    pub fn from<I>(t: (I, I)) -> Pos
    where
        I: Into<i32>,
    {
        Pos {
            x: t.0.into(),
            y: t.1.into(),
        }
    }

    #[allow(dead_code)]
    pub fn adv<D: Dir>(&self, d: D) -> Pos {
        let v = d.vector();
        Pos {
            x: self.x + v.0,
            y: self.y + v.1,
        }
    }
}

pub struct Neighbours<'a, T, DIt> {
    grid: &'a Grid<T>,
    di: DIt,
    pos: Pos,
}

impl<'a, T, D, DIt> Iterator for Neighbours<'a, T, DIt>
where
    D: Dir,
    DIt: Iterator<Item = D>,
{
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(d) = self.di.next() {
            if let Some(p) = self.grid.adv(self.pos, d) {
                return Some(p);
            }
        }
        None
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    y: i32,
    x: i32,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        while self.y < self.grid.height {
            while self.x < self.grid.width {
                let x = self.x;
                self.x += 1;
                return Some(Pos { x, y: self.y });
            }
            self.x = 0;
            self.y += 1;
        }
        None
    }
}

impl<'a, T> FusedIterator for GridIter<'a, T> {}

impl<T> Grid<T>
where
    T: Copy + Clone,
{
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, initial: T) -> Grid<T> {
        Grid {
            width: width as i32,
            height: height as i32,
            elems: vec![initial; width * height],
        }
    }
}

impl<T> Grid<T> {
    #[allow(dead_code)]
    pub fn new_with<F>(width: usize, height: usize, mut f: F) -> Grid<T>
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut elems = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                elems.push(f(x, y));
            }
        }
        Grid {
            width: width as i32,
            height: height as i32,
            elems,
        }
    }

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.height as usize
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.height as usize
    }

    #[allow(dead_code)]
    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid: &self,
            y: 0,
            x: 0,
        }
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.in_bounds(pos)
            .then_some(&self.elems[(pos.x + pos.y * self.width) as usize])
    }

    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        self.in_bounds(pos)
            .then_some(&mut self.elems[(pos.x + pos.y * self.width) as usize])
    }

    pub fn adv<D: Dir>(&self, pos: Pos, d: D) -> Option<Pos> {
        let v = d.vector();
        let npos = Pos {
            x: pos.x + v.0,
            y: pos.y + v.1,
        };
        self.in_bounds(npos).then_some(npos)
    }

    #[allow(dead_code)]
    pub fn neighbours<'a, Dit>(&'a self, di: Dit, pos: Pos) -> Neighbours<'a, T, Dit> {
        Neighbours {
            grid: self,
            di: di,
            pos,
        }
    }
}

impl Grid<u8> {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Grid<u8> {
        let rows: Vec<&[u8]> = s.trim().split("\n").map(|v| v.as_bytes()).collect();
        Grid::new_with(rows[0].len(), rows.len(), |x, y| rows[y][x])
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        self.get(pos).unwrap()
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        self.get_mut(pos).unwrap()
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pos in self.iter() {
            let c = char::from_u32(self[pos] as u32).ok_or(std::fmt::Error)?;
            if pos.x == 0 && pos.y != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

pub trait Dir {
    fn vector(&self) -> (i32, i32);
}
