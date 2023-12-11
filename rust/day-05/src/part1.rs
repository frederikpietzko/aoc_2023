use super::common::*;

use anyhow::{anyhow, Result};

pub fn process(input: &str) -> Result<String> {
    let (seeds, maps) = parse(input.trim())?;
    let mut locations = Vec::new();
    for seed in seeds.iter() {
        let mut seed = *seed;
        for map in maps.iter() {
            seed = map.transform(seed);
        }
        locations.push(seed);
    }
    locations.sort();
    let lowest = locations[0];
    Ok(format!("{}", lowest).to_string())
}

fn parse(input: &str) -> Result<(Seeds, Vec<Map>)> {
    let mut lines = input.trim().split("\n\n");
    let seeds: Seeds = lines.next().ok_or(anyhow!("no next line"))?.try_into()?;
    let maps: Vec<Map> = lines
        .map(|line| line.try_into())
        .collect::<Result<Vec<_>>>()?;
    Ok((seeds, maps))
}

impl TryFrom<&str> for Seeds {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let seeds = input
            .split(':')
            .nth(1)
            .ok_or(anyhow!(""))?
            .split_whitespace()
            .map(|num| {
                num.parse::<usize>()
                    .map_err(|_e| anyhow!("Could not parse number"))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Seeds(seeds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("35", process(INPUT)?);
        Ok(())
    }

    #[test]
    fn test_parse_seeds() -> Result<()> {
        let input = r#"seeds: 79 14 55 13"#;
        let seeds: Seeds = input.try_into()?;
        assert_eq!(seeds.len(), 4);
        assert_eq!(seeds[0], 79);
        assert_eq!(seeds[1], 14);
        assert_eq!(seeds[2], 55);
        assert_eq!(seeds[3], 13);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let (seeds, maps) = parse(INPUT)?;
        assert_eq!(seeds.len(), 4);
        assert_eq!(maps.len(), 7);
        Ok(())
    }

    #[test]
    fn test_transform() -> Result<()> {
        let (seeds, maps) = parse(INPUT)?;
        let mut seed = seeds[0];
        for map in maps.iter() {
            seed = map.transform(seed);
        }
        assert_eq!(seed, 82);
        Ok(())
    }
}
