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
    J = 1,
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
    fn get_type_2(&self) -> Type {
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
                // One of the two card types all jokers?
                if pairs.iter().any(|(card, _)| *card == Card::J) {
                    Type::FiveOfKind
                }
                // Four of kind, no joker
                else if pairs.iter().any(|(_, count)| *count == 1) {
                    Type::FourOfKind
                } else {
                    // Full house
                    Type::FullHouse
                }
            }
            3 => {
                // are there three of a kind? (and two singular cards)
                if pairs.iter().any(|(_, count)| *count == 3) {
                    // A joker is there?
                    if pairs.iter().any(|(card, _)| *card == Card::J) {
                        // Directly four of a kind, better than full house. If the three are jokers, flip them to one of the other
                        Type::FourOfKind
                    } else {
                        Type::ThreeOfKind
                    }
                }
                // Two pairs, one singular card
                else {
                    // The single card a joker?
                    if pairs
                        .iter()
                        .any(|(card, count)| *card == Card::J && *count == 1)
                    {
                        // Flip it to one of the pairs, making a
                        Type::FullHouse
                    }
                    // Any of the two pairs consists of jokers?
                    else if pairs.iter().any(|(card, _)| *card == Card::J) {
                        // Convert both to the other card
                        Type::FourOfKind
                    } else {
                        Type::TwoPair
                    }
                }
            }
            4 => {
                // There be joker?
                if pairs.iter().any(|(card, _)| *card == Card::J) {
                    // If one joker, flip to the pair. If pair of jokers, flip to one of the cards.
                    Type::ThreeOfKind
                } else {
                    Type::OnePair
                }
            }

            5 => {
                // Any joker in there?
                if pairs.iter().any(|(card, _)| *card == Card::J) {
                    // Make any other card out of it to form a pair
                    Type::OnePair
                } else {
                    Type::HighCard
                }
            }
            _ => unreachable!(),
        }
    }

    fn cmp_2(&self, other: &Self) -> std::cmp::Ordering {
        let type_ordering = self.get_type_2().cmp(&other.get_type_2());
        match type_ordering {
            Ordering::Equal => (),
            _ => return type_ordering,
        };

        self.cards.cmp(&other.cards)
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

pub fn part_2(input: &str) {
    let mut hands: Vec<(Hand, usize)> = get_hands(input);
    hands.sort_by(|a, b| a.0.cmp_2(&b.0));

    let sum = sum_hands(hands);

    println!("Result 2: {sum}");
}
