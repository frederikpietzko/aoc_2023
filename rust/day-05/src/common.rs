use std::ops::Deref;

use anyhow::Result;

pub struct Entry {
    source: usize,
    destination: usize,
    range: usize,
}

impl Entry {
    fn new(source: usize, destination: usize, range: usize) -> Entry {
        Entry {
            source,
            destination,
            range,
        }
    }

    pub fn includes(&self, num: usize) -> bool {
        self.source <= num && num <= self.source + self.range
    }

    pub fn transform(&self, num: usize) -> usize {
        if self.includes(num) {
            self.destination + (num - self.source)
        } else {
            num
        }
    }
}

impl TryFrom<&str> for Entry {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut nums = line.split_whitespace().map(|num| num.parse::<usize>());
        let destination = nums.next().unwrap()?; //?
        let source = nums.next().unwrap()?;
        let range = nums.next().unwrap()?;
        Ok(Entry::new(source, destination, range))
    }
}

pub struct Map {
    entries: Vec<Entry>,
}

impl Map {
    pub fn transform(&self, seed: usize) -> usize {
        if let Some(entry) = self.entries.iter().find(|entry| entry.includes(seed)) {
            entry.transform(seed)
        } else {
            seed
        }
    }
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let entries = input
            .trim()
            .split('\n')
            .skip(1)
            .map(|line| line.try_into())
            .collect::<Result<Vec<_>>>()?;
        Ok(Map { entries })
    }
}

pub struct Seeds(pub Vec<usize>);

impl Deref for Seeds {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() -> Result<()> {
        let entry: Entry = "52 50 49".try_into()?;
        assert_eq!(entry.source, 50);
        assert_eq!(entry.destination, 52);
        assert_eq!(entry.range, 49);
        Ok(())
    }

    #[test]
    fn test_parse_map() -> Result<()> {
        let input = r#"seed-to-soil map:
52 50 49
50 52 49"#;
        let map: Map = input.try_into()?;
        assert_eq!(map.entries.len(), 2);
        Ok(())
    }
}
