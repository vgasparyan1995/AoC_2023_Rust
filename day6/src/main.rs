use std::{io::{self, Lines, StdinLock}, println};

struct Race {
    time: i32,
    distance: i32,
}

struct Input {
    races: Vec<Race>,
}

impl From<Lines<StdinLock<'static>>> for Input {
    fn from(lines: Lines<StdinLock<'static>>) -> Self {
        let mut lines = lines.into_iter();
        let times: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();
        let distances: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();
        Input {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(t, d)| Race {
                    time: t,
                    distance: d,
                })
                .collect(),
        }
    }
}

fn part1(input: Input) -> i32 {
    0
}

fn part2(input: Input) -> i32 {
    0
}

fn main() {
    let input = Input::from(io::stdin().lines());
    println!("{}", part1(input));
}
