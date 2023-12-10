use std::{
    io::{Lines, StdinLock},
    println,
};

use itertools::Itertools;

type Row = Vec<char>;
type Mtx = Vec<Row>;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    match mtx[pos.r][pos.c] {
        'S' => vec![
            Pos {
                r: pos.r,
                c: pos.c + 1,
            },
            Pos {
                r: pos.r,
                c: pos.c - 1,
            },
            Pos {
                r: pos.r + 1,
                c: pos.c,
            },
            Pos {
                r: pos.r - 1,
                c: pos.c,
            },
        ],
        '-' => vec![
            Pos {
                r: pos.r,
                c: pos.c + 1,
            },
            Pos {
                r: pos.r,
                c: pos.c - 1,
            },
        ],
        '|' => vec![
            Pos {
                r: pos.r + 1,
                c: pos.c,
            },
            Pos {
                r: pos.r - 1,
                c: pos.c,
            },
        ],
        'L' => vec![
            Pos {
                r: pos.r,
                c: pos.c + 1,
            },
            Pos {
                r: pos.r - 1,
                c: pos.c,
            },
        ],
        'J' => vec![
            Pos {
                r: pos.r,
                c: pos.c - 1,
            },
            Pos {
                r: pos.r - 1,
                c: pos.c,
            },
        ],
        '7' => vec![
            Pos {
                r: pos.r,
                c: pos.c - 1,
            },
            Pos {
                r: pos.r + 1,
                c: pos.c,
            },
        ],
        'F' => vec![
            Pos {
                r: pos.r,
                c: pos.c + 1,
            },
            Pos {
                r: pos.r + 1,
                c: pos.c,
            },
        ],
        _ => vec![],
    }
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

fn main() {
    let input = Input::from(std::io::stdin().lines());
    println!("{}", part1(input));
}
