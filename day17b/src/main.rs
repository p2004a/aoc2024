use enum_map::{enum_map, Enum, EnumMap};
use itertools::{self, Itertools};
use regex::Regex;
use std::io;

#[derive(Enum, Clone, Copy, Debug)]
enum Reg {
    A,
    B,
    C,
}

#[derive(Clone, Copy, Debug)]
enum Combo {
    Val(u64),
    Reg(Reg),
}

impl Combo {
    fn decode(operand: u32) -> Self {
        match operand {
            0..=3 => Self::Val(operand as u64),
            4 => Self::Reg(Reg::A),
            5 => Self::Reg(Reg::B),
            6 => Self::Reg(Reg::C),
            _ => panic!("unexpected combo value"),
        }
    }

    fn val(&self, reg: &EnumMap<Reg, u64>) -> u64 {
        match self {
            Self::Val(v) => *v,
            Self::Reg(r) => reg[*r],
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Div(Combo, Reg),
    Bxl(u64),
    Bst(Combo),
    Jnz(usize),
    Bxc,
    Out(Combo),
}

impl Op {
    fn decode(opcode: u32, operand: u32) -> Self {
        match opcode {
            0 => Self::Div(Combo::decode(operand), Reg::A),
            1 => Self::Bxl(operand as u64),
            2 => Self::Bst(Combo::decode(operand)),
            3 => Self::Jnz(operand as usize),
            4 => Self::Bxc,
            5 => Self::Out(Combo::decode(operand)),
            6 => Self::Div(Combo::decode(operand), Reg::B),
            7 => Self::Div(Combo::decode(operand), Reg::C),
            _ => panic!("Unexpected opcode"),
        }
    }
}

fn simulate(mut reg: EnumMap<Reg, u64>, prog: &[Op]) -> Vec<u64> {
    let mut out = Vec::new();

    let mut ip = 0;
    while let Some(op) = prog.get(ip) {
        match op {
            Op::Div(c, r) => {
                reg[*r] = reg[Reg::A] >> c.val(&reg);
            }
            Op::Bxl(l) => {
                reg[Reg::B] ^= l;
            }
            Op::Bst(c) => {
                reg[Reg::B] = c.val(&reg) % 8;
            }
            Op::Jnz(p) => {
                if reg[Reg::A] != 0 {
                    ip = *p;
                    continue;
                }
            }
            Op::Bxc => {
                reg[Reg::B] ^= reg[Reg::C];
            }
            Op::Out(c) => {
                out.push(c.val(&reg) % 8);
            }
        };
        ip += 1;
    }
    out
}

fn find_a(prog: &[Op], expected: &[u64], initial_a: u64) -> Option<u64> {
    for a_suff in 0..8 {
        let a = initial_a << 3 | a_suff;
        if a == 0 {
            // Just stupid edge case.
            continue;
        }
        let regs = enum_map! {
            Reg::A => a,
            Reg::B => 0,
            Reg::C => 0,
        };
        let out = simulate(regs, prog);
        if expected[expected.len() - out.len()..] == out {
            if expected.len() == out.len() {
                return Some(a);
            }
            if let Some(final_a) = find_a(prog, expected, a) {
                return Some(final_a);
            }
        }
    }
    None
}

fn main() {
    let re = Regex::new(r"R.* A: (\d+)\nR.* B: (\d+)\nR.* C: (\d+)\n\nP.*: ([0-9,]+)").unwrap();
    let input = io::read_to_string(io::stdin()).unwrap();
    let c = re.captures(input.as_str()).unwrap();
    let input: Vec<u64> = c[4].split(",").map(|v| v.parse().unwrap()).collect();

    let prog: Vec<Op> = input
        .iter()
        .map(|v| *v as u32)
        .tuples()
        .map(|(opcode, operand)| Op::decode(opcode, operand))
        .collect();
    let a = find_a(prog.as_slice(), input.as_slice(), 0).unwrap();
    println!("{}", a);
}
