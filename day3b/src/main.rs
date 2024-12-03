use regex::Regex;
use std::io::{self, Read};

enum Op {
    Do,
    Dont,
    Mul(i64, i64),
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(mul)\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for op in re.captures_iter(&input).map(|c| {
        match c.get(1).or(c.get(4)).or(c.get(5)).unwrap().as_str() {
            "mul" => Op::Mul(c[2].parse().unwrap(), c[3].parse().unwrap()),
            "do" => Op::Do,
            "don't" => Op::Dont,
            _ => panic!("Unexpected value {}", &c[0]),
        }
    }) {
        match op {
            Op::Do => enabled = true,
            Op::Dont => enabled = false,
            Op::Mul(a, b) => {
                if enabled {
                    sum += a * b
                }
            }
        }
    }
    println!("{}", sum);
}
