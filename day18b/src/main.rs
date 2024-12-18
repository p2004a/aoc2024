use itertools::Itertools;
use std::{cmp::max, io};

// Union-find Forest
struct FU {
    size: Vec<usize>,
    parent: Vec<usize>,
}

impl FU {
    fn new(num_sets: usize) -> FU {
        let mut size = Vec::with_capacity(num_sets);
        let mut parent = Vec::with_capacity(num_sets);
        for n in 0..num_sets {
            size.push(1);
            parent.push(n);
        }
        FU { size, parent }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.parent[n] != n {
            self.parent[n] = self.find(self.parent[n]);
            self.parent[n]
        } else {
            n
        }
    }

    fn union(&mut self, mut a: usize, mut b: usize) {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            return;
        }
        if self.size[a] < self.size[b] {
            (a, b) = (b, a)
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
    }
}

struct Neighbours {
    size: i32,
    x: i32,
    y: i32,
    id: usize,
}

fn neighbours(size: usize, pos: usize) -> Neighbours {
    Neighbours {
        size: size as i32,
        x: (pos % size) as i32,
        y: (pos / size) as i32,
        id: 0,
    }
}

impl Iterator for Neighbours {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.id..4 {
            let n = match i {
                0 => (self.x + 1, self.y),
                1 => (self.x - 1, self.y),
                2 => (self.x, self.y + 1),
                3 => (self.x, self.y - 1),
                _ => panic!("impossible"),
            };
            if n.0 >= 0 && n.0 < self.size && n.1 >= 0 && n.1 < self.size {
                self.id = i + 1;
                return Some((n.1 * self.size + n.0) as usize);
            }
        }
        None
    }
}

fn main() {
    let input: Vec<(usize, usize)> = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    // That should be fine assumption
    let size = input.iter().map(|(a, b)| max(a, b)).reduce(max).unwrap() + 1;

    let mut blocked = Vec::new();
    blocked.resize(size * size, false);

    for s in input.iter() {
        let pos = s.1 * size + s.0;
        assert!(!blocked[pos]);
        blocked[pos] = true;
    }

    let mut fu = FU::new(size * size);

    for pos in 0..(size * size) {
        if !blocked[pos] {
            for npos in neighbours(size, pos) {
                if !blocked[npos] {
                    fu.union(pos, npos);
                }
            }
        }
    }

    for s in input.iter().rev() {
        let pos = s.1 * size + s.0;
        blocked[pos] = false;
        for npos in neighbours(size, pos) {
            if !blocked[npos] {
                fu.union(pos, npos);
            }
        }

        if fu.find(0) == fu.find(size * size - 1) {
            println!("{},{}", s.0, s.1);
            break;
        }
    }
}
