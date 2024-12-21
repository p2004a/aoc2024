use std::fmt::Display;
use std::io;

trait Key {
    fn key_pos(&self) -> (i32, i32);
    fn block() -> (i32, i32);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DirKey {
    Left,
    Right,
    Up,
    Down,
    A,
}

impl DirKey {
    fn from_u8(c: u8) -> Option<DirKey> {
        match c {
            b'<' => Some(DirKey::Left),
            b'>' => Some(DirKey::Right),
            b'^' => Some(DirKey::Up),
            b'v' => Some(DirKey::Down),
            b'A' => Some(DirKey::A),
            _ => None,
        }
    }
}

impl Key for DirKey {
    fn key_pos(&self) -> (i32, i32) {
        match &self {
            DirKey::Up => (1, 0),
            DirKey::A => (2, 0),
            DirKey::Left => (0, 1),
            DirKey::Down => (1, 1),
            DirKey::Right => (2, 1),
        }
    }

    fn block() -> (i32, i32) {
        (0, 0)
    }
}

impl Display for DirKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            DirKey::Up => "^",
            DirKey::A => "A",
            DirKey::Left => "<",
            DirKey::Down => "v",
            DirKey::Right => ">",
        };
        write!(f, "{s}")
    }
}

// fn dirkey_steps(a: DirKey, b: DirKey) -> ((u64, DirKey), (u64, DirKey)) {
//     let pa = a.key_pos();
//     let pb = b.key_pos();
//     let d = (pb.0 - pa.0, pb.1 - pa.1);
//     let dx = if d.0 >= 0 {
//         DirKey::Right
//     } else {
//         DirKey::Left
//     };
//     let dy = if d.1 >= 0 { DirKey::Down } else { DirKey::Up };
//     let mut res = ((d.0.abs() as u64, dx), (d.1.abs() as u64, dy));
//     // if we are on top row, go down first
//     if pa.1 == 0 {
//         res = (res.1, res.0);
//     }
//     res
// }

fn append_seq(seq: &mut Vec<DirKey>, s: (u64, DirKey)) {
    for _ in 0..s.0 {
        seq.push(s.1);
    }
}

fn steps(
    level: u32,
    s1: (u64, DirKey),
    s2: (u64, DirKey),
    an: u64,
    build_seq: bool,
) -> (u64, Vec<DirKey>) {
    if level == 0 {
        let res = s1.0 + s2.0 + an;
        let seq = if build_seq {
            let mut seq = Vec::with_capacity(res as usize);
            append_seq(&mut seq, s1);
            append_seq(&mut seq, s2);
            append_seq(&mut seq, (an, DirKey::A));
            seq
        } else {
            Vec::default()
        };
        return (res, seq);
    }

    let mut res = 0;
    let mut res_seq = Vec::new();
    let mut prev_dir = DirKey::A;
    for (num, dir) in [s1, s2, (an, DirKey::A)] {
        if num == 0 {
            continue;
        }

        let (cost, seq) = key_steps(prev_dir, dir)
            .iter()
            .map(|(ss1, ss2)| steps(level - 1, *ss1, *ss2, num, build_seq))
            .reduce(|a, b| if a.0 < b.0 { a } else { b })
            .unwrap();
        res += cost;
        res_seq.extend(seq);

        prev_dir = dir;
    }

    (res, res_seq)
}

#[derive(Clone, Copy, Debug)]
enum LockKey {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    Activate,
}

impl LockKey {
    fn from_u8(c: u8) -> Option<LockKey> {
        match c {
            b'0' => Some(LockKey::K0),
            b'1' => Some(LockKey::K1),
            b'2' => Some(LockKey::K2),
            b'3' => Some(LockKey::K3),
            b'4' => Some(LockKey::K4),
            b'5' => Some(LockKey::K5),
            b'6' => Some(LockKey::K6),
            b'7' => Some(LockKey::K7),
            b'8' => Some(LockKey::K8),
            b'9' => Some(LockKey::K9),
            b'A' => Some(LockKey::Activate),
            _ => None,
        }
    }
}

impl Key for LockKey {
    fn key_pos(&self) -> (i32, i32) {
        match &self {
            LockKey::K7 => (0, 0),
            LockKey::K8 => (1, 0),
            LockKey::K9 => (2, 0),
            LockKey::K4 => (0, 1),
            LockKey::K5 => (1, 1),
            LockKey::K6 => (2, 1),
            LockKey::K1 => (0, 2),
            LockKey::K2 => (1, 2),
            LockKey::K3 => (2, 2),
            LockKey::K0 => (1, 3),
            LockKey::Activate => (2, 3),
        }
    }

    fn block() -> (i32, i32) {
        (0, 3)
    }
}

fn key_steps<K: Key>(a: K, b: K) -> Vec<((u64, DirKey), (u64, DirKey))> {
    let pa = a.key_pos();
    let pb = b.key_pos();
    let d = (pb.0 - pa.0, pb.1 - pa.1);
    let dx = if d.0 >= 0 {
        DirKey::Right
    } else {
        DirKey::Left
    };
    let dy = if d.1 >= 0 { DirKey::Down } else { DirKey::Up };

    let res1 = ((d.0.abs() as u64, dx), (d.1.abs() as u64, dy));
    let res2 = (res1.1, res1.0);

    let mut res = Vec::with_capacity(2);
    if pa.1 != K::block().1 || pa.0 + d.0 != K::block().0 {
        res.push(res1);
    }
    if pa.0 != K::block().0 || pa.1 + d.1 != K::block().1 {
        res.push(res2);
    }

    res
}

fn sequence(code: &[LockKey], robots: u32, build_seq: bool) -> (u64, Vec<DirKey>) {
    let mut res = 0;
    let mut res_seq = Vec::new();
    let mut prev = LockKey::Activate;
    for c in code {
        let (cost, seq) = key_steps(prev, *c)
            .iter()
            .map(|(s1, s2)| steps(robots, *s1, *s2, 1, build_seq))
            .reduce(|a, b| if a.0 < b.0 { a } else { b })
            .unwrap();
        res += cost;
        res_seq.extend(seq);
        prev = *c;
    }

    (res, res_seq)
}

fn print_sequence(sequence: &Vec<DirKey>) {
    for c in sequence {
        print!("{c}");
    }
    println!("");
}

fn main() {
    let mut res = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let code: Vec<LockKey> = line.bytes().map(|c| LockKey::from_u8(c).unwrap()).collect();
        let code_val: u64 = line.strip_suffix("A").unwrap().parse().unwrap();
        let (len, seq) = sequence(code.as_slice(), 2, true);
        res += code_val * len;
        print!("{}: {} ", line, len);
        print_sequence(&seq);
    }
    println!("{}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_steps() {
        // The output doesn't have to be deterministic, but that's the
        // stable correct output at the moment.
        let expected: Vec<DirKey> = "v<A<AA>>^AvA^<A>AvA^A"
            .bytes()
            .map(|c| DirKey::from_u8(c).unwrap())
            .collect();

        let (_, seq) = steps(2, (1, DirKey::Left), (1, DirKey::Up), 1, true);
        assert_eq!(expected, seq);
    }
}
