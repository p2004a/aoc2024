use itertools::Itertools;
use std::io;

fn prng(mut n: u64) -> u64 {
    let m = (1 << 24) - 1;
    n ^= (n << 6) & m;
    n ^= n >> 5;
    n ^= (n << 11) & m;
    n
}

fn main() {
    println!(
        "{}",
        io::stdin()
            .lines()
            .flat_map(|line| {
                let secret = line.unwrap().parse().unwrap();
                (0..2000)
                    // build prices list
                    .scan(secret, |n, _| {
                        let old_n = *n;
                        *n = prng(*n);
                        Some((old_n % 10) as i8)
                    })
                    // compute (change, price_after)
                    .tuple_windows()
                    .map(|(a, b)| (b - a, b as u64))
                    // build grouping pattern -> costs
                    .tuple_windows()
                    .map(|(p1, p2, p3, p4)| ((p1.0, p2.0, p3.0, p4.0), p4.1))
                    .into_grouping_map()
                    // take first cost for each pattern
                    .reduce(|a, _key, _b| a)
                    .into_iter()
            })
            .into_grouping_map()
            .sum()
            .into_values()
            .max()
            .unwrap()
    );
}
