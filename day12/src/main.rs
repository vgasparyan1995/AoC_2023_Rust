use cached::proc_macro::cached;
use std::{
    io::{stdin, Lines, StdinLock},
    println,
};

struct InputLine {
    pattern: String,
    numbers: Vec<usize>,
}
struct Input {
    input_lines: Vec<InputLine>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let input_lines = lines
            .filter_map(|line| {
                if let Some((row, numbers)) = line.unwrap().split_once(' ') {
                    let pattern = row.to_owned();
                    let numbers = numbers
                        .split(',')
                        .filter_map(|x| x.parse().ok())
                        .collect::<Vec<_>>();
                    return Some(InputLine { pattern, numbers });
                }
                None
            })
            .collect::<Vec<_>>();
        Self { input_lines }
    }
}

#[cached]
fn num_combinations(pattern: String, mut numbers: Vec<usize>) -> usize {
    if numbers.is_empty() {
        return if pattern.is_empty() || pattern.chars().all(|ch| ch == '.' || ch == '?') {
            1
        } else {
            0
        };
    }
    let last_len = numbers.pop().unwrap();
    if pattern.len() < last_len {
        return 0;
    }
    let last = std::iter::repeat('#').take(last_len).collect::<String>();
    let mut max_last_len = pattern.len();
    if !numbers.is_empty() {
        max_last_len -= numbers.iter().sum::<usize>() + numbers.len() - 1;
    }

    let mut total_combinations = 0;
    for dot_count in 0..=max_last_len - last.len() {
        let last_dots = std::iter::repeat('.').take(dot_count).collect::<String>();
        let suffix = format!(
            "{}{}{}",
            if numbers.is_empty() { "" } else { "." },
            last,
            last_dots
        );
        if does_match(suffix.as_str(), &pattern[pattern.len() - suffix.len()..]) {
            total_combinations += num_combinations(
                pattern[..pattern.len() - suffix.len()].to_owned(),
                numbers.clone(),
            );
        }
    }
    total_combinations
}

fn does_match(row: &str, pattern: &str) -> bool {
    let row = row.as_bytes();
    let pattern = pattern.as_bytes();
    assert!(row.len() == pattern.len());
    for i in 0..row.len() {
        if pattern[i] != '?' as u8 && pattern[i] != row[i] {
            return false;
        }
    }
    return true;
}

fn part1(input: Input) -> usize {
    input
        .input_lines
        .into_iter()
        .map(|InputLine { pattern, numbers }| num_combinations(pattern, numbers))
        .sum()
}

fn part2(input: Input) -> usize {
    input
        .input_lines
        .into_iter()
        .map(|InputLine { pattern, numbers }| {
            let pattern = std::iter::repeat(pattern)
                .take(5)
                .collect::<Vec<String>>()
                .join("?");
            let numbers = std::iter::repeat(numbers)
                .take(5)
                .flatten()
                .collect::<Vec<usize>>();
            num_combinations(pattern, numbers)
        })
        .sum()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
