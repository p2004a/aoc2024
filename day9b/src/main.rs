use std::io;

fn main() {
    let mut input: Vec<u64> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as u64)
        .collect();

    let mut off: Vec<u64> = input
        .iter()
        .scan(0, |s, v| {
            let r = *s;
            *s += v;
            Some(r)
        })
        .collect();

    let mut checksum: u64 = 0;
    for j in (0..input.len()).step_by(2).rev() {
        let mut pos = j;
        for i in (1..j).step_by(2) {
            if input[i] >= input[j] {
                pos = i;
                break;
            }
        }
        let prev_off = off[pos];
        off[pos] += input[j];
        checksum += (j as u64 / 2)
            * (off[pos] * off[pos].saturating_sub(1) - prev_off * prev_off.saturating_sub(1))
            / 2;
        input[pos] -= input[j];
    }
    println!("{checksum}");
}
