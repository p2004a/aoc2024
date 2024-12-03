use std::io::{self, Read};
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for [a, b] in re.captures_iter(&input).map(|c| { c.extract().1 }) {
        let a: i64 = a.parse().unwrap();
        let b: i64 = b.parse().unwrap();
        sum += a * b;
    }
    println!("{}", sum);
}
