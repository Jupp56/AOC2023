use std::{cmp::Ordering, collections::HashMap};

use crate::{get_hands, sum_hands, Type};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    #[default]
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Unknown card"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut map: HashMap<Card, usize> = HashMap::new();
        for card in self.cards {
            match map.get_mut(&card) {
                Some(count) => *count += 1,
                None => {
                    map.insert(card, 1);
                }
            }
        }

        let pairs: Vec<(Card, usize)> = map.drain().collect();
        match pairs.len() {
            1 => Type::FiveOfKind,
            2 => {
                // One of the two card types has four cards
                if pairs[0].1 == 4 || pairs[1].1 == 4 {
                    Type::FourOfKind
                } else {
                    Type::FullHouse
                }
            }
            3 => {
                // Any card type has three
                if pairs.iter().any(|(_, count)| *count == 3) {
                    Type::ThreeOfKind
                } else {
                    Type::TwoPair
                }
            }
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => unreachable!(),
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_ordering = self.get_type().cmp(&other.get_type());
        match type_ordering {
            Ordering::Equal => self.cards.cmp(&other.cards),
            _ => type_ordering,
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        let cards: [Card; 5] = [
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
        ];

        Self { cards }
    }
}

pub fn part_1(input: &str) {
    let mut hands: Vec<(Hand, usize)> = get_hands(input);
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    let sum = sum_hands(hands);

    println!("Result 1: {sum}");
}
