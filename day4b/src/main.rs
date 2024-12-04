use std::io;

fn main() {
    let mut input = Vec::new();
    for line in io::stdin().lines() {
        input.push(line.unwrap().trim().to_owned());
    }

    let w = input[0].len();
    let h = input.len();

    let mut found = 0;
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if input[y].as_bytes()[x] != b'A' {
                continue;
            }
            let d1 = (
                input[y - 1].as_bytes()[x - 1],
                input[y + 1].as_bytes()[x + 1],
            );
            let d2 = (
                input[y - 1].as_bytes()[x + 1],
                input[y + 1].as_bytes()[x - 1],
            );
            let var1 = (b'M', b'S');
            let var2 = (b'S', b'M');
            if (d1 == var1 || d1 == var2) && (d2 == var1 || d2 == var2) {
                found += 1;
            }
        }
    }
    println!("{}", found);
}
