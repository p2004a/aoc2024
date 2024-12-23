use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io;

fn cliques<F>(edges: &Vec<Vec<usize>>, edges_set: &HashSet<(usize, usize)>, start: usize, mut f: F)
where
    F: FnMut(&[usize]),
{
    let mut node = vec![start];
    let mut edge = vec![0];
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
                if node.len() > 2 {
                    f(node.as_slice());
                }
                continue 'outer;
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
    let nodes_str: Vec<&str> = edges_str
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .sorted()
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

    let mut largest_clique = Vec::new();
    for n in 0..nodes_str.len() {
        cliques(&edges, &edges_set, n, |clique| {
            if clique.len() > largest_clique.len() {
                largest_clique = Vec::from(clique);
            }
        });
    }
    println!(
        "{}",
        largest_clique
            .iter()
            .map(|n| nodes_str[*n])
            .sorted()
            .join(",")
    );
}
