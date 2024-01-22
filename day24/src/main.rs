use std::{assert_eq, collections::VecDeque, io::stdin, ops::Add, println, str::FromStr};

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

fn prepare_eq_system(equations: &mut VecDeque<VecDeque<f64>>) {
    let idx = equations
        .iter()
        .find_position(|&eq| eq.iter().next().is_some_and(|&a| a != 0.0))
        .unwrap()
        .0;
    equations.swap(0, idx);
    for equation in equations {
        let a = equation[0];
        if a == 0.0 {
            continue;
        }
        for coeff in equation {
            *coeff /= a;
        }
    }
}

fn solve_eq_system(mut equations: VecDeque<VecDeque<f64>>) -> VecDeque<f64> {
    assert!(!equations.is_empty());
    prepare_eq_system(&mut equations);
    let first_equation = equations.pop_front().unwrap();
    assert_eq!(first_equation[0], 1.0);
    if equations.is_empty() {
        assert_eq!(first_equation.len(), 2);
        let b = first_equation[1];
        return [-b].into();
    }
    assert_eq!(first_equation.len(), equations.len() + 2);
    for equation in equations.iter_mut() {
        if equation[0] == 0.0 {
            equation.pop_front();
            continue;
        }
        equation
            .iter_mut()
            .zip(first_equation.iter())
            .for_each(|(coeff, coeff0)| *coeff -= coeff0);
        assert_eq!(equation[0], 0.0);
        equation.pop_front();
    }
    let mut values = solve_eq_system(equations);
    let x = -first_equation
        .iter()
        .skip(1)
        .zip(values.iter().chain([&1.0]))
        .map(|(coeff, value)| coeff * value)
        .sum::<f64>();
    values.push_front(x);
    values
}

fn part2(input: Input) -> i64 {
    #[rustfmt::skip]
    let ( Point { x: x1, y: y1, z: z1, }, Point { x: dx1, y: dy1, z: dz1, },) = input.lines[0];
    #[rustfmt::skip]
    let ( Point { x: x2, y: y2, z: z2, }, Point { x: dx2, y: dy2, z: dz2, },) = input.lines[1];
    #[rustfmt::skip]
    let ( Point { x: x3, y: y3, z: z3, }, Point { x: dx3, y: dy3, z: dz3, },) = input.lines[2];
    #[rustfmt::skip]
    let solution = solve_eq_system([
        [dy1 - dy2,     dx2 - dx1,        0.0,  y2 - y1,    x1 - x2,        0.0,    - x1 * dy1 + x2 * dy2 + y1 * dx1 - y2 * dx2].into(),
        [dz1 - dz2,           0.0,  dx2 - dx1,  z2 - z1,        0.0,    x1 - x2,    - x1 * dz1 + x2 * dz2 + z1 * dx1 - z2 * dx2].into(),
        [      0.0,     dz1 - dz2,  dy2 - dy1,      0.0,    z2 - z1,    y1 - y2,    - y1 * dz1 + y2 * dz2 + z1 * dy1 - z2 * dy2].into(),
        [dy1 - dy3,     dx3 - dx1,        0.0,  y3 - y1,    x1 - x3,        0.0,    - x1 * dy1 + x3 * dy3 + y1 * dx1 - y3 * dx3].into(),
        [dz1 - dz3,           0.0,  dx3 - dx1,  z3 - z1,        0.0,    x1 - x3,    - x1 * dz1 + x3 * dz3 + z1 * dx1 - z3 * dx3].into(),
        [      0.0,     dz1 - dz3,  dy3 - dy1,      0.0,    z3 - z1,    y1 - y3,    - y1 * dz1 + y3 * dz3 + z1 * dy1 - z3 * dy3].into(),
    ].into());
    println!("{solution:?}");
    let result = solution[0] + solution[1] + solution[2];
    println!("result: {result}");
    result as i64
}

fn main() {
    let input = Input::from_iter(stdin().lines().filter_map(|line| line.ok()));
    println!("{}", part2(input));
}
