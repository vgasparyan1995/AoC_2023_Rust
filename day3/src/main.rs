use itertools::Itertools;
use std::{io, println};

fn get(mtx: &Vec<Vec<char>>, r: i32, c: i32) -> char {
    if r < 0 || c < 0 {
        return '.';
    }
    *mtx.get(r as usize)
        .and_then(|row| row.get(c as usize))
        .unwrap_or(&'.')
}

fn ctoi(c: char) -> i32 {
    c as i32 - '0' as i32
}

fn part1(mtx: Vec<Vec<char>>) -> i32 {
    let mut valid_numbers = Vec::new();
    let num_rows = mtx.len() as i32;
    let num_cols = mtx[0].len() as i32;
    for row in 0..num_rows {
        let mut number = 0;
        let mut is_valid = false;
        for col in 0..num_cols {
            let c = get(&mtx, row, col);
            if c.is_ascii_digit() {
                number = number * 10 + ctoi(c);
                is_valid |= (row - 1..=row + 1)
                    .cartesian_product(col - 1..=col + 1)
                    .map(|(r, c)| get(&mtx, r, c))
                    .any(|ch| ch != '.' && !ch.is_ascii_digit());
            } else if number != 0 {
                if is_valid {
                    valid_numbers.push(number);
                } else {
                    println!("invalid number: {}", number);
                }
                number = 0;
                is_valid = false;
            }
        }
        if is_valid {
            valid_numbers.push(number);
        }
    }
    println!("{valid_numbers:?}");
    valid_numbers.iter().sum()
}

fn main() {
    let mtx: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    println!("{}", part1(mtx));
}
