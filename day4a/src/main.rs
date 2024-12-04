use std::io;

fn main() {
    let mut input = Vec::new();
    for line in io::stdin().lines() {
        input.push(line.unwrap().trim().to_owned());
    }

    let w = input[0].len() as i32;
    let h = input.len() as i32;

    let pattern = "XMAS".as_bytes();
    let mut found = 0;

    for xi in -1..=1 as i32 {
        for yi in -1..=1 as i32 {
            if xi == 0 && yi == 0 {
                continue;
            }
            for y in 0..h {
                'outer: for x in 0..w {
                    for off in 0..pattern.len() as i32 {
                        let ox = x + off * xi;
                        let oy = y + off * yi;
                        if 0 > ox || ox >= w || 0 > oy || oy >= h {
                            continue 'outer;
                        }
                        if pattern[off as usize] != input[oy as usize].as_bytes()[ox as usize] {
                            continue 'outer;
                        }
                    }
                    found += 1;
                }
            }
        }
    }
    println!("{}", found);
}
