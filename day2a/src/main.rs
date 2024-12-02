use std::io;

fn is_safe(l: &Vec<i64>) -> bool {
    let dir = (l[0] - l[1]).signum();
    for i in 1..l.len() {
        let diff = l[i-1] - l[i];
        if diff.signum() != dir || 1 > diff.abs() || diff.abs() > 3 {
            return false
        }
    }
    true
}

fn main() {
    let mut num_safe = 0;
    for line in io::stdin().lines() {
        let l: Vec<i64> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if is_safe(&l) {
            num_safe += 1;
        }
    }
    println!("{}", num_safe);
}
