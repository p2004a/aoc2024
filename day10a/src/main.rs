use std::io;

fn add_dir(map: &Vec<Vec<u8>>, pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    let (mw, mh) = (map[0].len(), map.len());
    let out = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    if 0 <= out.0 && out.0 < mw as isize && 0 <= out.1 && out.1 < mh as isize {
        Some((out.0 as usize, out.1 as usize))
    } else {
        None
    }
}

fn dfs(map: &Vec<Vec<u8>>, visited: &mut Vec<bool>, pos: (usize, usize), d: usize) -> u32 {
    let mw = map[0].len();
    visited[pos.0 + pos.1 * mw] = true;
    let h = map[pos.1][pos.0];
    // println!("{}{} {}: {}", "  ".repeat(d), pos.0, pos.1, h);
    if h == 9 {
        return 1;
    }
    let mut res = 0;
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let Some(next_pos) = add_dir(map, pos, dir) {
            if map[next_pos.1][next_pos.0] == h + 1 && !visited[next_pos.0 + next_pos.1 * mw] {
                res += dfs(map, visited, next_pos, d + 1);
            }
        }
    }
    res
}

fn num_reachable(map: &Vec<Vec<u8>>, start: (usize, usize)) -> u32 {
    let (mw, mh) = (map[0].len(), map.len());
    let mut visited: Vec<bool> = Vec::new();
    visited.resize(mw * mh, false);
    return dfs(&map, &mut visited, start, 0);
}

fn main() {
    let map: Vec<Vec<u8>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().as_bytes().iter().map(|c| (c - b'0')).collect())
        .collect();
    let mut res = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if *h == 0 {
                // println!("start:");
                res += num_reachable(&map, (x, y));
            }
        }
    }
    println!("{res}");
}
