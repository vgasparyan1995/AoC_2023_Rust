use std::{
    collections::BTreeMap,
    io::stdin,
    io::{Lines, StdinLock},
};

struct Input {
    seeds: Vec<i32>,
    maps: Vec<BTreeMap<i32, (i32, i32)>>,
}

impl From<Lines<StdinLock<'static>>> for Input {
    fn from(mut lines: Lines<StdinLock<'static>>) -> Self {
        let seeds = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();
        let mut maps: Vec<BTreeMap<i32, (i32, i32)>> = Vec::new();
        while (lines.next().is_some()) {
            lines.find(|line| line.unwrap().starts_with("seed-to_soil"));
            let map = lines.take_while(|line| !line.unwrap().is_empty()).fold(
                BTreeMap::new(),
                |mut acc, line| {
                    if let [dest, src, length, ..] = line
                        .unwrap()
                        .split(' ')
                        .filter_map(|num| num.parse().ok())
                        .collect::<Vec<i32>>()[..]
                    {
                        acc.insert(src, (dest, length));
                    }
                    acc
                },
            );
            maps.push(map);
        }
        Self {
            seeds,
            maps: vec![],
        }
    }
}

fn main() {
    let input = Input::from(stdin().lines());
}
