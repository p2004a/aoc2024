use std::collections::{HashMap, HashSet};
use std::io;

type Rules = HashMap<i32, HashSet<i32>>;

fn read_rules() -> Rules {
    let mut rules = HashMap::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        let l = line
            .split("|")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        rules.entry(l[0]).or_insert(HashSet::new()).insert(l[1]);
    }
    return rules;
}

fn valid_pages_order(rules: &Rules, p1: i32, p2: i32) -> bool {
    if let Some(rule) = rules.get(&p2) {
        return !rule.contains(&p1);
    }
    true
}

fn sort_pages(rules: &Rules, pages: &mut [i32]) -> bool {
    let mut changed = false;
    for i in 1..pages.len() {
        for j in 0..i {
            if !valid_pages_order(rules, pages[j], pages[i]) {
                pages[j..=i].rotate_right(1);
                changed = true;
                break;
            }
        }
    }
    changed
}

fn main() {
    let rules = read_rules();
    let mut result = 0;
    for line in io::stdin().lines() {
        let mut pages = line
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        if sort_pages(&rules, &mut pages) {
            result += pages[pages.len() / 2];
        }
    }
    println!("{}", result);
}
