use itertools::Itertools;
use std::io;

type Pos = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Dir {
    East,
    West,
    North,
    South,
}

impl Dir {
    fn from_char(c: char) -> Option<Dir> {
        match c {
            '<' => Some(Dir::West),
            '>' => Some(Dir::East),
            '^' => Some(Dir::North),
            'v' => Some(Dir::South),
            _ => None,
        }
    }

    // We take adventage of the fact that grid in this task is very nice
    // and has border so we can use simple function like this 🎉.
    fn adv(&self, pos: Pos) -> Pos {
        let d = match self {
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
            Dir::North => (0, -1),
            Dir::South => (0, 1),
        };
        (
            (pos.0 as isize + d.0) as usize,
            (pos.1 as isize + d.1) as usize,
        )
    }

    fn horizontal(&self) -> bool {
        match self {
            Dir::East | Dir::West => true,
            Dir::North | Dir::South => false,
        }
    }
}

#[derive(Clone, Copy)]
enum GridBox {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum GridElem {
    Empty,
    Wall,
    Robot,
    Box(GridBox),
}

fn find_robot(grid: &Vec<Vec<GridElem>>) -> Option<Pos> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let GridElem::Robot = grid[y][x] {
                return Some((x, y));
            }
        }
    }
    None
}

fn other_box_part(box_part: GridBox, pos: Pos) -> Pos {
    match box_part {
        GridBox::Left => (pos.0 + 1, pos.1),
        GridBox::Right => (pos.0 - 1, pos.1),
    }
}

fn can_move(grid: &Vec<Vec<GridElem>>, pos: Pos, dir: Dir) -> bool {
    match grid[pos.1][pos.0] {
        GridElem::Empty => true,
        GridElem::Wall => false,
        GridElem::Box(b) => {
            if dir.horizontal() {
                can_move(grid, dir.adv(dir.adv(pos)), dir)
            } else {
                let other = other_box_part(b, pos);
                can_move(grid, dir.adv(pos), dir) && can_move(grid, dir.adv(other), dir)
            }
        }
        GridElem::Robot => can_move(grid, dir.adv(pos), dir),
    }
}

fn do_move_single_piece(grid: &mut Vec<Vec<GridElem>>, pos: Pos, dir: Dir) {
    let next_pos = dir.adv(pos);
    do_move(grid, next_pos, dir);
    grid[next_pos.1][next_pos.0] = grid[pos.1][pos.0];
    grid[pos.1][pos.0] = GridElem::Empty;
}

fn do_move(grid: &mut Vec<Vec<GridElem>>, pos: Pos, dir: Dir) {
    match grid[pos.1][pos.0] {
        GridElem::Empty => (),
        GridElem::Robot => do_move_single_piece(grid, pos, dir),
        GridElem::Box(b) => {
            if dir.horizontal() {
                do_move_single_piece(grid, pos, dir);
            } else {
                let other = other_box_part(b, pos);
                do_move_single_piece(grid, pos, dir);
                do_move_single_piece(grid, other, dir);
            }
        }
        GridElem::Wall => panic!("impossible to move wall"),
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<GridElem>>) {
    let out = grid
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| match c {
                    GridElem::Empty => '.',
                    GridElem::Wall => '#',
                    GridElem::Robot => '@',
                    GridElem::Box(GridBox::Left) => '[',
                    GridElem::Box(GridBox::Right) => ']',
                })
                .join("")
        })
        .join("\n");
    println!("{}", out);
}

fn compute_result(grid: &Vec<Vec<GridElem>>) -> usize {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let GridElem::Box(GridBox::Left) = grid[y][x] {
                result += y * 100 + x;
            }
        }
    }
    result
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let (grid_str, steps_str) = input.split("\n\n").collect_tuple().unwrap();

    let mut grid: Vec<Vec<GridElem>> = grid_str
        .trim()
        .split_whitespace()
        .map(|l| {
            l.as_bytes()
                .iter()
                .flat_map(|c| match c {
                    b'#' => [GridElem::Wall, GridElem::Wall],
                    b'.' => [GridElem::Empty, GridElem::Empty],
                    b'O' => [GridElem::Box(GridBox::Left), GridElem::Box(GridBox::Right)],
                    b'@' => [GridElem::Robot, GridElem::Empty],
                    _ => panic!("unexpected input char in grid"),
                })
                .collect()
        })
        .collect();

    let steps: Vec<Dir> = steps_str.chars().filter_map(Dir::from_char).collect();

    let mut pos = find_robot(&grid).unwrap();
    //print_grid(&grid);
    for s in steps {
        //dbg!(s);
        if can_move(&grid, pos, s) {
            do_move(&mut grid, pos, s);
            pos = s.adv(pos);
            //print_grid(&grid);
        } else {
            //println!("can't move");
        }
    }

    println!("{}", compute_result(&grid));
}
