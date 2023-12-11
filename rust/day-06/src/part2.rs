use anyhow::{anyhow, Result};
use itertools::Itertools;

struct Race {
    time: usize,
    to_beat: usize,
}

pub fn process(input: &str) -> Result<String> {
    let mut lines = input.trim().split('\n');
    let time = lines
        .next()
        .ok_or(anyhow!("No input"))?
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<usize>()?;

    let to_beat = lines
        .next()
        .ok_or(anyhow!("No input"))?
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<usize>()?;

    let times = (0..time)
        .filter(|time_held| (time - time_held) * time_held > to_beat)
        .count();

    Ok(times.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("71503", process(INPUT)?);
        Ok(())
    }
}
