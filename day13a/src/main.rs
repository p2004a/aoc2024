use regex::Regex;
use std::cmp::min;
use std::io;

fn get_cost(a: (u32, u32), b: (u32, u32), end: (u32, u32)) -> Option<u32> {
    let a_cost = 3;
    let b_cost = 1;

    let mut res = None;

    let max_a = min(min(end.0 / a.0, end.1 / a.1), 100);
    for num_a in 0..=max_a {
        let p = (end.0 - a.0 * num_a, end.1 - a.1 * num_a);
        if p.0 % b.0 == 0 && p.1 % b.1 == 0 && p.0 / b.0 == p.1 / b.1 {
            let num_b = p.0 / b.0;
            let candidate = num_b * b_cost + num_a * a_cost;
            if candidate < res.unwrap_or(u32::MAX) {
                res = Some(candidate);
            }
        }
    }

    res
}

fn main() {
    let input_re = Regex::new(r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)").unwrap();
    let mut total_price = 0;
    for entry in io::read_to_string(io::stdin()).unwrap().split("\n\n") {
        let c = input_re.captures(entry).unwrap();
        let ax: u32 = c["ax"].parse().unwrap();
        let ay: u32 = c["ay"].parse().unwrap();
        let bx: u32 = c["bx"].parse().unwrap();
        let by: u32 = c["by"].parse().unwrap();
        let px: u32 = c["px"].parse().unwrap();
        let py: u32 = c["py"].parse().unwrap();
        if let Some(price) = get_cost((ax, ay), (bx, by), (px, py)) {
            total_price += price;
        }
    }
    println!("{total_price}");
}
