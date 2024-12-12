use std::io;

fn dfs(map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, p: (usize, usize)) -> (u32, u32) {
    if visited[p.1][p.0] {
        return (0, 0);
    }
    visited[p.1][p.0] = true;
    let mut area = 1;
    let mut perimeter = 0;
    for np in [
        (p.0.wrapping_sub(1), p.1),
        (p.0 + 1, p.1),
        (p.0, p.1.wrapping_sub(1)),
        (p.0, p.1 + 1),
    ] {
        if map.get(np.1).and_then(|r| r.get(np.0)) == Some(&map[p.1][p.0]) {
            let r = dfs(map, visited, np);
            perimeter += r.0;
            area += r.1;
        } else {
            perimeter += 1
        }
    }
    (perimeter, area)
}

fn main() {
    let map: Vec<Vec<u8>> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| x.bytes().collect())
        .collect();
    let (mw, mh) = (map[0].len(), map.len());

    let mut visited: Vec<Vec<bool>> = Vec::new();
    visited.resize_with(mh, || {
        let mut v = Vec::new();
        v.resize(mw, false);
        v
    });
    let mut total_price = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let (perimeter, area) = dfs(&map, &mut visited, (x, y));
            total_price += perimeter * area;
        }
    }
    println!("{total_price}");
}
