use std::{
    io::{self, Lines, StdinLock},
    println,
};

#[derive(Copy, Clone, Debug)]
struct Race {
    time: i64,
    distance: i64,
}

struct Input {
    races: Vec<Race>,
    one_race: Race,
}

fn read_vec(s: &str) -> Vec<i64> {
    s.trim()
        .split(' ')
        .filter_map(|num| num.parse().ok())
        .collect()
}

impl From<Lines<StdinLock<'static>>> for Input {
    fn from(lines: Lines<StdinLock<'static>>) -> Self {
        let mut lines = lines.into_iter();
        let times_str = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .to_owned();
        let times: Vec<i64> = read_vec(times_str.as_str());
        let one_time: i64 = times_str
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>()
            .parse()
            .unwrap();
        let distances_str = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .to_owned();
        let distances: Vec<i64> = read_vec(distances_str.as_str());
        let one_distance: i64 = distances_str
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>()
            .parse()
            .unwrap();
        Input {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(t, d)| Race {
                    time: t,
                    distance: d,
                })
                .collect(),
            one_race: Race {
                time: one_time,
                distance: one_distance,
            },
        }
    }
}

fn number_of_ways_to_win(race: Race) -> i64 {
    let press_times: Vec<i64> = (0..=race.time / 2).collect();
    let pp = press_times.partition_point(|press| press * (race.time - press) <= race.distance);
    let num_ways_half = (press_times.len() - pp) as i64;
    let result = if race.time % 2 == 0 {
        num_ways_half * 2 - 1
    } else {
        num_ways_half * 2
    };
    println!("race: {race:?}, num_ways: {result}");
    result
}

fn part1(input: Input) -> i64 {
    input.races.into_iter().map(number_of_ways_to_win).product()
}

fn part2(input: Input) -> i64 {
    number_of_ways_to_win(input.one_race)
}

fn main() {
    let input = Input::from(io::stdin().lines());
    println!("{}", part2(input));
}
