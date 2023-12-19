use std::{io::stdin, ops::IndexMut};

enum Operation {
    Remove,
    Insert(i32),
}
struct Step {
    hash: u8,
    label_hash: u8,
    key: String,
    operation: Operation,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let h = hash(value);
        match value.split_once("=") {
            Some((key, value)) => Self {
                hash: h,
                label_hash: hash(key),
                key: key.to_owned(),
                operation: Operation::Insert(value.parse().unwrap()),
            },
            None => {
                let key = value.strip_suffix("-").unwrap().to_owned();
                Self {
                    hash: h,
                    label_hash: hash(key.as_str()),
                    key,
                    operation: Operation::Remove,
                }
            }
        }
    }
}

type Input = Vec<Step>;

fn hash(step: &str) -> u8 {
    step.bytes()
        .into_iter()
        .fold(0, |res, curr| (res + curr as i32) * 17 % 256) as u8
}

fn part1(input: Input) -> i32 {
    input.into_iter().map(|step| step.hash as i32).sum()
}

fn part2(input: Input) -> i32 {
    let hash_table: Vec<Vec<(String, i32)>> = vec![vec![]; 256];
    let hash_table = input.into_iter().fold(hash_table, |mut acc, step| {
        let list = acc.index_mut(step.label_hash as usize);
        match step.operation {
            Operation::Remove => {
                if let Some((idx, _)) = list
                    .iter()
                    .enumerate()
                    .filter(|(_, (k, _))| *k == step.key)
                    .next()
                {
                    list.remove(idx);
                }
            }
            Operation::Insert(value) => {
                if let Some((_, v)) = list.iter_mut().filter(|(k, _)| *k == step.key).next() {
                    *v = value;
                } else {
                    list.push((step.key, value));
                }
            }
        };
        acc
    });
    hash_table
        .into_iter()
        .enumerate()
        .map(|(list_idx, list)| {
            (list_idx + 1) as i32
                * list
                    .into_iter()
                    .enumerate()
                    .map(|(slot_idx, (_, strength))| (slot_idx + 1) as i32 * strength)
                    .sum::<i32>()
        })
        .sum()
}

fn main() {
    let input = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(Step::from)
        .collect::<Input>();

    println!("{}", part2(input));
}