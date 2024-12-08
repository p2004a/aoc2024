use std::io;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut antenas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let (mut w, mut h) = (0, 0);
    for (y, line) in io::stdin().lines().enumerate() {
        w = 0;
        for (x, c) in line.unwrap().as_bytes().iter().enumerate() {
            if *c != b'.' {
                antenas.entry(*c).or_insert_with(|| Vec::new()).push((x as i32, y as i32));
            }
            w += 1;
        }
        h += 1;
    }

    let in_bounds = |(x, y)| 0 <= x && x < w && 0 <= y && y < h;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for same_antenas in antenas.values() {
        for a1 in same_antenas {
            for a2 in same_antenas {
                if a1 == a2 {
                    continue;
                }
                let diff = (a2.0 - a1.0, a2.1 - a1.1);
                let mut an = *a2;
                while in_bounds(an) {
                    antinodes.insert(an);
                    an = (an.0 + diff.0, an.1 + diff.1);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
