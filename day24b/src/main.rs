use itertools::Itertools;
use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::{io, usize};

// Ids are created sequentially
struct IdToStr {
    id_to_name: Vec<String>,
    name_to_id: HashMap<String, usize>,
}

impl IdToStr {
    fn new() -> IdToStr {
        IdToStr {
            id_to_name: Vec::new(),
            name_to_id: HashMap::new(),
        }
    }

    fn get_or_create_id(&mut self, s: &str) -> usize {
        if let Some(id) = self.name_to_id.get(s) {
            *id
        } else {
            let id = self.name_to_id.len();
            self.name_to_id.insert(s.to_string(), id);
            self.id_to_name.push(s.to_string());
            id
        }
    }

    fn get_id(&self, s: &str) -> Option<usize> {
        self.name_to_id.get(s).copied()
    }

    fn get_str(&self, id: usize) -> Option<&str> {
        self.id_to_name.get(id).map(|x| x.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    AND,
    XOR,
    OR,
}

impl Op {
    fn from_str(s: &str) -> Option<Op> {
        match s {
            "AND" => Some(Op::AND),
            "XOR" => Some(Op::XOR),
            "OR" => Some(Op::OR),
            _ => None,
        }
    }

    #[inline(always)]
    fn eval(&self, g1: u8, g2: u8) -> u8 {
        match self {
            Op::AND => g1 & g2,
            Op::XOR => g1 ^ g2,
            Op::OR => g1 | g2,
        }
    }
}

#[derive(Clone, Debug)]
struct Gate {
    id: usize,
    ins: [usize; 2],
    op: Op,
}

struct Device<'a> {
    ops: Vec<&'a Gate>,
    bitwidth: usize,
}

impl<'a> Device<'a> {
    fn try_new(graph: &Graph) -> Option<Device> {
        let mut ops = Vec::new();
        // Topological sort of gates so evaluation later is only simple loop
        let mut queue: VecDeque<usize> = (0..graph.bitwidth * 2).collect();
        let mut incomming = Vec::new();
        incomming.resize(graph.gates.len(), 2);
        let mut visited = 0;
        while let Some(gate) = queue.pop_front() {
            visited += 1;
            for next_gate in graph.outs[gate].iter() {
                incomming[*next_gate] -= 1;
                if incomming[*next_gate] == 0 {
                    queue.push_back(*next_gate);
                    ops.push(&graph.gates[*next_gate]);
                }
            }
        }
        (visited == graph.gates.len()).then_some(Device {
            ops,
            bitwidth: graph.bitwidth,
        })
    }

    fn eval(&self, x: u64, y: u64) -> u64 {
        let mut val = Vec::new();
        val.resize(self.bitwidth * 2 + 1 + self.ops.len(), 0 as u8);
        for i in 0..self.bitwidth {
            val[i] = ((x >> i) & 1) as u8;
            val[i + self.bitwidth] = ((y >> i) & 1) as u8;
        }
        for g in self.ops.iter() {
            val[g.id] = g.op.eval(val[g.ins[0]], val[g.ins[1]]);
        }
        let mut res = 0;
        for i in 0..self.bitwidth + 1 {
            res |= (val[i + self.bitwidth * 2] as u64) << i;
        }
        res
    }
}

struct Graph {
    bitwidth: usize,
    nameid: IdToStr,
    gates: Vec<Gate>,
    outs: Vec<Vec<usize>>,
}

impl Graph {
    fn from_str(bitwidth: usize, gates_str: &str) -> Graph {
        let mut nameid = IdToStr::new();

        for (n, p) in [(bitwidth, "x"), (bitwidth, "y"), (bitwidth + 1, "out")] {
            for i in 0..n {
                nameid.get_or_create_id(format!("{}{:02}", p, i).as_str());
            }
        }

        let gate_re = Regex::new(
            r"(?<in1>[a-z0-9]+) (?<op>AND|OR|XOR) (?<in2>[a-z0-9]+) -> (?<out>[a-z0-9]+)",
        )
        .unwrap();

        let mut tmp_gates: Vec<(usize, Gate)> = gates_str
            .trim()
            .split("\n")
            .map(|gs| {
                let c = gate_re.captures(gs).unwrap();
                let in1 = nameid.get_or_create_id(c.name("in1").unwrap().as_str());
                let in2 = nameid.get_or_create_id(c.name("in2").unwrap().as_str());
                let out = nameid.get_or_create_id(c.name("out").unwrap().as_str());
                let op = Op::from_str(c.name("op").unwrap().as_str()).unwrap();
                (
                    out,
                    Gate {
                        id: out,
                        ins: [in1, in2],
                        op,
                    },
                )
            })
            .collect();

        for i in 0..bitwidth * 2 {
            tmp_gates.push((
                i,
                Gate {
                    id: i,
                    ins: [i, i],
                    op: Op::AND,
                },
            ));
        }
        // virtual "exit" gates to allow for output gate swapping.
        for i in 0..bitwidth + 1 {
            let s = format!("z{:02}", i);
            let in_id = nameid.get_id(s.as_str()).unwrap();
            let id = bitwidth * 2 + i;
            tmp_gates.push((
                id,
                Gate {
                    id,
                    ins: [in_id, in_id],
                    op: Op::AND,
                },
            ));
        }
        tmp_gates.sort_unstable_by_key(|(out, _g)| *out);
        let gates: Vec<Gate> = tmp_gates
            .into_iter()
            .enumerate()
            .map(|(i, (out, g))| {
                assert!(i == out);
                g
            })
            .collect();

        let mut outs = Vec::new();
        outs.resize_with(gates.len(), || Vec::new());
        for i in bitwidth * 2..gates.len() {
            for j in gates[i].ins.iter() {
                outs[*j].push(i);
            }
        }

        Graph {
            bitwidth,
            nameid,
            gates,
            outs,
        }
    }

