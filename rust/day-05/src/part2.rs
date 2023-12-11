use super::common::*;
use anyhow::{anyhow, Result};
use itertools::Itertools;

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
    let seeds: Seeds = lines.next().map(parse_seeds).ok_or(anyhow!("no line"))?;
    let maps: Vec<Map> = lines
        .map(|line| line.try_into())
        .collect::<Result<Vec<_>>>()?;
    Ok((seeds, maps))
}

fn parse_seeds(line: &str) -> Seeds {
    let mut seeds = Vec::new();
    line.split_whitespace()
        .filter_map(|it| match it.parse::<usize>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .batching(|it| match it.next() {
            None => None,
            Some(x) => it.next().map(|y| (x, y)),
        })
        .for_each(|(x, y)| {
            for i in x..(x + y) {
                seeds.push(i);
            }
        });
    Seeds(seeds)
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
        assert_eq!("46", process(INPUT)?);
        Ok(())
    }
}
