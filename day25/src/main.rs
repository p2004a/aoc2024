use std::io;

fn key_matches_lock(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();
    for entry in input.trim().split("\n\n") {
        let kl = entry.as_bytes();
        let mut heights: [u8; 5] = [0; 5];
        for x in 0..5 {
            heights[x] = (1..=5)
                .map(|y| kl[y * 6 + x])
                .filter(|b| *b == b'#')
                .count() as u8;
        }
        if entry.as_bytes()[0] == b'#' {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut res = 0;
    for k in keys.iter() {
        for l in locks.iter() {
            if key_matches_lock(k, l) {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