    fn swap_gates(&mut self, a: usize, b: usize) {
        assert!(a != b && a >= self.bitwidth * 3 + 1 && b >= self.bitwidth * 3 + 1);
        let offset = 1000000000;
        for n in [a, b] {
            for gn in self.outs[n].iter() {
                for input in self.gates[*gn].ins.iter_mut() {
                    if *input == n {
                        *input = n + offset;
                    }
                }
            }
        }
        self.outs.swap(a, b);
        for (ap, bp) in [(a, b), (b, a)] {
            for gn in self.outs[ap].iter() {
                for input in self.gates[*gn].ins.iter_mut() {
                    if *input == bp + offset {
                        *input = ap;
                    }
                }
            }
        }
    }
}

fn bfs<P>(graph: &Graph, from: usize, mut p: P) -> (Vec<Vec<usize>>, Vec<usize>)
where
    P: FnMut(&Vec<usize>, usize) -> bool,
{
    let mut prev = Vec::new();
    prev.resize_with(graph.gates.len(), || Vec::new());
    let mut dist = Vec::new();
    dist.resize(graph.gates.len(), usize::MAX);
    dist[from] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(from);
    while let Some(n) = queue.pop_front() {
        if !p(&dist, n) {
            continue;
        }
        for m in graph.outs[n].iter() {
            if dist[n] + 1 < dist[*m] {
                queue.push_back(*m);
                dist[*m] = dist[n] + 1;
                prev[*m].clear();
            }
            if dist[n] + 1 <= dist[*m] {
                prev[*m].push(n);
            }
        }
    }
    (prev, dist)
}

fn get_shortest_paths(graph: &Graph, from: usize, to: usize) -> Vec<BTreeMap<Op, u8>> {
    let (prev, dist) = bfs(graph, from, |d, n| d[n] + 1 <= d[to]);

    if dist[to] == usize::MAX {
        return vec![];
    }

    let mut res = Vec::new();
    res.resize_with(dist[to] + 1, || BTreeMap::new());

    let mut visited = Vec::new();
    visited.resize(graph.gates.len(), false);
    let mut queue = VecDeque::new();
    queue.push_back(to);
    while let Some(n) = queue.pop_front() {
        *res[dist[n]].entry(graph.gates[n].op).or_insert(0) += 1;
        for m in prev[n].iter() {
            if !visited[*m] {
                visited[*m] = true;
                queue.push_back(*m);
            }
        }
    }

    // frirst and last gates not needed as they are artificial ANDs
    res.remove(0);
    res.pop();
    res
}

fn reachable_gates(graph: &Graph, from: usize, max_dist: usize) -> HashSet<usize> {
    let mut visited = HashSet::new();
    bfs(graph, from, |d, n| {
        visited.insert(n);
        d[n] < max_dist
    });
    visited
}

fn reachable_gates_back(graph: &Graph, from: usize, max_dist: usize) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut dist = Vec::new();
    dist.resize(graph.gates.len(), usize::MAX);
    dist[from] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(from);
    while let Some(n) = queue.pop_front() {
        visited.insert(n);
        if dist[n] >= max_dist {
            continue;
        }
        for m in graph.gates[n].ins {
            if dist[n] + 1 < dist[m] {
                queue.push_back(m);
                dist[m] = dist[n] + 1;
            }
        }
    }
    visited
}

fn find_good_bits(dev: &Device) -> Vec<usize> {
    let mut bits = Vec::new();
    'outer: for i in 0..dev.bitwidth {
        for c in 0..=1 {
            let carry = if i == 0 { 0 } else { c << (i - 1) };
            for a in 0..=1 {
                let x = (a << i) | carry;
                for b in 0..=1 {
                    let y = (b << i) | carry;
                    if dev.eval(x, y) != x + y {
                        continue 'outer;
                    }
                }
            }
        }
        bits.push(i);
    }
    bits
}

