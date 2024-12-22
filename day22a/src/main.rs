use std::io;

fn prng(mut n: u64) -> u64 {
    let m = (1 << 24) - 1;
    n ^= (n << 6) & m;
    n ^= n >> 5;
    n ^= (n << 11) & m;
    n
}

fn main() {
    let nums: Vec<u64> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split("\n")
        .map(|v| v.parse().unwrap())
        .collect();
    let res: u64 = nums
        .iter()
        .map(|v| (0..2000).fold(*v, |n, _| prng(n)))
        .sum();
    println!("{}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prng() {
        let seq: Vec<u64> = (0..10)
            .scan(123, |n, _| {
                *n = prng(*n);
                Some(*n)
            })
            .collect();
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        assert_eq!(seq, expected);
    }
}
