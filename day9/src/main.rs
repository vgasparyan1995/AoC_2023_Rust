use std::{
    io::{stdin, Lines, StdinLock},
    iter,
    ops::Deref,
};

struct Input {
    arrs: Vec<Vec<i32>>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let arrs = lines
            .map(|line| {
                line.unwrap()
                    .split(' ')
                    .filter_map(|x| x.parse().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        Self { arrs }
    }
}

fn find_next(arr: &mut [i32]) -> i32 {
    if arr.iter().all(|&x| x == 0) {
        return 0;
    }
    for i in 0..arr.len() - 1 {
        arr[i] = arr[i + 1] - arr[i];
    }
    if let [head @ .., tail] = arr {
        return find_next(head) + *tail;
    }
    panic!()
}

fn part1(mut input: Input) -> i32 {
    input
        .arrs
        .iter_mut()
        .map(|arr| find_next(&mut arr[..]))
        .sum()
}

fn part2(mut input: Input) -> i32 {
    input
        .arrs
        .iter_mut()
        .map(|arr| {
            arr.reverse();
            find_next(&mut arr[..])
        })
        .sum()
}

fn adjacent_difference(arr: Vec<i32>) -> Vec<i32> {
    arr.windows(2)
        .filter_map(|pairs| match pairs {
            [a, b] => Some(b - a),
            _ => None,
        })
        .collect()
}

fn find_next_rec(arr: Vec<i32>) -> i32 {
    if arr.is_empty() || arr.iter().all(|&x| x == 0) {
        return 0;
    }
    let last = *arr.last().unwrap();
    last + find_next_rec(adjacent_difference(arr))
}

fn part1_rec(input: Input) -> i32 {
    input.arrs.into_iter().map(|arr| find_next_rec(arr)).sum()
}

fn part2_rec(input: Input) -> i32 {
    input
        .arrs
        .into_iter()
        .map(|mut arr| {
            arr.reverse();
            find_next_rec(arr)
        })
        .sum()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2_rec(input));
}
