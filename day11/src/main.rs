use std::{
    collections::BTreeSet,
    io::{stdin, Lines, StdinLock},
    println,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos {
    r: usize,
    c: usize,
}
type Row = Vec<char>;
type Mtx = Vec<Row>;
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

fn distance(
    p1: Pos,
    p2: Pos,
    empty_rows: &BTreeSet<usize>,
    empty_cols: &BTreeSet<usize>,
    extend_rate: usize,
) -> usize {
    let min_r = p1.r.min(p2.r);
    let max_r = p1.r.max(p2.r);
    let min_c = p1.c.min(p2.c);
    let max_c = p1.c.max(p2.c);

    (max_r - min_r)
        + (max_c - min_c)
        + empty_rows.range(min_r..max_r).count() * extend_rate
        + empty_cols.range(min_c..max_c).count() * extend_rate
}

fn solve(input: Input, extend_rate: usize) -> usize {
    let mtx = input.mtx;
    let rows = mtx.len();
    let cols = mtx[0].len();

    let mut galaxies = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if mtx[r][c] == '#' {
                galaxies.push(Pos { r, c });
            }
        }
    }

    let empty_rows = (0..rows)
        .filter(|&r| mtx[r].iter().all(|&ch| ch == '.'))
        .collect::<BTreeSet<_>>();
    let empty_cols = (0..cols)
        .filter(|&c| (0..rows).all(|r| mtx[r][c] == '.'))
        .collect::<BTreeSet<_>>();

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_distance += distance(
                galaxies[i],
                galaxies[j],
                &empty_rows,
                &empty_cols,
                extend_rate,
            );
        }
    }
    total_distance
}

fn main() {
    let input = Input::from(stdin().lines());
    // println!("{}", solve(input, 1));
    println!("{}", solve(input, 999999));
}
