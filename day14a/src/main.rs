use regex::Regex;
use std::io;

const W: i64 = 101;
const H: i64 = 103;

struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn simulate_steps(r: &Robot, steps: i64) -> Robot {
    Robot {
        p: (
            (r.p.0 + r.v.0 * steps % W + W) % W,
            (r.p.1 + r.v.1 * steps % H + H) % H,
        ),
        v: r.v,
    }
}

fn quadrant(r: &Robot) -> Option<usize> {
    if r.p.0 == W / 2 || r.p.1 == H / 2 {
        return None;
    }
    let xq = if r.p.0 < W / 2 { 0 } else { 1 };
    let yq = if r.p.1 < H / 2 { 0 } else { 2 };
    Some(xq + yq)
}

fn main() {
    let mut robots: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>\-?\d+),(?<vy>\-?\d+)").unwrap();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let c = re.captures(line.as_str()).unwrap();
        robots.push(Robot {
            p: (c["px"].parse().unwrap(), c["py"].parse().unwrap()),
            v: (c["vx"].parse().unwrap(), c["vy"].parse().unwrap()),
        });
    }

    let mut q: [u64; 4] = [0, 0, 0, 0];
    for r in robots.iter() {
        let new_r = simulate_steps(&r, 100);
        if let Some(qn) = quadrant(&new_r) {
            q[qn] += 1;
        }
    }
    dbg!(q);
    let res = q.into_iter().reduce(|a, b| a * b).unwrap();
    println!("{res}");
}
