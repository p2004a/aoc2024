use std::io;

fn dfs(map: &Vec<Vec<u8>>, paths: &mut Vec<i32>, p: (usize, usize)) -> i32 {
    let h = map[p.1][p.0];
    if h == 9 {
        return 1;
    }
    let pi = p.0 + p.1 * map[0].len();
    if paths[pi] >= 0 {
        return paths[pi];
    }
    let mut res = 0;
    for np in [
        (p.0.wrapping_sub(1), p.1),
        (p.0 + 1, p.1),
        (p.0, p.1.wrapping_sub(1)),
        (p.0, p.1 + 1),
    ] {
        if *map.get(np.1).and_then(|r| r.get(np.0)).unwrap_or(&100) == h + 1 {
            res += dfs(map, paths, np);
        }
    }
    paths[pi] = res;
    res
}

fn main() {
    let map: Vec<Vec<u8>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().as_bytes().iter().map(|c| (c - b'0')).collect())
        .collect();
    let mut res = 0;
    let mut paths: Vec<i32> = Vec::new();
    paths.resize(map[0].len() * map.len(), -1);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                res += dfs(&map, &mut paths, (x, y));
            }
        }
    }
    println!("{res}");
}
