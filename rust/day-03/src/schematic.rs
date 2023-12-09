use itertools::Itertools;
use std::cmp::{max, min};

pub struct Schematic {
    schematic: Vec<char>,
    len: usize,
}

impl Schematic {
    pub fn parse(input: &str) -> Schematic {
        let lines: Vec<&str> = input.split('\n').collect();
        let line_len = lines[0].len();
        let schematic = input.chars().filter(|c| !c.is_whitespace()).collect();
        Schematic {
            schematic,
            len: line_len,
        }
    }

    pub fn symbol_positions(&self) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        for (i, c) in self.schematic.iter().enumerate() {
            if is_symbol(*c) {
                res.push(i);
            }
        }
        res
    }

    pub fn adjacent_numbers(&self, pos: usize) -> Vec<u32> {
        let positions = self.adjacent_positions(pos);
        positions
            .iter()
            .filter(|pos| self.schematic[**pos].is_numeric())
            .map(|pos| self.get_number(*pos))
            .dedup()
            .collect_vec()
    }

    pub fn gears(&self) -> Vec<u32> {
        self.symbol_positions()
            .iter()
            .filter(|pos| is_gear_symbol(self.schematic[**pos]))
            .filter_map(|pos| {
                let numbers = self.adjacent_numbers(*pos);
                if numbers.len() == 2 {
                    Some(numbers[0] * numbers[1])
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_number(&self, pos: usize) -> u32 {
        if !self.schematic[pos].is_numeric() {
            return 0;
        }
        let x = pos % self.len;
        let mut left_bound: usize = pos - x;
        let mut right_bound: usize = pos + (self.len - 1 - x);
        for i in 1..=x {
            if !self.schematic[pos - i].is_numeric() {
                left_bound = pos - i + 1;
                break;
            }
        }
        for i in 1..(self.len - x) {
            if !self.schematic[pos + i].is_numeric() {
                right_bound = pos + i - 1;
                break;
            }
        }
        let number: String = self.schematic[left_bound..=right_bound].iter().collect();
        number.parse::<u32>().unwrap()
    }

    fn adjacent_positions(&self, pos: usize) -> Vec<usize> {
        let x = (pos % self.len) as i32;
        let y = (pos / self.len) as i32;
        let len = self.len as i32;

        let possible_positions = [
            (y - 1) * len + max(x - 1, 0),
            (y - 1) * len + x,
            (y - 1) * len + min(x + 1, len - 1),
            y * len + max(x - 1, 0),
            y * len + min(x + 1, len - 1),
            (y + 1) * len + max(x - 1, 0),
            (y + 1) * len + x,
            (y + 1) * len + min(x + 1, len - 1),
        ];
        possible_positions
            .iter()
            .filter(|my_pos| {
                **my_pos > 0 && **my_pos < self.schematic.len() as i32 && **my_pos != pos as i32
            })
            .dedup()
            .copied()
            .map(|pos| pos as usize)
            .collect_vec()
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn is_gear_symbol(c: char) -> bool {
    c == '*'
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_schematic() -> Result<()> {
        let schematic = Schematic::parse(INPUT);
        assert_eq!(schematic.len, 10);
        assert_eq!(schematic.schematic.len(), 100);
        Ok(())
    }

    #[test]
    fn test_symbol_positions() -> Result<()> {
        let schematic = Schematic::parse(INPUT);
        let positions = schematic.symbol_positions();
        assert_eq!(positions.len(), 6);
        assert_eq!(positions[0], 13);
        assert_eq!(positions[1], 36);
        assert_eq!(positions[2], 43);
        Ok(())
    }

    #[test]
    fn test_adjecent_postions() {
        let schematic = Schematic::parse(INPUT);
        let mut positions = schematic.adjacent_positions(0);
        assert_eq!(positions, vec![1, 10, 11]);
        positions = schematic.adjacent_positions(9);
        assert_eq!(positions, vec![8, 18, 19]);
        positions = schematic.adjacent_positions(13);
        assert_eq!(positions, vec![2, 3, 4, 12, 14, 22, 23, 24]);
        positions = schematic.adjacent_positions(90);
        assert_eq!(positions, vec![80, 81, 91]);
        positions = schematic.adjacent_positions(99);
        assert_eq!(positions, vec![88, 89, 98]);
    }

    #[test]
    fn test_get_number() -> Result<()> {
        let schematic = Schematic::parse(INPUT);
        assert_eq!(schematic.get_number(27), 633);
        assert_eq!(schematic.get_number(0), 467);
        assert_eq!(schematic.get_number(1), 467);
        assert_eq!(schematic.get_number(2), 467);
        assert_eq!(schematic.get_number(3), 0);
        Ok(())
    }

    #[test]
    fn test_adjacent_numbers() -> Result<()> {
        let schematic = Schematic::parse(INPUT);
        let numbers = schematic.adjacent_numbers(13);
        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0], 467);
        assert_eq!(numbers[1], 35);

        Ok(())
    }
}
