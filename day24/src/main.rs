use std::{io::stdin, ops::Add, println, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
type Vector = Point;

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y, z)) = s
            .split(", ")
            .filter_map(|n| n.parse::<f64>().ok())
            .collect_tuple()
        {
            Ok(Point { x, y, z })
        } else {
            Err(())
        }
    }
}

struct Input {
    lines: Vec<(Point, Vector)>,
}

impl FromIterator<String> for Input {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let lines = iter
            .into_iter()
            .filter_map(|line| {
                line.split_once(" @ ").and_then(|(p, v)| {
                    Some((Point::from_str(p).unwrap(), Vector::from_str(v).unwrap()))
                })
            })
            .collect();
        Input { lines }
    }
}

fn intersection(
    x0: f64,
    y0: f64,
    dx0: f64,
    dy0: f64,
    x1: f64,
    y1: f64,
    dx1: f64,
    dy1: f64,
) -> Option<(f64, f64)> {
    if dx0 * dy1 == dx1 * dy0 {
        return None;
    }
    let t0 = ((x1 - x0) * dy1 + (y0 - y1) * dx1) / (dx0 * dy1 - dx1 * dy0);
    let t1 = (y0 - y1 + dy0 * t0) / dy1;
    Some((t0, t1))
}

fn part1(input: Input) -> usize {
    let low = 2.0 * 10.0f64.powi(14);
    let high = 4.0 * 10.0f64.powi(14);
    // let low = 7.0;
    // let high = 27.0;
    input
        .lines
        .iter()
        .combinations(2)
        .filter(|lines| {
            let &(p0, v0) = lines[0];
            let &(p1, v1) = lines[1];
            let Point { x: x0, y: y0, .. } = p0;
            let Point { x: x1, y: y1, .. } = p1;
            let Point { x: dx0, y: dy0, .. } = v0;
            let Point { x: dx1, y: dy1, .. } = v1;
            intersection(x0, y0, dx0, dy0, x1, y1, dx1, dy1).is_some_and(|(t0, t1)| {
                let x = x0 + t0 * dx0;
                let y = y0 + t0 * dy0;
                x >= low && x <= high && y >= low && y <= high && t0 > 0.0 && t1 > 0.0
            })
        })
        .count()
}

fn main() {
    let input = Input::from_iter(stdin().lines().filter_map(|line| line.ok()));
    println!("{}", part1(input));
}
