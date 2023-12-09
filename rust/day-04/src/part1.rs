use std::{collections::HashSet, num::ParseIntError};

use anyhow::Result;
use itertools::Itertools;

pub fn process(input: &str) -> Result<String> {
    let result: u32 = input
        .trim()
        .split('\n')
        .map(|line| line.trim().split(':').collect::<Vec<&str>>()[1])
        .map(|line| {
            let halves = line.trim().split('|').collect::<Vec<&str>>();
            (halves[0], halves[1])
        })
        .map(|(winning, got)| -> Result<(HashSet<u32>, HashSet<u32>)> {
            let winning = winning
                .split_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<HashSet<_>, _>>()?;
            let got = got
                .split_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<HashSet<_>, _>>()?;
            Ok((winning, got))
        })
        .map_ok(|(winning, got)| {
            let intersecting = got.intersection(&winning).count() as u32;
            if intersecting == 0 {
                return 0;
            }
            if intersecting == 1 {
                return 1;
            }
            let sum = 2u32.pow(intersecting - 1);
            sum
        })
        .sum::<Result<u32>>()?;

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
        assert_eq!("13", process(INPUT)?);
        Ok(())
    }
}
