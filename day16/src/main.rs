use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Lines, StdinLock},
    ops::{Add, Sub},
    panic, println,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    r: isize,
    c: isize,
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            c: self.c - rhs.c,
        }
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

// Intended for direction vectors, i.e. {0, 1}, {0, -1}, {1, 0}, {-1, 0}
fn clockwise_90(p: Pos) -> Pos {
    Pos { r: p.c, c: -p.r }
}

// Intended for direction vectors, i.e. {0, 1}, {0, -1}, {1, 0}, {-1, 0}
fn counterclockwise_90(p: Pos) -> Pos {
    Pos { r: -p.c, c: p.r }
}

type Row = Vec<char>;
struct Mtx {
    rows: Vec<Row>,
}

impl Mtx {
    fn get(&self, index: Pos) -> Option<char> {
        let rows = self.rows.len() as isize;
        let cols = self.rows[0].len() as isize;
        if index.r >= 0 && index.r < rows && index.c >= 0 && index.c < cols {
            Some(self.rows[index.r as usize][index.c as usize])
        } else {
            None
        }
    }
}

struct Input {
    mtx: Mtx,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let rows = lines
            .into_iter()
            .map(|line| line.unwrap().chars().collect::<Row>())
            .collect::<Vec<_>>();
        Self { mtx: Mtx { rows } }
    }
}

fn visit(
    mtx: &Mtx,
    from: Pos,
    to: Pos,
    mut visited: HashSet<(Pos, Pos)>,
    mut energized: HashSet<Pos>,
) -> (HashSet<(Pos, Pos)>, HashSet<Pos>) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((from, to));
    while let Some((from, to)) = to_visit.pop_front() {
        if visited.contains(&(from, to)) {
            continue;
        }
        let ch = mtx.get(to);
        if ch.is_none() {
            continue;
        }

        visited.insert((from, to));
        energized.insert(to);
        let straight_ahead = to - from;
        let turn_left = counterclockwise_90(straight_ahead);
        let turn_right = clockwise_90(straight_ahead);
        match ch.unwrap() {
            '.' => to_visit.push_back((to, to + straight_ahead)),
            '-' => {
                if straight_ahead.r == 0 {
                    to_visit.push_back((to, to + straight_ahead));
                } else {
                    to_visit.push_back((to, to + turn_left));
                    to_visit.push_back((to, to + turn_right));
                }
            }
            '|' => {
                if straight_ahead.c == 0 {
                    to_visit.push_back((to, to + straight_ahead));
                } else {
                    to_visit.push_back((to, to + turn_left));
                    to_visit.push_back((to, to + turn_right));
                }
            }
            '/' => {
                if straight_ahead.r == 0 {
                    to_visit.push_back((to, to + turn_left));
                } else {
                    to_visit.push_back((to, to + turn_right));
                }
            }
            '\\' => {
                if straight_ahead.r == 0 {
                    to_visit.push_back((to, to + turn_right));
                } else {
                    to_visit.push_back((to, to + turn_left));
                }
            }
            _ => panic!(),
        };
    }
    (visited, energized)
}

fn count_energy(mtx: &Mtx, start: Pos, from: Pos) -> usize {
    visit(
        mtx,
        from,
        start,
        HashSet::<(Pos, Pos)>::new(),
        HashSet::<Pos>::new(),
    )
    .1
    .len()
}

fn part1(input: Input) -> usize {
    count_energy(&input.mtx, Pos { r: 0, c: 0 }, Pos { r: 0, c: -1 })
}

fn part2(input: Input) -> usize {
    let rows = input.mtx.rows.len() as isize;
    let cols = input.mtx.rows[0].len() as isize;
    let mut max_energy = 0;
    for c in 0..cols {
        max_energy = max_energy.max(count_energy(&input.mtx, Pos { r: 0, c }, Pos { r: -1, c }));
        max_energy = max_energy.max(count_energy(
            &input.mtx,
            Pos { r: rows - 1, c },
            Pos { r: rows, c },
        ));
    }
    for r in 0..rows {
        max_energy = max_energy.max(count_energy(&input.mtx, Pos { r, c: 0 }, Pos { r, c: -1 }));
        max_energy = max_energy.max(count_energy(
            &input.mtx,
            Pos { r, c: cols - 1 },
            Pos { r, c: cols },
        ));
    }
    max_energy
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
