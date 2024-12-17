use enum_map::{enum_map, Enum, EnumMap};
use itertools::{self, Itertools};
use regex::Regex;
use std::io;

#[derive(Enum, Clone, Copy)]
enum Reg {
    A,
    B,
    C,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

fn main() {
    let re = Regex::new(r"R.* A: (\d+)\nR.* B: (\d+)\nR.* C: (\d+)\n\nP.*: ([0-9,]+)").unwrap();
    let input = io::read_to_string(io::stdin()).unwrap();
    let c = re.captures(input.as_str()).unwrap();
    let regs = enum_map! {
        Reg::A => c[1].parse().unwrap(),
        Reg::B => c[2].parse().unwrap(),
        Reg::C => c[3].parse().unwrap(),
    };
    let prog: Vec<Op> = c[4]
        .split(",")
        .map(|v| v.parse().unwrap())
        .tuples()
        .map(|(opcode, operand)| Op::decode(opcode, operand))
        .collect();
    let out = simulate(regs, prog.as_slice());
    println!("{}", out.iter().map(|v| v.to_string()).join(","));
}
