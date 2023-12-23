use std::{
    collections::VecDeque,
    fmt::Debug,
    io::{stdin, Lines, StdinLock},
    ops::Index,
    ops::IndexMut,
    panic, println, writeln,
};

type Row = Vec<char>;
#[derive(Clone, PartialEq, Eq)]
struct Mtx {
    rows: Vec<Row>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    r: isize,
    c: isize,
}

impl Index<Pos> for Mtx {
    type Output = char;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.rows[index.r as usize][index.c as usize]
    }
}

impl IndexMut<Pos> for Mtx {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.rows[index.r as usize][index.c as usize]
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

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.mtx
                .rows
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn swap(mtx: &mut Mtx, p1: Pos, p2: Pos) {
    let tmp = mtx[p1];
    mtx[p1] = mtx[p2];
    mtx[p2] = tmp;
}

fn move_towards(pos: Pos, dest: Pos) -> Pos {
    if pos.r == dest.r {
        Pos {
            r: pos.r,
            c: if dest.c > pos.c { pos.c + 1 } else { pos.c - 1 },
        }
    } else {
        Pos {
            r: if dest.r > pos.r { pos.r + 1 } else { pos.r - 1 },
            c: pos.c,
        }
    }
}

fn move_rocks(mtx: &mut Mtx, towards: Pos, from: Pos) -> isize {
    let rows = mtx.rows.len() as isize;
    let mut load = 0;
    let mut empty_slots = VecDeque::new();
    let mut pos = towards;
    while pos != from {
        match mtx[pos] {
            '#' => empty_slots.clear(),
            '.' => empty_slots.push_back(pos),
            'O' => {
                if let Some(p_swap) = empty_slots.pop_front() {
                    swap(mtx, pos, p_swap);
                    empty_slots.push_back(pos);
                    load += rows - p_swap.r;
                } else {
                    load += rows - pos.r;
                }
            }
            _ => panic!(),
        }
        pos = move_towards(pos, from);
    }
    load
}

fn move_rocks_up(mtx: &mut Mtx) -> isize {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    (0..cols)
        .map(|c| move_rocks(mtx, Pos { r: 0, c }, Pos { r: rows, c }))
        .sum()
}

fn move_rocks_down(mtx: &mut Mtx) -> isize {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    (0..cols)
        .map(|c| move_rocks(mtx, Pos { r: rows - 1, c }, Pos { r: -1, c }))
        .sum()
}

fn move_rocks_left(mtx: &mut Mtx) -> isize {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    (0..rows)
        .map(|r| move_rocks(mtx, Pos { r, c: 0 }, Pos { r, c: cols }))
        .sum()
}

fn move_rocks_right(mtx: &mut Mtx) -> isize {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    (0..rows)
        .map(|r| move_rocks(mtx, Pos { r, c: cols - 1 }, Pos { r, c: -1 }))
        .sum()
}

fn part1(mut input: Input) -> isize {
    move_rocks_up(&mut input.mtx)
}

fn part2(mut input: Input) -> isize {
    let mut states = Vec::new();
    let mut remaining = isize::max_value();
    for i in 0..1000000000 {
        move_rocks_up(&mut input.mtx);
        move_rocks_left(&mut input.mtx);
        move_rocks_down(&mut input.mtx);
        let load = move_rocks_right(&mut input.mtx);
        remaining -= 1;
        let curr_state = input.mtx.clone();
        if let Some((idx, _)) = states.iter().enumerate().find(|(_, s)| **s == curr_state) {
            let len_cycle = i - idx;
            remaining = (1000000000 - i as isize - 1) % len_cycle as isize;
        }
        if remaining == 0 {
            return load;
        }
        states.push(curr_state);
    }
    0
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