fn try_find_swaps(
    graph: &mut Graph,
    bad_gate_candidates: &Vec<usize>,
    used_bad_gate: &mut Vec<bool>,
    good_bits: usize,
    max_swaps: usize,
    swaps: &mut Vec<usize>,
    min_i: usize,
) -> bool {
    for i in min_i..bad_gate_candidates.len() - 1 {
        if used_bad_gate[i] {
            continue;
        }
        used_bad_gate[i] = true;
        swaps.push(bad_gate_candidates[i]);
        for j in i + 1..bad_gate_candidates.len() {
            if used_bad_gate[j] {
                continue;
            }
            used_bad_gate[j] = true;
            swaps.push(bad_gate_candidates[j]);
            graph.swap_gates(bad_gate_candidates[i], bad_gate_candidates[j]);

            if let Some(dev) = Device::try_new(graph) {
                let new_good_bits = find_good_bits(&dev).len();
                if new_good_bits == graph.bitwidth
                    || (new_good_bits > good_bits
                        && max_swaps > 0
                        && try_find_swaps(
                            graph,
                            bad_gate_candidates,
                            used_bad_gate,
                            new_good_bits,
                            max_swaps - 1,
                            swaps,
                            i + 1,
                        ))
                {
                    return true;
                }
            }

            graph.swap_gates(bad_gate_candidates[i], bad_gate_candidates[j]);
            swaps.pop();
            used_bad_gate[j] = false;
        }
        swaps.pop();
        used_bad_gate[i] = false;
    }
    false
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let (xy_str, gates_str) = input.split("\n\n").collect_tuple().unwrap();
    let bitwidth = xy_str.trim().split("\n").count() / 2;
    let mut graph = Graph::from_str(bitwidth, gates_str);
    println!("bitwidth: {}", bitwidth);
    println!("total_gates: {}", graph.gates.len());

    let good_bits = find_good_bits(&Device::try_new(&graph).unwrap());
    let num_good_bits = good_bits.len();
    println!("good_bits: {}", num_good_bits);

    let max_gate_depth = 10; // just assumption :shrug:
    let mut good_gates: HashSet<usize> = HashSet::new();
    for bit in good_bits {
        let out = bit + bitwidth * 2;
        let reach_back = reachable_gates_back(&graph, out, max_gate_depth);
        for input in [bit, bit + bitwidth] {
            let reach_front = reachable_gates(&graph, input, max_gate_depth);
            good_gates.extend(reach_front.intersection(&reach_back));
        }
    }

    println!("good_gates: {}", good_gates.len());

    let mut bad_gate_candidates: HashSet<usize> = HashSet::new();

    for (s_in, s_out) in [
        (0, bitwidth * 2),            // x_n -> res_n
        (bitwidth, bitwidth * 2),     // y_n -> res_n
        (0, bitwidth * 2 + 1),        // x_n -> res_{n+1}
        (bitwidth, bitwidth * 2 + 1), // y_n -> res_{n+1}
    ] {
        let mut paths = HashMap::new();
        for i in 0..bitwidth {
            let from = i + s_in;
            let to = i + s_out;
            let p = get_shortest_paths(&graph, from, to);
            paths.entry(p).or_insert(Vec::new()).push(i);
        }

        let mut with_wrong_paths: Vec<usize> = Vec::new();
        let mut prev_max = 0;
        let mut path_len = 0;
        let mut prev_set: &Vec<usize> = &Vec::new();
        for (paths, g) in paths.iter() {
            if g.len() >= prev_max {
                with_wrong_paths.extend_from_slice(&prev_set.as_slice());
                prev_max = g.len();
                path_len = paths.len();
                prev_set = &g;
            } else {
                with_wrong_paths.extend_from_slice(g.as_slice());
            }
        }

        for g in with_wrong_paths {
            let visited = reachable_gates(&graph, g, path_len);
            bad_gate_candidates.extend(
                visited
                    .into_iter()
                    .filter(|g| *g >= bitwidth * 3 + 1)
                    .filter(|g| !good_gates.contains(g)),
            );
        }
    }

    println!("bad_gate_candidates: {}", bad_gate_candidates.len());
    let bad_gate_candidates = bad_gate_candidates.into_iter().collect_vec();

    let mut swaps = Vec::new();
    let mut used_bad_gate = Vec::new();
    used_bad_gate.resize(bad_gate_candidates.len(), false);
    if try_find_swaps(
        &mut graph,
        &bad_gate_candidates,
        &mut used_bad_gate,
        num_good_bits,
        4,
        &mut swaps,
        0,
    ) {
        println!(
            "result: {}",
            swaps
                .iter()
                .map(|n| graph.nameid.get_str(*n).unwrap())
                .sorted()
                .join(",")
        );
    } else {
        println!("Failure :(");
    }
}
