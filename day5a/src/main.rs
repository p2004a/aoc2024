use std::collections::{BTreeSet, HashMap};
use std::io;

type Rules = HashMap<i32, BTreeSet<i32>>;

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
        rules.entry(l[0]).or_insert(BTreeSet::new()).insert(l[1]);
    }
    return rules;
}

fn is_valid_order(rules: &Rules, pages: &[i32]) -> bool {
    let mut prev_pages = BTreeSet::new();
    for p in pages {
        if let Some(rule) = rules.get(p) {
            if !rule.is_disjoint(&prev_pages) {
                return false;
            }
        }
        prev_pages.insert(*p);
    }
    true
}

fn main() {
    let rules = read_rules();
    let mut result = 0;
    for line in io::stdin().lines() {
        let pages = line
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        if is_valid_order(&rules, &pages) {
            result += pages[pages.len() / 2];
        }
    }
    println!("{}", result);
}
