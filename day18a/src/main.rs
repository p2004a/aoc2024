use itertools::Itertools;
use std::collections::VecDeque;
use std::env;
use std::io;
use std::process::ExitCode;

fn create_grid<T>(size: i32, initial: T) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    let mut grid: Vec<Vec<T>> = Vec::new();
    grid.resize_with(size as usize, || {
        let mut v = Vec::new();
        v.resize(size as usize, initial);
        v
    });
    grid
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("{} size steps", args[0]);
        return ExitCode::FAILURE;
    }
    let size = args[1].parse::<i32>().unwrap() + 1;
    let steps: usize = args[2].parse().unwrap();

    let input: Vec<(usize, usize)> = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut grid = create_grid(size, false);
    for s in 0..steps {
        grid[input[s].1][input[s].0] = true;
    }

    let mut queue = VecDeque::new();
    let mut dist = create_grid(size, std::u32::MAX);
    dist[0][0] = 0;
    queue.push_back((0, 0));
    while let Some(pos) = queue.pop_front() {
        let idx = (pos.0 as usize, pos.1 as usize);
        for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let npos = (pos.0 + dir.0, pos.1 + dir.1);
            if npos.0 < 0 || size <= npos.0 || npos.1 < 0 || size <= npos.1 {
                continue;
            }
            let nidx = (npos.0 as usize, npos.1 as usize);
            if !grid[nidx.1][nidx.0] && dist[idx.1][idx.0] + 1 < dist[nidx.1][nidx.0] {
                dist[nidx.1][nidx.0] = dist[idx.1][idx.0] + 1;
                queue.push_back(npos);
            }
        }
    }

    println!("{}", dist[size as usize - 1][size as usize - 1]);

    ExitCode::SUCCESS
}
