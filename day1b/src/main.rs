use std::{collections::HashMap, io};

fn main() {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for line in io::stdin().lines() {
        let res: Vec<i64> = line
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        l1.push(res[0]);
        l2.push(res[1]);
    }

    let mut freq: HashMap<i64, i64> = HashMap::new();
    for v in l2 {
        let f = *freq.get(&v).unwrap_or(&0);
        freq.insert(v, f+1);
    }

    let mut dist: i64 = 0;
    for v in l1 {
        dist += v * (*freq.get(&v).unwrap_or(&0));
    }

    println!("{}", dist);
}
