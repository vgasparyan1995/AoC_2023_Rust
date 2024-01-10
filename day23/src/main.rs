use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Lines, StdinLock},
    ops::Add,
    panic, println, vec,
};

use itertools::{self, Itertools};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

const UP: Pos = Pos { r: -1, c: 0 };
const DOWN: Pos = Pos { r: 1, c: 0 };
const LEFT: Pos = Pos { r: 0, c: -1 };
const RIGHT: Pos = Pos { r: 0, c: 1 };

fn turn_left(dir: Dir) -> Dir {
    match dir {
        UP => LEFT,
        LEFT => DOWN,
        DOWN => RIGHT,
        RIGHT => UP,
        _ => panic!(),
    }
}

fn turn_right(dir: Dir) -> Dir {
    match dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => panic!(),
    }
}

type Dir = Pos;

struct Matrix {
    rows: Vec<Vec<char>>,
}

impl Matrix {
    fn get(&self, pos: Pos) -> char {
        self.rows[pos.r as usize][pos.c as usize]
    }

    fn try_get(&self, pos: Pos) -> Option<char> {
        self.rows
            .get(pos.r as usize)
            .and_then(|row| row.get(pos.c as usize).copied())
    }
}

struct Input {
    mtx: Matrix,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        Input {
            mtx: Matrix {
                rows: lines.map(|line| line.unwrap().chars().collect()).collect(),
            },
        }
    }
}

fn part1(mtx: &Matrix) -> usize {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    let dest_pos: Pos = Pos {
        r: rows,
        c: cols - 2,
    };
    let mut max_path = 0;
    let mut num_steps = 0;
    let mut paths = vec![(Pos { r: 0, c: 1 }, DOWN)];
    while !paths.is_empty() {
        paths = paths
            .into_iter()
            .map(|(pos, dir)| {
                let field = mtx.get(pos);
                let directions = match field {
                    '.' => vec![dir, turn_left(dir), turn_right(dir)],
                    '>' => {
                        if dir == LEFT {
                            vec![]
                        } else {
                            vec![RIGHT]
                        }
                    }
                    '<' => {
                        if dir == RIGHT {
                            vec![]
                        } else {
                            vec![LEFT]
                        }
                    }
                    '^' => {
                        if dir == DOWN {
                            vec![]
                        } else {
                            vec![UP]
                        }
                    }
                    'v' => {
                        if dir == UP {
                            vec![]
                        } else {
                            vec![DOWN]
                        }
                    }
                    _ => panic!(),
                };
                directions
                    .into_iter()
                    .map(|dir| (pos + dir, dir))
                    .filter_map(|(next_pos, dir)| {
                        if next_pos == dest_pos {
                            max_path = num_steps;
                            return None;
                        }
                        if mtx.get(next_pos) == '#' {
                            return None;
                        }
                        Some((next_pos, dir))
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        num_steps += 1;
    }
    max_path
}

fn is_path(ch: char) -> bool {
    ch != '#'
}

type Vertex = Pos;
#[derive(Debug, Clone, Copy)]
struct Edge {
    neighbor: Vertex,
    distance: i32,
}
type Graph = HashMap<Vertex, Vec<Edge>>;

fn find_vertices(mtx: &Matrix) -> HashSet<Vertex> {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    let start_pos = Pos { r: 0, c: 1 };
    let dest_pos = Pos {
        r: rows - 1,
        c: cols - 2,
    };
    let mut vertices: HashSet<Pos> = (1..rows - 1)
        .cartesian_product(1..cols - 1)
        .filter_map(|(r, c)| {
            let pos = Pos { r, c };
            if !is_path(mtx.get(pos)) {
                return None;
            }
            let path_neighbors = [pos + UP, pos + DOWN, pos + LEFT, pos + RIGHT]
                .into_iter()
                .filter(|&pos| is_path(mtx.get(pos)))
                .count();
            assert!(path_neighbors >= 2);
            if path_neighbors == 2 {
                return None;
            }
            Some(pos)
        })
        .collect();
    vertices.insert(start_pos);
    vertices.insert(dest_pos);
    vertices
}

fn make_connections(vertices: HashSet<Vertex>, mtx: &Matrix) -> Graph {
    let vertices_clone = vertices.clone();
    vertices
        .into_iter()
        .map(|v| {
            let neigbors = [UP, DOWN, LEFT, RIGHT]
                .into_iter()
                .filter_map(|dir| {
                    let mut curr_dir = dir;
                    let mut next = v + curr_dir;
                    if !mtx.try_get(next).is_some_and(is_path) {
                        return None;
                    }
                    let mut distance = 1;
                    while !vertices_clone.contains(&next) {
                        distance += 1;
                        (next, curr_dir) = [curr_dir, turn_left(curr_dir), turn_right(curr_dir)]
                            .into_iter()
                            .find_map(|dir| {
                                let next_pos = next + dir;
                                if mtx.try_get(next_pos).is_some_and(is_path) {
                                    return Some((next_pos, dir));
                                }
                                None
                            })
                            .unwrap();
                    }
                    Some(Edge {
                        neighbor: next,
                        distance,
                    })
                })
                .collect_vec();
            (v, neigbors)
        })
        .collect()
}

fn build_graph(mtx: &Matrix) -> Graph {
    let vertices = find_vertices(mtx);
    make_connections(vertices, mtx)
}

fn longest_path(
    graph: &Graph,
    from: Pos,
    to: Pos,
    mut visited: HashSet<Pos>,
    distance_so_far: i32,
) -> (HashSet<Pos>, i32) {
    if from == to {
        return (visited, distance_so_far);
    }
    let mut longest_distance = 0;
    visited.insert(from);
    for &Edge { neighbor, distance } in graph[&from].iter() {
        if visited.contains(&neighbor) {
            continue;
        }
        let (new_visited, new_distance) =
            longest_path(graph, neighbor, to, visited, distance_so_far + distance);
        visited = new_visited;
        longest_distance = longest_distance.max(new_distance);
    }
    visited.remove(&from);
    (visited, longest_distance)
}

fn part2(mtx: &Matrix) -> i32 {
    let rows = mtx.rows.len() as isize;
    let cols = mtx.rows[0].len() as isize;
    let start_pos = Pos { r: 0, c: 1 };
    let dest_pos = Pos {
        r: rows - 1,
        c: cols - 2,
    };
    let graph = build_graph(mtx);
    let (_, distance) = longest_path(&graph, start_pos, dest_pos, HashSet::new(), 0);
    distance
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part1(&input.mtx));
    println!("{}", part2(&input.mtx));
}
