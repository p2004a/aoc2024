use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io;

fn cliques<F>(
    edges: &Vec<Vec<usize>>,
    edges_set: &HashSet<(usize, usize)>,
    start: usize,
    size: usize,
    mut f: F,
) where
    F: FnMut(&[usize]),
{
    let mut node = Vec::with_capacity(size);
    node.push(start);
    let mut edge = Vec::with_capacity(size);
    edge.push(0);

    'outer: while node.len() > 0 {
        let i = node.len() - 1;
        let n = node[i];
        for j in edge[i]..edges[n].len() {
            let m = edges[n][j];
            if m < n {
                continue;
            }
            if node[0..node.len() - 1]
                .iter()
                .all(|prev_n| edges_set.contains(&(m, *prev_n)))
            {
                node.push(m);
                edge.push(0);
                edge[i] = j + 1;
                if node.len() == size {
                    f(node.as_slice());
                    break;
                } else {
                    continue 'outer;
                }
            }
        }
        edge.pop();
        node.pop();
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let edges_str: Vec<(&str, &str)> = input
        .trim()
        .split("\n")
        .map(|l| l.split("-").collect_tuple().unwrap())
        .collect();
    // Make sure all nodes that start with "t" have lower node numbers
    let nodes_str: Vec<&str> = edges_str
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .sorted_by(|a, b| {
            if a.starts_with("t") == b.starts_with("t") {
                a.cmp(b)
            } else if a.starts_with("t") {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .dedup()
        .collect();
    let nodes_idx: HashMap<&str, usize> =
        nodes_str.iter().enumerate().map(|(a, b)| (*b, a)).collect();

    let mut edges = Vec::new();
    let mut edges_set = HashSet::new();
    edges.resize(nodes_idx.len(), Vec::default());
    for (a, b) in edges_str.iter().map(|(a, b)| (nodes_idx[a], nodes_idx[b])) {
        edges[a].push(b);
        edges[b].push(a);
        edges_set.insert((a, b));
        edges_set.insert((b, a));
    }

    let mut res = 0;
    for (n, _name) in nodes_str
        .iter()
        .take_while(|n| n.starts_with("t"))
        .enumerate()
    {
        cliques(&edges, &edges_set, n, 3, |_| {
            res += 1;
        });
    }
    println!("{res}");
}
