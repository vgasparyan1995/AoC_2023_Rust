use std::{collections::HashSet, io};

fn part1(input: Vec<(u32, HashSet<u32>, HashSet<u32>)>) -> u32 {
    input
        .into_iter()
        .map(|(_, winning, found)| {
            found
                .into_iter()
                .filter(|found_number| winning.contains(found_number))
                .count() as u32
        })
        .map(|n| match n {
            0 => 0,
            _ => (2 as u32).pow(n - 1),
        })
        .sum()
}

fn main() {
    let input: Vec<(u32, HashSet<u32>, HashSet<u32>)> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (card, numbers) = line.split_once(':').unwrap();
            let id: u32 = card.split(' ').last().unwrap().parse().unwrap();
            let (winning, found) = numbers.split_once('|').unwrap();
            let winning_numbers: HashSet<u32> = winning
                .trim()
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .collect();
            let found_numbers: HashSet<u32> = found
                .trim()
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .collect();
            (id, winning_numbers, found_numbers)
        })
        .collect();
    println!("{}", part1(input));
}
