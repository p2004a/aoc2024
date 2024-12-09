use std::cmp;
use std::io;

fn main() {
    let mut input: Vec<u64> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as u64)
        .collect();

    let mut checksum: u64 = 0;
    let mut i = 0;
    let mut j = input.len() - 1;
    if j % 2 == 1 {
        j -= 1;
    }
    let mut off = 0;
    while i <= j {
        let next_off = off + input[i];
        checksum += (i as u64 / 2)
            * (next_off * next_off.saturating_sub(1) - off * off.saturating_sub(1))
            / 2;
        off = next_off;

        i += 1;
        while i < j && input[i] > 0 {
            let d = cmp::min(input[i], input[j]);
            input[i] -= d;
            input[j] -= d;

            let next_off = off + d;
            checksum += (j as u64 / 2)
                * (next_off * next_off.saturating_sub(1) - off * off.saturating_sub(1))
                / 2;
            off = next_off;

            if input[j] == 0 {
                j -= 2;
            }
        }
        i += 1;
    }
    println!("{checksum}");
}
