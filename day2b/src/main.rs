use std::io;

fn is_safe(l: &Vec<i64>, ignore_idx: usize) -> bool {
    let mut prev = -1;
    let mut dir = 0;
    for i in 0..l.len() {
        if ignore_idx == i {
            continue;
        }
        if prev != -1 {
            let diff = prev - l[i];
            if dir == 0 {
                dir = diff.signum();
            }
            if diff.signum() != dir || 1 > diff.abs() || diff.abs() > 3 {
                return false;
            }
        }
        prev = l[i]
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
        if (0..l.len() + 1).any(|i| is_safe(&l, i)) {
            num_safe += 1
        }
    }
    println!("{}", num_safe);
}
