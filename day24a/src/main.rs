use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::io;

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

    fn eval(&self, g1: u8, g2: u8) -> u8 {
        match self {
            Op::AND => g1 & g2,
            Op::XOR => g1 ^ g2,
            Op::OR => g1 | g2,
        }
    }
}

struct Gate<'a> {
    gate1: &'a str,
    gate2: &'a str,
    op: Op,
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let (signals_str, gates_str) = input.split("\n\n").collect_tuple().unwrap();

    let signal_re = Regex::new(r"(?<gate>[a-z0-9]+): (?<value>0|1)").unwrap();
    let gate_re = Regex::new(
        r"(?<gate1>[a-z0-9]+) (?<op>AND|OR|XOR) (?<gate2>[a-z0-9]+) -> (?<result>[a-z0-9]+)",
    )
    .unwrap();

    let mut values: HashMap<&str, u8> = signals_str
        .trim()
        .split("\n")
        .map(|s| {
            let c = signal_re.captures(s).unwrap();
            (
                c.name("gate").unwrap().as_str(),
                c["value"].parse().unwrap(),
            )
        })
        .collect();

    let mut gates: HashMap<&str, Gate> = HashMap::new();

    let connections = gates_str
        .trim()
        .split("\n")
        .flat_map(|s| {
            let c = gate_re.captures(s).unwrap();

            let gate1 = c.name("gate1").unwrap().as_str();
            let gate2 = c.name("gate2").unwrap().as_str();
            let result = c.name("result").unwrap().as_str();
            let op = Op::from_str(c.name("op").unwrap().as_str()).unwrap();

            gates.insert(result, Gate { gate1, gate2, op });

            [(gate1, result), (gate2, result)]
        })
        .into_group_map();

    // each gate has exactly 2 inputs
    let mut inputs: HashMap<&str, usize> = gates.keys().map(|k| (*k, 2)).collect();

    let mut queue: VecDeque<&str> = values.keys().map(|v| *v).collect();
    while let Some(gate) = queue.pop_front() {
        for next_gate in connections.get(gate).unwrap_or(&Vec::default()) {
            *inputs.get_mut(next_gate).unwrap() -= 1;
            if inputs[next_gate] == 0 {
                queue.push_back(next_gate);
                let g = &gates[next_gate];
                values.insert(next_gate, g.op.eval(values[g.gate1], values[g.gate2]));
            }
        }
    }

    let mut res: u64 = 0;
    for i in 0..64 {
        let rname = format!("z{:02}", i);
        if let Some(v) = values.get(rname.as_str()) {
            res |= ((*v) as u64) << i;
        } else {
            break;
        }
    }

    println!("{}", res);
}
