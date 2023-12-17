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

fn cols_equal_error(
    row: &Row,
    left_range: Take<Rev<Range<usize>>>,
    right_range: Take<Range<usize>>,
) -> usize {
    left_range
        .zip(right_range)
        .filter(|(left, right)| row[*left] != row[*right])
        .count()
}

fn rows_equal_error(
    map: &Map,
    top_range: Take<Rev<Range<usize>>>,
    bottom_range: Take<Range<usize>>,
) -> usize {
    top_range
        .zip(bottom_range)
        .map(|(top, bottom)| {
            let top_row = &map[top];
            let bottom_row = &map[bottom];
            top_row
                .iter()
                .zip(bottom_row.iter())
                .filter(|(top, bottom)| top != bottom)
                .count()
        })
        .sum::<usize>()
}

fn find_vertical_symmetry(map: &Map, error: usize) -> Option<usize> {
    let col_max = map[0].len();
    (1..col_max)
        .filter(|&col| {
            let right_range = col..col_max;
            let left_range = (0..col).rev();
            let common_length = right_range.len().min(left_range.len());
            let right_range = right_range.take(common_length);
            let left_range = left_range.take(common_length);
            map.iter()
                .map(|row| cols_equal_error(row, left_range.clone(), right_range.clone()))
                .sum::<usize>()
                == error
        })
        .next()
}

fn find_horizontal_symmetry(map: &Map, error: usize) -> Option<usize> {
    let row_max = map.len();
    (1..row_max)
        .filter(|&row| {
            let bottom_range = row..row_max;
            let top_range = (0..row).rev();
            let common_length = bottom_range.len().min(top_range.len());
            let bottom_range = bottom_range.take(common_length);
            let top_range = top_range.take(common_length);
            rows_equal_error(map, top_range, bottom_range) == error
        })
        .next()
}

fn solve(map: Map, error: usize) -> usize {
    if let Some(col) = find_vertical_symmetry(&map, error) {
        return col;
    }
    if let Some(row) = find_horizontal_symmetry(&map, error) {
        return row * 100;
    }
    panic!()
}

fn part1(input: Input) -> usize {
    input.maps.into_iter().map(|map| solve(map, 0)).sum()
}

fn part2(input: Input) -> usize {
    input.maps.into_iter().map(|map| solve(map, 1)).sum()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
