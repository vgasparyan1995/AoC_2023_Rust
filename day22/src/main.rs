use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Lines, StdinLock},
    ops::Add,
    println,
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let coords = value
            .split(",")
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<_>>();
        assert!(coords.len() == 3);
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

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

#[derive(Clone, Copy, Debug)]
struct Cuboid {
    p0: Point,
    p1: Point,
}

struct Input {
    cuboids: Vec<Cuboid>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let cuboids = lines
            .map(|line| {
                let line = line.unwrap();
                let (p0, p1) = line.split_once('~').unwrap();
                let p0 = Point::from(p0);
                let p1 = Point::from(p1) + Point { x: 1, y: 1, z: 1 };
                Cuboid { p0, p1 }
            })
            .collect();
        Input { cuboids }
    }
}

#[derive(Debug)]
struct Stacked {
    cuboid: Cuboid,
    cuboids_under: Vec<isize>,
    cuboids_over: Vec<isize>,
}

fn xy_spread(cuboids: &Vec<Cuboid>) -> (usize, usize) {
    let (x_min, y_min, x_max, y_max) = cuboids
        .iter()
        .map(|c| (c.p0.x, c.p0.y, c.p1.x, c.p1.y))
        .fold(
            (1000000, 1000000, 0, 0),
            |(x_min, y_min, x_max, y_max), (x0, y0, x1, y1)| {
                (x_min.min(x0), y_min.min(y0), x_max.max(x1), y_max.max(y1))
            },
        );
    ((x_max - x_min) as usize, (y_max - y_min) as usize)
}

fn stack(mut cuboids: Vec<Cuboid>) -> Vec<Stacked> {
    cuboids.sort_by_key(|cuboid| cuboid.p0.z);
    let (dx, dy) = xy_spread(&cuboids);
    let mut mask: Vec<Vec<isize>> = vec![vec![-1; dy]; dx];
    let mut stacked_cuboids: Vec<Stacked> = Vec::new();
    for cuboid in cuboids {
        let curr_idx = stacked_cuboids.len() as isize;
        let cuboids_under = (cuboid.p0.x..cuboid.p1.x)
            .cartesian_product(cuboid.p0.y..cuboid.p1.y)
            .map(|(x, y)| mask[x as usize][y as usize])
            .unique()
            .map(|idx| {
                if idx == -1 {
                    (idx, 0)
                } else {
                    (idx, stacked_cuboids[idx as usize].cuboid.p1.z)
                }
            })
            .collect::<Vec<_>>();
        let base_z = *cuboids_under.iter().map(|(_, z)| z).max().unwrap();
        let dz = Point {
            x: 0,
            y: 0,
            z: base_z - cuboid.p0.z,
        };
        let cuboid = Cuboid {
            p0: cuboid.p0 + dz,
            p1: cuboid.p1 + dz,
        };
        let cuboids_under = cuboids_under
            .into_iter()
            .filter_map(|(idx, z)| {
                if z == base_z && idx != -1 {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<isize>>();
        for &c_under in cuboids_under.iter() {
            stacked_cuboids[c_under as usize]
                .cuboids_over
                .push(curr_idx);
        }
        stacked_cuboids.push(Stacked {
            cuboid,
            cuboids_under,
            cuboids_over: Vec::new(),
        });
        for x in cuboid.p0.x..cuboid.p1.x {
            for y in cuboid.p0.y..cuboid.p1.y {
                mask[x as usize][y as usize] = curr_idx;
            }
        }
    }
    stacked_cuboids
}

fn part1(cuboids: Vec<Cuboid>) -> usize {
    let stacked_cuboids = stack(cuboids);
    stacked_cuboids
        .iter()
        .filter(|&Stacked { cuboids_over, .. }| {
            cuboids_over
                .iter()
                .all(|&cuboid_over| stacked_cuboids[cuboid_over as usize].cuboids_under.len() > 1)
        })
        .count()
}

fn remove(cuboid_idx: isize, stacked: &Vec<Stacked>) -> HashSet<isize> {
    let mut removed = HashSet::new();
    removed.insert(cuboid_idx);
    let mut queue = VecDeque::new();
    for &cuboid_over in stacked[cuboid_idx as usize].cuboids_over.iter() {
        queue.push_back(cuboid_over);
    }
    while let Some(cuboid_idx) = queue.pop_front() {
        let cuboid = &stacked[cuboid_idx as usize];
        if cuboid
            .cuboids_under
            .iter()
            .filter(|under| !removed.contains(under))
            .count()
            == 0
        {
            removed.insert(cuboid_idx);
        }
        for &over in cuboid.cuboids_over.iter() {
            queue.push_back(over);
        }
    }
    removed
}

fn part2(cuboids: Vec<Cuboid>) -> usize {
    let stacked_cuboids = stack(cuboids);
    (0..stacked_cuboids.len())
        .map(|cuboid_idx| remove(cuboid_idx as isize, &stacked_cuboids).len() - 1)
        .sum()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input.cuboids));
}
