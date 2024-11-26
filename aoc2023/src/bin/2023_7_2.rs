use std::cmp::Ordering;

use advent_of_code::*;

const CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [u8; 5],
}

impl Hand {
    pub fn new(s: &str) -> Self {
        let cards: Vec<u8> = s
            .chars()
            .map(|c| {
                CARDS
                    .iter()
                    .enumerate()
                    .find_map(|(idx, card)| if *card == c { Some(idx as u8) } else { None })
                    .unwrap()
            })
            .collect();

        Hand {
            cards: cards.try_into().unwrap(),
        }
    }

    pub fn kind(&self) -> u8 {
        let counter: counter::Counter<u8> = self.cards.iter().copied().collect();
        let mut freq = counter.most_common_ordered();

        if let Some(&(_, joker)) = freq.iter().find(|(card, _)| *card == 0) {
            freq.retain(|(card, _)| *card != 0);
            if !freq.is_empty() {
                freq[0].1 += joker;
            }
        }

        match freq.len() {
            0 => {
                // This only happens with there a only Joker
                return 7;
            }
            1 => {
                return 7;
            }
            2 => {
                if freq[0].1 == 4 {
                    return 6;
                } else {
                    return 5;
                }
            }
            3 => {
                if freq[0].1 == 3 {
                    return 4;
                } else {
                    return 3;
                }
            }
            4 => {
                return 2;
            }
            5 => {
                return 1;
            }
            _ => {
                panic!()
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind() > other.kind() {
            return Ordering::Greater;
        } else if self.kind() == other.kind() {
            return self.cards.cmp(&other.cards);
        } else {
            return Ordering::Less;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = get_input(2023, 7);

    let mut hands: Vec<(Hand, u64)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(hand, bid)| (Hand::new(hand), bid.parse().unwrap()))
        .collect();

    hands.sort();

    //println!("{:?}", hands);

    let acc = hands
        .iter()
        .enumerate()
        .fold(0u64, |acc, (idx, (_, bid))| acc + (idx as u64 + 1) * bid);

    println!("{:?}", acc);
}
