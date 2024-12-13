use regex::Regex;
use std::io;

fn get_cost(a: (i64, i64), b: (i64, i64), r: (i64, i64)) -> Option<i64> {
    let a_cost = 3;
    let b_cost = 1;

    // Verify that the vector are not overlaping, if they are, it would be
    // an additional case to cover.
    assert!(a.0 * b.1 != b.0 * a.1);

    // Once we know they aren't overlapping, it's a solution over integers
    // for a set of 2 linear equations:
    //   a_num * a.0 + b_num * b.0 = r.0
    //   a_num * a.1 + b_num * b.1 = r.1

    let bn = r.0 * a.1 - r.1 * a.0;
    let bd = b.0 * a.1 - b.1 * a.0;
    if bn % bd != 0 {
        return None;
    }
    let b_num = bn / bd;

    let an = r.0 - b_num * b.0;
    let ad = a.0;
    if an % ad != 0 {
        return None;
    }
    let a_num = an / ad;

    Some(a_num * a_cost + b_num * b_cost)
}

fn main() {
    let input_re = Regex::new(r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)").unwrap();
    let mut total_price = 0;
    for entry in io::read_to_string(io::stdin()).unwrap().split("\n\n") {
        let c = input_re.captures(entry).unwrap();
        let ax: i64 = c["ax"].parse().unwrap();
        let ay: i64 = c["ay"].parse().unwrap();
        let bx: i64 = c["bx"].parse().unwrap();
        let by: i64 = c["by"].parse().unwrap();
        let px: i64 = c["px"].parse().unwrap();
        let py: i64 = c["py"].parse().unwrap();
        if let Some(price) = get_cost(
            (ax, ay),
            (bx, by),
            (px + 10000000000000, py + 10000000000000),
        ) {
            total_price += price;
        }
    }
    println!("{total_price}");
}
