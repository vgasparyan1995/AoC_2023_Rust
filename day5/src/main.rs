use std::ops::{Bound::Excluded, Bound::Included, Bound::Unbounded};
use std::{
    collections::BTreeMap,
    io::stdin,
    io::{Lines, StdinLock},
};

struct Input {
    seeds: Vec<i64>,
    maps: Vec<BTreeMap<i64, i64>>,
}

impl From<Lines<StdinLock<'static>>> for Input {
    fn from(lines: Lines<StdinLock<'static>>) -> Self {
        let mut lines = lines.into_iter();
        let seeds = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();
        lines.next();
        let mut maps: Vec<BTreeMap<i64, i64>> = Vec::new();
        let mut map = BTreeMap::from([(0, 0)]);
        for line in lines {
            match line.unwrap() {
                line if line.ends_with("map:") => {
                    map.clear();
                    map.insert(0, 0);
                }
                line if line.is_empty() => {
                    if !map.is_empty() {
                        maps.push(map);
                        map = BTreeMap::from([(0, 0)]);
                    }
                }
                line => {
                    let mapping_numbers: Vec<i64> =
                        line.split(' ').filter_map(|num| num.parse().ok()).collect();
                    if let [dst, src, length, ..] = mapping_numbers[..] {
                        let diff = dst - src;
                        map.insert(src, diff);
                        let end = src + length;
                        if !map.contains_key(&end) {
                            map.insert(end, 0);
                        }
                    }
                }
            }
        }
        if !map.is_empty() {
            maps.push(map);
        }
        Self { seeds, maps }
    }
}

fn do_map(map: &BTreeMap<i64, i64>, key: i64) -> i64 {
    let diff = map.range((Unbounded, Included(&key))).last().unwrap().1;
    key + diff
}

fn do_map_range(map: &BTreeMap<i64, i64>, key_ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    println!("map: {map:?}, key_ranges: {key_ranges:?}");
    let mut result = Vec::new();
    for (key_begin, key_end) in key_ranges {
        let mut src_begin = key_begin;
        let mut curr_diff = do_map(map, key_begin) - key_begin;
        for (&src_end, &diff) in map.range((Included(&key_begin), Included(&key_end))) {
            if src_end == src_begin {
                continue;
            }
            result.push((src_begin + curr_diff, src_end + curr_diff));
            src_begin = src_end;
            curr_diff = diff;
        }
        if src_begin != key_end {
            result.push((src_begin + curr_diff, key_end + curr_diff));
        }
    }
    println!("Result: {result:?}");
    result
}

fn part1(input: Input) -> i64 {
    input
        .seeds
        .into_iter()
        .map(|seed| {
            input
                .maps
                .clone()
                .into_iter()
                .fold(seed, |src, map| do_map(&map, src))
        })
        .min()
        .unwrap()
}

fn part2(input: Input) -> i64 {
    let seed_ranges =
        (0..(input.seeds.len() / 2)).map(|i| (input.seeds[2 * i], input.seeds[2 * i + 1]));
    seed_ranges
        .map(|(begin, length)| {
            input
                .maps
                .clone()
                .into_iter()
                .fold(vec![(begin, begin + length - 1)], |ranges, map| {
                    do_map_range(&map, ranges)
                })
                .into_iter()
        })
        .flatten()
        .map(|(range_start, _)| range_start)
        .min().unwrap()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
