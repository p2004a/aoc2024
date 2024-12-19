use std::cmp::min;
use std::io;

fn num_matches(patterns: &[&[u8]], design: &[u8]) -> u64 {
    if design.len() < 1 {
        return 0;
    }

    let mut combi = Vec::new();
    combi.resize(design.len() + 1, 0);
    combi[design.len()] = 1;

    for i in (0..design.len()).rev() {
        for p in patterns {
            if design[i..min(i + p.len(), design.len())] == **p {
                combi[i] += combi[i + p.len()];
            }
        }
    }

    combi[0]
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let patterns: Vec<&[u8]> = line.trim().split(", ").map(|s| s.as_bytes()).collect();
    let res = io::stdin()
        .lines()
        .map(|l| num_matches(patterns.as_slice(), l.as_ref().unwrap().as_bytes()))
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{res}");
}
