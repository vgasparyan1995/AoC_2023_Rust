use std::{
    collections::HashSet,
    io::{Lines, StdinLock},
    print, println,
};

use itertools::Itertools;

type Row = Vec<char>;
type Mtx = Vec<Row>;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos {
    r: usize,
    c: usize,
}
struct Input {
    mtx: Mtx,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let mtx = lines
            .map(|line| line.unwrap().chars().collect::<Row>())
            .collect::<Mtx>();
        Self { mtx }
    }
}

fn find(mtx: &Mtx, ch: char) -> Option<Pos> {
    let rows = mtx.len();
    let cols = mtx[0].len();
    (0..rows).cartesian_product(0..cols).find_map(|(r, c)| {
        if mtx[r][c] == ch {
            Some(Pos { r, c })
        } else {
            None
        }
    })
}

fn neighbors(mtx: &Mtx, pos: Pos) -> Vec<Pos> {
    let rows = mtx.len();
    let cols = mtx[0].len();
    let (r, c) = (pos.r as i32, pos.c as i32);
    match mtx[pos.r][pos.c] {
        'S' => vec![(r, c + 1), (r, c - 1), (r + 1, c), (r - 1, c)],
        '-' => vec![(r, c + 1), (r, c - 1)],
        '|' => vec![(r + 1, c), (r - 1, c)],
        'L' => vec![(r, c + 1), (r - 1, c)],
        'J' => vec![(r, c - 1), (r - 1, c)],
        '7' => vec![(r, c - 1), (r + 1, c)],
        'F' => vec![(r, c + 1), (r + 1, c)],
        _ => vec![],
    }
    .into_iter()
    .filter_map(|(r, c)| {
        if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
            Some(Pos {
                r: r as usize,
                c: c as usize,
            })
        } else {
            None
        }
    })
    .collect()
}

fn advance(mtx: &Mtx, curr: Pos, next: Pos) -> Option<Pos> {
    if !neighbors(&mtx, curr).contains(&next) {
        return None;
    }
    let next_neighbors = neighbors(&mtx, next);
    if !next_neighbors.contains(&curr) || next_neighbors.len() != 2 {
        return None;
    }
    next_neighbors.into_iter().find(|&pos| pos != curr)
}

fn traverse(mtx: &Mtx, mut curr: Pos, mut next: Pos, pred: fn(char) -> bool) -> Option<usize> {
    let mut length = 0;
    loop {
        if let Some(next_next) = advance(&mtx, curr, next) {
            length += 1;
            curr = next;
            next = next_next;
            if pred(mtx[next.r][next.c]) {
                return Some(length);
            }
        } else {
            return None;
        }
    }
}

fn traverse_visit(
    mtx: &mut Mtx,
    mut curr: Pos,
    mut next: Pos,
    pred: fn(char) -> bool,
    mut visit: impl FnMut(char, Pos),
) -> bool {
    loop {
        if let Some(next_next) = advance(&mtx, curr, next) {
            curr = next;
            next = next_next;
            visit(mtx[curr.r][curr.c], curr);
            if pred(mtx[next.r][next.c]) {
                return true;
            }
        } else {
            return false;
        }
    }
}

fn print(mtx: &Mtx, boundary: &HashSet<Pos>, inside: &HashSet<Pos>) {
    let rows = mtx.len();
    let cols = mtx[0].len();
    for r in 0..rows {
        for c in 0..cols {
            let pos = Pos { r, c };
            if boundary.contains(&pos) {
                print!("{}", mtx[r][c])
            } else if inside.contains(&pos) {
                print!(".");
            } else {
                print!(":");
            }
        }
        println!("");
    }
}

fn count_inside(mtx: &Mtx, boundary: HashSet<Pos>) -> usize {
    let rows = mtx.len();
    let cols = mtx[0].len();
    let mut horizontal = HashSet::new();
    for r in 0..rows {
        let mut inside = false;
        for c in 0..cols {
            let pos = Pos { r, c };
            if boundary.contains(&pos) {
                inside = !inside;
            } else if inside {
                horizontal.insert(pos);
            }
        }
    }
    let mut vertical = HashSet::new();
    for c in 0..cols {
        let mut inside = false;
        for r in 0..rows {
            let pos = Pos { r, c };
            if boundary.contains(&pos) {
                inside = !inside;
            } else if inside {
                vertical.insert(pos);
            }
        }
    }
    let inside_cells = horizontal
        .intersection(&vertical)
        .map(|&pos| pos)
        .collect::<HashSet<_>>();
    print(&mtx, &boundary, &inside_cells);
    inside_cells.len()
}

fn part1(input: Input) -> usize {
    let start = find(&input.mtx, 'S').unwrap();
    let neighbors = neighbors(&input.mtx, start);
    for next in neighbors {
        if let Some(length) = traverse(&input.mtx, start, next, |ch| ch == 'S') {
            return (length + 1) / 2;
        }
    }
    0
}

fn part2(mut input: Input) -> usize {
    let start = find(&input.mtx, 'S').unwrap();
    let neighbors = neighbors(&input.mtx, start);
    for next in neighbors {
        let mut boundary: HashSet<Pos> = HashSet::new();
        if traverse_visit(
            &mut input.mtx,
            start,
            next,
            |ch| ch == 'S',
            |_, pos| {
                boundary.insert(pos);
            },
        ) {
            boundary.insert(start);
            return count_inside(&input.mtx, boundary);
        }
    }
    0
}

fn main() {
    let input = Input::from(std::io::stdin().lines());
    println!("{}", part2(input));
}
