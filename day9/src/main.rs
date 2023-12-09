use std::io::{stdin, Lines, StdinLock};

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

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
