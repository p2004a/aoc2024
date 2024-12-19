use regex::Regex;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let patterns: Vec<&str> = line.trim().split(", ").collect();
    let re_str = format!("^({})+$", patterns.join("|"));
    let re = Regex::new(&re_str).unwrap();
    let res = io::stdin()
        .lines()
        .filter(|l| re.is_match(l.as_ref().unwrap().as_str()))
        .count();
    println!("{res}");
}
