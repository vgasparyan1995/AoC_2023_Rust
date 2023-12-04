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

fn part2(input: Vec<(u32, HashSet<u32>, HashSet<u32>)>) -> u32 {
    let mut cards = vec![1; input.len()];
    input
        .into_iter()
        .map(|(id, winning, found)| {
            let num_matches = found
                .into_iter()
                .filter(|found_number| winning.contains(found_number))
                .count() as usize;
            (id as usize, num_matches)
        })
        .for_each(|(id, num)| {
            if num != 0 {
                for idx in id..(id + num) {
                    cards[idx] += cards[id - 1];
                }
            }
        });
    cards.into_iter().sum()
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
    println!("{}", part2(input));
}
