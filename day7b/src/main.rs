use std::io;

fn digit_multipler(val: i64) -> i64 {
    let mut multiplier = 10;
    while multiplier <= val {
        multiplier *= 10;
    }
    multiplier
}

fn can_eval(res: i64, vals: &[(i64, i64)]) -> bool {
    if let Some(((last, dm), rest)) = vals.split_last() {
        let (last, dm) = (*last, *dm);
        return (res >= last && can_eval(res - last, rest))
            || (res > 0 && res % last == 0 && can_eval(res / last, rest))
            || (res > dm && res % dm == last && can_eval(res / dm, rest));
    }
    res == 0
}

fn main() {
    let mut result = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let l: Vec<&str> = line.split(":").collect();
        let calibration: i64 = l[0].parse().unwrap();
        let vals: Vec<(i64, i64)> = l[1]
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .map(|v| (v, digit_multipler(v)))
            .collect();
        if can_eval(calibration, &vals) {
            result += calibration;
        }
    }
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        assert!(can_eval(0, &[]));
        assert!(can_eval(10, &[(10, 100)]));
        assert!(can_eval(10, &[(1, 10), (10, 100)]));
        assert!(!can_eval(10, &[(1, 10), (8, 10)]));
        assert!(!can_eval(10, &[(2, 10), (10, 100)]));
        assert!(can_eval(156, &[(15, 100), (6, 10)]));
    }

    #[test]
    fn dm() {
        assert_eq!(digit_multipler(1), 10);
        assert_eq!(digit_multipler(100), 1000);
        assert_eq!(digit_multipler(99), 100);
    }
}
