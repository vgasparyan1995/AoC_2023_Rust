use std::{
    collections::HashSet,
    io::{stdin, Lines, StdinLock},
    ops::Add,
    println, vec,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos {
    r: isize,
    c: isize,
}

const UP: Pos = Pos { r: -1, c: 0 };
const DOWN: Pos = Pos { r: 1, c: 0 };
const LEFT: Pos = Pos { r: 0, c: -1 };
const RIGHT: Pos = Pos { r: 0, c: 1 };

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

type Row = Vec<char>;
struct Mtx {
    rows: Vec<Row>,
}

impl Mtx {
    fn get(&self, pos: Pos) -> char {
        let rows = self.rows.len() as isize;
        let cols = self.rows[0].len() as isize;
        self.rows[pos.r.rem_euclid(rows) as usize][pos.c.rem_euclid(cols) as usize]
    }
}

struct Input {
    mtx: Mtx,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let rows = lines.map(|line| line.unwrap().chars().collect()).collect();
        Input { mtx: Mtx { rows } }
    }
}

fn find_start(mtx: &Mtx) -> Pos {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    for r in 0..rows {
        for c in 0..cols {
            if mtx.get(Pos { r, c }) == 'S' {
                return Pos { r, c };
            }
        }
    }
    Pos { r: 0, c: 0 }
}

fn count_reachable(mtx: &Mtx, start: Pos, steps: isize) -> i64 {
    let mut positions = vec![start];
    for _ in 0..steps {
        positions = positions
            .into_iter()
            .map(|p| vec![p + UP, p + DOWN, p + LEFT, p + RIGHT].into_iter())
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|&p| mtx.get(p) != '#')
            .collect();
    }
    positions.len() as i64
}

fn part1(mtx: Mtx) -> i64 {
    count_reachable(&mtx, find_start(&mtx), 64)
}

fn part2(mtx: Mtx) -> i64 {
    let steps = 26501365;
    // assume 'steps' is 65 + 131k
    assert!((steps - 65) % 131 == 0);
    let k = (steps - 65) / 131; // 202300
    let start = find_start(&mtx);
    let mut f = vec![
        count_reachable(&mtx, start, 65 + 0 * 131),
        count_reachable(&mtx, start, 65 + 1 * 131),
        count_reachable(&mtx, start, 65 + 2 * 131),
    ];
    println!("{f:?}");
    // assume f(x) is a quadratic function
    let double_derivative = (f[2] - f[1]) - (f[1] - f[0]);
    for _ in 3..=k {
        let f3 = f[2] + f[2] - f[1] + double_derivative;
        f[0] = f[1];
        f[1] = f[2];
        f[2] = f3;
    }
    f[2]
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input.mtx));
}
