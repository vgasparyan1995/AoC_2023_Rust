use std::{
    io::{Lines, StdinLock},
    ops::Add,
    println, vec,
};

use itertools::Itertools;

type Row = Vec<char>;
type Mtx = Vec<Row>;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos {
    r: isize,
    c: isize,
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
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
            Some(Pos {
                r: r as isize,
                c: c as isize,
            })
        } else {
            None
        }
    })
}

fn neighbors(mtx: &Mtx, pos: Pos) -> Vec<Pos> {
    let rows = mtx.len();
    let cols = mtx[0].len();
    let (r, c) = (pos.r as isize, pos.c as isize);
    match mtx[pos.r as usize][pos.c as usize] {
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
        if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
            Some(Pos { r, c })
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
            if pred(mtx[next.r as usize][next.c as usize]) {
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
            visit(mtx[curr.r as usize][curr.c as usize], curr);
            if pred(mtx[next.r as usize][next.c as usize]) {
                return true;
            }
        } else {
            return false;
        }
    }
}

fn area(vs: Vec<Pos>) -> isize {
    let length = vs.len();
    if length < 3 {
        return 0;
    }
    let mut s1 = 0;
    let mut s2 = 0;
    for i in 0..length - 1 {
        s1 += vs[i].r * vs[i + 1].c;
        s2 += vs[i].c * vs[i + 1].r;
    }
    s1 += vs[length - 1].r * vs[0].c;
    s2 += vs[length - 1].c * vs[0].r;
    (s1 - s2).abs() / 2
}

fn part2(mut input: Input) -> isize {
    let start = find(&input.mtx, 'S').unwrap();
    let neighbors = neighbors(&input.mtx, start);
    for next in neighbors {
        let mut polygon: Vec<Pos> = Vec::new();
        if traverse_visit(
            &mut input.mtx,
            start,
            next,
            |ch| ch == 'S',
            |_, pos| {
                polygon.push(pos);
            },
        ) {
            polygon.push(start);
            let boundary_points = polygon.len() as isize;
            let area = area(polygon);
            // A = i + b/2 - 1
            // thus
            // i = A - b/2 + 1
            return area - boundary_points / 2 + 1;
        }
    }
    0
}

fn main() {
    let input = Input::from(std::io::stdin().lines());
    println!("{}", part2(input));
}
