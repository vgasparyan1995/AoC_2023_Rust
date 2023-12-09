use num::integer::lcm;
use std::{
    collections::HashMap,
    io::Lines,
    io::{stdin, StdinLock},
};

type Label = String;
enum Direction {
    R,
    L,
}

struct Input {
    directions: Vec<Direction>,
    nodes: HashMap<Label, (Label, Label)>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(mut lines: Lines<StdinLock<'_>>) -> Self {
        let directions = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                'R' => Some(Direction::R),
                'L' => Some(Direction::L),
                _ => None,
            })
            .collect::<Vec<Direction>>();
        lines.next();
        let nodes = lines
            .map(|line| {
                let line = line.unwrap();
                let (src, dst) = line.split_once('=').unwrap();
                let src = src.trim().to_string();
                let (left, right) = dst
                    .trim()
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split_once(',')
                    .unwrap();
                let left = left.trim().to_string();
                let right = right.trim().to_string();
                (src, left, right)
            })
            .fold(HashMap::new(), |mut graph, (src, left, right)| {
                graph.insert(src, (left, right));
                graph
            });
        Self { directions, nodes }
    }
}

fn num_steps(
    nodes: &HashMap<Label, (Label, Label)>,
    directions: &Vec<Direction>,
    src: Label,
) -> usize {
    directions
        .iter()
        .cycle()
        .scan(src, |src, dir| {
            *src = match dir {
                Direction::L => nodes[src].clone().0,
                Direction::R => nodes[src].clone().1,
            };
            Some(src.clone())
        })
        .take_while(|src| !src.ends_with('Z'))
        .count()
        + 1
}

fn part1(input: Input) -> usize {
    num_steps(&input.nodes, &input.directions, "AAA".to_owned())
}

fn part2(input: Input) -> usize {
    input
        .nodes
        .keys()
        .filter(|src| src.ends_with('A'))
        .map(|src| num_steps(&input.nodes, &input.directions, src.clone()))
        .reduce(lcm).unwrap()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
