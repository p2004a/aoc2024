use std::io;

fn main() {
    let mut l1: Vec<i64> = Vec::new();
    let mut l2: Vec<i64> = Vec::new();
    let mut line = String::new();
    while io::stdin().read_line(&mut line).unwrap() > 0 {
        let mut iter = line.split_whitespace();
        l1.push(iter.next().unwrap().parse().unwrap());
        l2.push(iter.next().unwrap().parse().unwrap());
        line.clear();
    }
    l1.sort();
    l2.sort();
    let mut dist: i64 = 0;
    for i in 0..l1.len() {
        dist += (l1[i] - l2[i]).abs();
    }
    println!("{}", dist);
}
