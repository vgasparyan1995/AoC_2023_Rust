use std::{
    collections::HashMap,
    io::{self, BufRead},
    panic, println,
};

const J: char = 'J';

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Card {
    rank: i32,
}

impl From<char> for Card {
    fn from(card: char) -> Self {
        let rank = match card {
            '2'..='9' => card as i32 - '0' as i32,
            'T' => 10,
            // 'J' => 11, // PART1
            'J' => 1, // PART2
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        };
        Card { rank }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
#[allow(dead_code)]
enum HandStrength {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl From<&str> for HandStrength {
    fn from(hand: &str) -> Self {
        let mut sorted_hand: Vec<(char, i32)> = hand
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                acc.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
                acc
            })
            .into_iter()
            .collect();
        sorted_hand.sort_by(|(lcard, lnum), (rcard, rnum)| rnum.cmp(lnum).then(rcard.cmp(lcard)));
        match sorted_hand[..] {
            [(_, 5)] => Self::Five,

            [(J, 4), (_, 1)] => Self::Five, // PART2
            [(_, 4), (J, 1)] => Self::Five, // PART2
            [(_, 4), (_, 1)] => Self::Four,

            [(J, 3), (_, 2)] => Self::Five, // PART2
            [(_, 3), (J, 2)] => Self::Five, // PART2
            [(_, 3), (_, 2)] => Self::FullHouse,

            [(J, 3), (_, 1), (_, 1)] => Self::Four, // PART2
            [(_, 3), (J, 1), (_, 1)] => Self::Four, // PART2
            [(_, 3), (_, 1), (J, 1)] => Self::Four, // PART2
            [(_, 3), (_, 1), (_, 1)] => Self::Three,

            [(J, 2), (_, 2), (_, 1)] => Self::Four, // PART2
            [(_, 2), (J, 2), (_, 1)] => Self::Four, // PART2
            [(_, 2), (_, 2), (J, 1)] => Self::FullHouse, // PART2
            [(_, 2), (_, 2), (_, 1)] => Self::TwoPair,

            [(J, 2), (_, 1), (_, 1), (_, 1)] => Self::Three, // PART2
            [(_, 2), (J, 1), (_, 1), (_, 1)] => Self::Three, // PART2
            [(_, 2), (_, 1), (J, 1), (_, 1)] => Self::Three, // PART2
            [(_, 2), (_, 1), (_, 1), (J, 1)] => Self::Three, // PART2
            [(_, 2), (_, 1), (_, 1), (_, 1)] => Self::Pair,

            [(J, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Self::Pair, // PART2
            [(_, 1), (J, 1), (_, 1), (_, 1), (_, 1)] => Self::Pair, // PART2
            [(_, 1), (_, 1), (J, 1), (_, 1), (_, 1)] => Self::Pair, // PART2
            [(_, 1), (_, 1), (_, 1), (J, 1), (_, 1)] => Self::Pair, // PART2
            [(_, 1), (_, 1), (_, 1), (_, 1), (J, 1)] => Self::Pair, // PART2
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Self::High,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Hand {
    strength: HandStrength,
    cards: Vec<Card>,
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        Hand {
            cards: hand.chars().map(Card::from).collect(),
            strength: HandStrength::from(hand),
        }
    }
}

struct Input {
    hands_to_bid: Vec<(Hand, i64)>,
}

impl FromIterator<String> for Input {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Input {
            hands_to_bid: iter
                .into_iter()
                .filter_map(|line| {
                    line.split_once(' ')
                        .map(|(hand, bid)| (Hand::from(hand), bid.parse().ok().unwrap()))
                })
                .collect(),
        }
    }
}

fn part1and2(mut input: Input) -> i64 {
    input.hands_to_bid.sort_by(|(lhs, _), (rhs, _)| {
        lhs.strength
            .cmp(&rhs.strength)
            .then(lhs.cards.cmp(&rhs.cards))
    });
    input
        .hands_to_bid
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as i64 + 1) * bid)
        .sum()
}

fn main() {
    let input = Input::from_iter(io::stdin().lock().lines().map(|line| line.unwrap()));
    println!("{}", part1and2(input));
}
