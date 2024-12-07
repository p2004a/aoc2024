use std::io;

fn can_eval(res: i64, vals: &[i64]) -> bool {
    if let Some((last, rest)) = vals.split_last() {
        let last = *last;
        return if res >= last {
            can_eval(res - last, rest)
        } else {
            false
        } || if res > 0 && res % last == 0 {
            can_eval(res / last, rest)
        } else {
            false
        };
    }
    res == 0
}

fn main() {
    let mut result = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let l: Vec<&str> = line.split(":").collect();
        let calibration: i64 = l[0].parse().unwrap();
        let vals: Vec<i64> = l[1].trim().split(" ").map(|x| x.parse().unwrap()).collect();
        if can_eval(calibration, &vals) {
            result += calibration;
        }
    }
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::can_eval;

    #[test]
    fn simple() {
        assert!(can_eval(0, &[]));
        assert!(can_eval(10, &[10]));
        assert!(can_eval(10, &[1, 10]));
        assert!(!can_eval(10, &[1, 8]));
        assert!(!can_eval(10, &[2, 10]));
    }
}
