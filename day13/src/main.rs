use std::{
    io::{stdin, Lines, StdinLock},
    iter::{Rev, Take},
    ops::Range,
    panic, println,
};

type Row = Vec<char>;
type Map = Vec<Row>;

struct Input {
    maps: Vec<Map>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let maps = lines
            .into_iter()
            .map(|line| line.unwrap().chars().collect::<Row>())
            .collect::<Vec<Row>>()
            .split(|row| row.is_empty())
            .map(|rows| rows.into_iter().map(|row| row.clone()).collect::<Map>())
            .collect::<Vec<Map>>();
        Self { maps }
    }
}

fn cols_equal(
    row: &Row,
    left_range: Take<Rev<Range<usize>>>,
    right_range: Take<Range<usize>>,
) -> bool {
    left_range
        .zip(right_range)
        .all(|(left, right)| row[left] == row[right])
}

fn rows_equal(
    map: &Map,
    top_range: Take<Rev<Range<usize>>>,
    bottom_range: Take<Range<usize>>,
) -> bool {
    top_range
        .zip(bottom_range)
        .all(|(top, bottom)| map[top] == map[bottom])
}

fn find_vertical_symmetry(map: &Map) -> Option<usize> {
    let col_max = map[0].len();
    (1..col_max)
        .filter(|&col| {
            let right_range = col..col_max;
            let left_range = (0..col).rev();
            let common_length = right_range.len().min(left_range.len());
            let right_range = right_range.take(common_length);
            let left_range = left_range.take(common_length);
            map.iter()
                .all(|row| cols_equal(row, left_range.clone(), right_range.clone()))
        })
        .next()
}

fn find_horizontal_symmetry(map: &Map) -> Option<usize> {
    let row_max = map.len();
    (1..row_max)
        .filter(|&row| {
            let bottom_range = row..row_max;
            let top_range = (0..row).rev();
            let common_length = bottom_range.len().min(top_range.len());
            let bottom_range = bottom_range.take(common_length);
            let top_range = top_range.take(common_length);
            rows_equal(map, top_range, bottom_range)
        })
        .next()
}

fn solve(map: Map) -> usize {
    if let Some(col) = find_vertical_symmetry(&map) {
        return col;
    }
    if let Some(row) = find_horizontal_symmetry(&map) {
        return row * 100;
    }
    panic!()
}

fn part1(input: Input) -> usize {
    input.maps.into_iter().map(solve).sum()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part1(input));
}
