use std::{
    collections::HashMap,
    io::{self, BufRead},
    println,
};

fn parse_game(game: String) -> (i32, HashMap<&'static str, i32>) {
    let (game_id, hands) = game.split_once(':').unwrap();
    let game_id: i32 = game_id.split_once(' ').unwrap().1.parse().unwrap();
    let cubes = hands
        .split(';')
        .map(|hand| {
            hand.split(',')
                .map(|n_cubes| {
                    n_cubes
                        .trim()
                        .split_once(' ')
                        .map(|(n, color)| (color, n.parse().unwrap()))
                        .unwrap()
                })
                .collect::<HashMap<&str, i32>>()
        })
        .fold(
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]),
            |acc, hand| {
                acc.into_iter()
                    .map(|(color, curr_max): (&str, i32)| {
                        (color, curr_max.max(*hand.get(color).unwrap_or(&0)))
                    })
                    .collect()
            },
        );
    (game_id, cubes)
}

fn is_possible(observed: &HashMap<&str, i32>, limit: &HashMap<&str, i32>) -> bool {
    println!("observed: {observed:?}");
    println!("limit: {limit:?}");
    observed
        .into_iter()
        .all(|(color, n)| limit.get(color).map(|limit| n <= limit).unwrap_or(false))
}

fn part1() -> i32 {
    io::stdin()
        .lock()
        .lines()
        .map(|line| parse_game(line.unwrap()))
        .filter(|(id, cubes)| {
            println!("id: {id}");
            is_possible(
                cubes,
                &HashMap::from([("red", 12), ("green", 13), ("blue", 14)]),
            )
        })
        .map(|(id, _)| id)
        .sum()
}

fn part2() -> i32 {
    io::stdin()
        .lock()
        .lines()
        .map(|line| parse_game(line.unwrap()))
        .map(|(_, cubes)| cubes.values().product::<i32>())
        .sum()
}

fn main() {
    println!("{}", part2());
}
