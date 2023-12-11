use anyhow::Result;
use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,  // 11
    Queen = 12, // 12
    King = 13,  // 13
    Ace = 14,
}

impl Card {
    fn parse(input: char) -> Card {
        match input {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", input),
        }
    }

    fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    OAK5 = 7,
    OAK4 = 6,
    FH = 5,
    OAK3 = 4,
    TP = 3,
    OP = 2,
    HC = 1,
}

impl HandType {
    fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bet: usize,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        let mut lines = input.split_whitespace();
        let cards = lines.next().expect("Invalid hand");
        let cards = cards.chars().map(Card::parse).collect::<Vec<_>>();
        let bet = lines
            .next()
            .expect("Invalid hand")
            .parse::<usize>()
            .expect("Unable to parse bet");

        Hand {
            cards: cards.try_into().expect("Invalid hand"),
            bet,
        }
    }

    fn hand_type(&self) -> HandType {
        let mut map: HashMap<Card, usize> = HashMap::new();
        for card in self.cards {
            map.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }

        match map.len() {
            1 => HandType::OAK5,
            2 => {
                let counts: (usize, usize) = map
                    .values()
                    .copied()
                    .sorted()
                    .collect_tuple()
                    .expect("Invalid hand");
                match counts {
                    (2, 3) => HandType::FH,
                    (1, 4) => HandType::OAK4,
                    _ => panic!("Invalid hand: {:?}", self.cards),
                }
            }
            3 => {
                let counts: (usize, usize, usize) = map
                    .values()
                    .copied()
                    .sorted()
                    .collect_tuple()
                    .expect("Invalid hand");
                match counts {
                    (1, 1, 3) => HandType::OAK3,
                    (1, 2, 2) => HandType::TP,
                    _ => panic!("Invalid hand: {:?}", self.cards),
                }
            }
            4 => HandType::OP,
            5 => HandType::HC,
            _ => panic!("Invalid hand: {:?}", self.cards),
        }
    }

    fn bet(&self) -> usize {
        self.bet
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().value().cmp(&other.hand_type().value()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (mc, oc) in self.cards.iter().zip(other.cards.iter()) {
                    match mc.value().cmp(&oc.value()) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => continue,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

pub fn process(input: &str) -> Result<String> {
    let mut sum = 0;
    for (rank, card) in input
        .trim()
        .split('\n')
        .map(Hand::parse)
        .sorted()
        .enumerate()
    {
        sum += (rank + 1) * card.bet();
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("6440", process(INPUT)?);
        Ok(())
    }
}
