use std::{
    io::{self, BufRead},
    println,
};

fn part1() -> u32 {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let digits: String = line.unwrap().chars().filter(|c| c.is_digit(10)).collect();
            let first_digit = digits.chars().next().unwrap().to_digit(10).unwrap();
            let last_digit = digits
                .chars()
                .last()
                .map_or(first_digit, |d| d.to_digit(10).unwrap());
            first_digit * 10 + last_digit
        })
        .sum()
}

fn part2() -> u32 {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut substr = line.as_str();
            let digits: String = std::iter::from_fn(|| match substr {
                "" => None,
                _ => {
                    let curr = substr;
                    substr = &substr[1..];
                    Some(curr)
                }
            })
            .filter_map(|substr| match substr {
                "" => None,
                _ if substr.starts_with("one") => Some('1'),
                _ if substr.starts_with("two") => Some('2'),
                _ if substr.starts_with("three") => Some('3'),
                _ if substr.starts_with("four") => Some('4'),
                _ if substr.starts_with("five") => Some('5'),
                _ if substr.starts_with("six") => Some('6'),
                _ if substr.starts_with("seven") => Some('7'),
                _ if substr.starts_with("eight") => Some('8'),
                _ if substr.starts_with("nine") => Some('9'),
                _ if substr.chars().next().unwrap().is_digit(10) => {
                    Some(substr.chars().next().unwrap())
                }
                _ => None,
            })
            .collect();
            let first_digit = digits.chars().next().unwrap().to_digit(10).unwrap();
            let last_digit = digits
                .chars()
                .last()
                .map_or(first_digit, |d| d.to_digit(10).unwrap());
            first_digit * 10 + last_digit
        })
        .sum()
}

fn main() {
    println!("{}", part2());
}
