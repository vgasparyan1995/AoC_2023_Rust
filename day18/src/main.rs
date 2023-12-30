use std::{
    io::{stdin, Lines, StdinLock},
    ops::{Add, Mul},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
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

impl Mul<isize> for Pos {
    type Output = Pos;
    fn mul(self, rhs: isize) -> Self::Output {
        Pos {
            r: self.r * rhs,
            c: self.c * rhs,
        }
    }
}

const UP: Pos = Pos { r: -1, c: 0 };
const DOWN: Pos = Pos { r: 1, c: 0 };
const RIGHT: Pos = Pos { r: 0, c: 1 };
const LEFT: Pos = Pos { r: 0, c: -1 };

struct Color(String);
struct Edge(Pos, isize, Color);
struct Input {
    edges: Vec<Edge>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        Self {
            edges: lines
                .filter_map(|line| {
                    let line = line.unwrap();
                    let components = line.split(" ").collect::<Vec<_>>();
                    if components.len() != 3 {
                        return None;
                    }
                    let direction = match components[0] {
                        "R" => Some(RIGHT),
                        "L" => Some(LEFT),
                        "U" => Some(UP),
                        "D" => Some(DOWN),
                        _ => None,
                    };
                    let length = components[1].parse::<isize>().ok();
                    let color = components[2].strip_prefix("(").unwrap().strip_suffix(")");
                    if direction.is_some() && length.is_some() && color.is_some() {
                        return Some(Edge(
                            direction.unwrap(),
                            length.unwrap(),
                            Color(color.unwrap().to_owned()),
                        ));
                    }
                    None
                })
                .collect(),
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

fn part1(input: Input) -> isize {
    let mut boundary_points = 0;
    let mut vertices = Vec::new();
    let mut curr = Pos::default();
    for Edge(dir, length, _) in input.edges {
        vertices.push(curr);
        curr = curr + dir * length;
        boundary_points += length;
    }
    let a = area(vertices);
    a + 1 + boundary_points / 2
}

fn part2(input: Input) -> isize {
    let mut boundary_points = 0;
    let mut vertices = Vec::new();
    let mut curr = Pos::default();
    for (dir, length) in input.edges.into_iter().map(|Edge(_, _, Color(color))| {
        let dir = match color.chars().last().unwrap() {
            '0' => RIGHT,
            '1' => DOWN,
            '2' => LEFT,
            '3' => UP,
            _ => panic!(),
        };
        let length = color.chars().skip(1).take(5).collect::<String>();
        let length = isize::from_str_radix(length.as_str(), 16).unwrap();
        (dir, length)
    }) {
        vertices.push(curr);
        curr = curr + dir * length;
        boundary_points += length;
    }
    let a = area(vertices);
    // a = i + b/2 - 1
    // i + b = a + 1 + b/2
    a + 1 + boundary_points / 2
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}