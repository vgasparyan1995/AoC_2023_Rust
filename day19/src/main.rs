use std::{
    collections::HashMap,
    io::{stdin, Lines, StdinLock},
    panic, println,
};

const REJECTED: &str = "R";
const APPROVED: &str = "A";

#[derive(Clone, Copy, Default, Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Copy)]
enum Range {
    X(i64, i64),
    M(i64, i64),
    A(i64, i64),
    S(i64, i64),
}

struct Condition(Option<Range>, String);

struct Workflow {
    conditions: Vec<Condition>,
}

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let mut line_iter = lines.into_iter();
        let workflows = line_iter
            .by_ref()
            .take_while(|line| !line.as_ref().unwrap().is_empty())
            .map(|workflow_line| {
                let workflow_line = workflow_line.unwrap().to_owned();
                let (id, workflow_str) = workflow_line.split_once("{").unwrap();
                let conditions = workflow_str
                    .split(",")
                    .map(|condition_str| {
                        if let Some(final_destination) = condition_str.strip_suffix("}") {
                            Condition(None, final_destination.to_string())
                        } else {
                            let (condition, destination) = condition_str.split_once(":").unwrap();
                            let prop = condition.chars().nth(0).unwrap();
                            let cmp = condition.chars().nth(1).unwrap();
                            let target = condition[2..].parse::<i64>().unwrap();
                            let min = if cmp == '<' { 1 } else { target + 1 };
                            let max = if cmp == '<' { target - 1 } else { 4000 };
                            let range = match prop {
                                'x' => Range::X(min, max),
                                'm' => Range::M(min, max),
                                'a' => Range::A(min, max),
                                's' => Range::S(min, max),
                                _ => panic!(),
                            };
                            Condition(Some(range), destination.to_string())
                        }
                    })
                    .collect::<Vec<_>>();
                (id.to_string(), Workflow { conditions })
            })
            .collect();
        let parts = line_iter
            .map(|part_line| {
                part_line
                    .unwrap()
                    .strip_prefix("{")
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .split(",")
                    .fold(Part::default(), |mut part, prop| {
                        // todo, populate fields
                        if let Some((key, value)) = prop.split_once("=") {
                            match key {
                                "x" => part.x = value.parse().unwrap(),
                                "m" => part.m = value.parse().unwrap(),
                                "a" => part.a = value.parse().unwrap(),
                                "s" => part.s = value.parse().unwrap(),
                                _ => panic!(),
                            }
                        }
                        part
                    })
            })
            .collect();
        Input { workflows, parts }
    }
}

fn check(part: Part, workflows: &HashMap<String, Workflow>, id: String) -> String {
    match workflows
        .get(&id)
        .unwrap()
        .conditions
        .iter()
        .find_map(|Condition(range, dest)| {
            if range.is_none() {
                return Some(dest.clone());
            }
            if match range.unwrap() {
                Range::X(min, max) => part.x >= min && part.x <= max,
                Range::M(min, max) => part.m >= min && part.m <= max,
                Range::A(min, max) => part.a >= min && part.a <= max,
                Range::S(min, max) => part.s >= min && part.s <= max,
            } {
                return Some(dest.clone());
            }
            None
        })
        .unwrap()
        .as_str()
    {
        APPROVED => APPROVED.to_string(),
        REJECTED => REJECTED.to_string(),
        next => check(part, workflows, next.to_string()),
    }
}

#[derive(Debug)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn rating(&self) -> i64 {
        let x_len = self.x.1 - self.x.0 + 1;
        let m_len = self.m.1 - self.m.0 + 1;
        let a_len = self.a.1 - self.a.0 + 1;
        let s_len = self.s.1 - self.s.0 + 1;
        x_len * m_len * a_len * s_len
    }

    fn limit(&self, range: Option<Range>) -> PartRange {
        if range.is_none() {
            return PartRange { ..*self };
        }
        match range.unwrap() {
            Range::X(new_min, new_max) => PartRange {
                x: (self.x.0.max(new_min), self.x.1.min(new_max)),
                ..*self
            },
            Range::M(new_min, new_max) => PartRange {
                m: (self.m.0.max(new_min), self.m.1.min(new_max)),
                ..*self
            },
            Range::A(new_min, new_max) => PartRange {
                a: (self.a.0.max(new_min), self.a.1.min(new_max)),
                ..*self
            },
            Range::S(new_min, new_max) => PartRange {
                s: (self.s.0.max(new_min), self.s.1.min(new_max)),
                ..*self
            },
        }
    }
}

impl Default for PartRange {
    fn default() -> Self {
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

fn is_empty((min, max): (i64, i64)) -> bool {
    max < min
}

fn complement(part_range: &PartRange, if_range: Option<Range>) -> Option<Range> {
    if_range.map(|if_range| {
        let ((a, b), (a0, b0)) = match if_range {
            Range::X(a, b) => ((a, b), part_range.x),
            Range::M(a, b) => ((a, b), part_range.m),
            Range::A(a, b) => ((a, b), part_range.a),
            Range::S(a, b) => ((a, b), part_range.s),
        };
        let a = a.max(a0);
        let b = b.min(b0);
        let a1 = if a == a0 { b + 1 } else { a0 };
        let b1 = if a == a0 { b0 } else { a - 1 };
        match if_range {
            Range::X(_, _) => Range::X(a1, b1),
            Range::M(_, _) => Range::M(a1, b1),
            Range::A(_, _) => Range::A(a1, b1),
            Range::S(_, _) => Range::S(a1, b1),
        }
    })
}

fn traverse(graph: &HashMap<String, Workflow>, node: String, mut part_range: PartRange) -> i64 {
    let mut total_rating = 0;
    for Condition(if_range, neighbor) in &graph.get(&node).unwrap().conditions {
        let else_range = complement(&part_range, *if_range);
        let if_part_range = part_range.limit(*if_range);
        part_range = part_range.limit(else_range);
        if is_empty(if_part_range.x)
            || is_empty(if_part_range.m)
            || is_empty(if_part_range.a)
            || is_empty(if_part_range.s)
        {
            continue;
        }
        if neighbor == REJECTED {
            continue;
        }
        if neighbor == APPROVED {
            total_rating += if_part_range.rating();
            continue;
        }
        total_rating += traverse(graph, neighbor.clone(), if_part_range);
    }
    total_rating
}

fn part1(input: Input) -> i64 {
    input
        .parts
        .iter()
        .filter(|&part| check(*part, &input.workflows, "in".to_string()) == APPROVED)
        .map(Part::rating)
        .sum()
}

fn part2(input: Input) -> i64 {
    traverse(&input.workflows, "in".to_string(), PartRange::default())
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
