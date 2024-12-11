use std::collections::HashMap;
use std::io;

fn num_digits(n: u64) -> u32 {
    let mut res = 1;
    let mut n = n;
    while n >= 10 {
        n /= 10;
        res += 1;
    }
    return res;
}

fn compute(cache: &mut HashMap<(u64, u32), u64>, n: u64, d: u32) -> u64 {
    if let Some(r) = cache.get(&(n, d)) {
        return *r;
    }
    if d == 0 {
        return 1;
    }
    let res = if n == 0 {
        compute(cache, 1, d - 1)
    } else if num_digits(n) % 2 == 0 {
        let div = 10u64.pow(num_digits(n) / 2);
        compute(cache, n / div, d - 1) + compute(cache, n % div, d - 1)
    } else {
        compute(cache, n * 2024, d - 1)
    };
    cache.insert((n, d), res);
    res
}

fn main() {
    let input: Vec<u64> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let depth = 75;
    let mut cache = HashMap::new();
    let res = input
        .iter()
        .map(|n| compute(&mut cache, *n, depth))
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{res}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digit() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(11), 2);
        assert_eq!(num_digits(1001), 4);
    }
}
