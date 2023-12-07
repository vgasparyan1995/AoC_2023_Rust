use std::{
    io::{self, BufRead},
    panic,
};

#[derive(Eq, PartialEq, PartialOrd)]
struct Card {
    rank: i32,
}

impl From<char> for Card {
    fn from(card: char) -> Self {
        let rank = match card {
            '2'..='9' => card as i32 - '0' as i32,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        };
        Card { rank }
    }
}

#[derive(Eq, PartialEq, PartialOrd)]
enum HandStrength {
    High { card: Card },
    Pair { card: Card },
    TwoPair { high: Card, low: Card },
    Three { card: Card },
    FullHouse { three: Card, two: Card },
    Four { card: Card },
    Five { card: Card },
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    strength: HandStrength,
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        Hand {
            cards: hand.chars().map(Card::from).collect(),
            strength: HandStrength::High {
                // TODO!
                card: Card::from('A'),
            },
        }
    }
}

struct Input {
    hands_to_bid: Vec<(Hand, i64)>,
}

impl FromIterator<&str> for Input {
    fn from_iter<T: IntoIterator<Item = &str>>(iter: T) -> Self {
        Input {
            hands_to_bid: iter
                .into_iter()
                .map(|line| line.split_once(' ').unwrap())
                .map(|(hand, bid)| (Hand::from(hand), bid.parse().ok().unwrap()))
                .collect(),
        }
    }
}

fn main() {
    let mut input = Input::from(
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().as_str()),
    );
}
