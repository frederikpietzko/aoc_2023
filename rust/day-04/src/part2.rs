use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
struct Card {
    matches: usize,
    copies: u32,
}

impl Card {
    fn new(winning: HashSet<u32>, got: HashSet<u32>) -> Self {
        let matches = got.intersection(&winning).count();
        Self { matches, copies: 1 }
    }
}

pub fn process(input: &str) -> Result<String> {
    let mut cards: Vec<Card> = input
        .trim()
        .split('\n')
        .map(|line| line.trim().split(':').collect::<Vec<&str>>()[1])
        .map(|line| {
            let halves = line.trim().split('|').collect::<Vec<&str>>();
            (halves[0], halves[1])
        })
        .map(|(winning, got)| -> Result<Card> {
            let winning = winning
                .split_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<HashSet<_>, _>>()?;
            let got = got
                .split_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<HashSet<_>, _>>()?;
            Ok(Card::new(winning, got))
        })
        .collect::<Result<_>>()?;

    for i in 0..cards.len() {
        let card = cards[i];
        cards
            .iter_mut()
            .skip(i + 1)
            .take(card.matches)
            .for_each(|c| {
                c.copies += card.copies;
            });
    }

    let result = cards.iter().map(|card| card.copies).sum::<u32>();

    Ok(format!("{}", result).to_string())
}

#[cfg(test)]
mod tests {

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    use super::*;
    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("30", process(INPUT)?);
        Ok(())
    }
}
