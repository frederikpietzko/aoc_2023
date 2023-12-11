use anyhow::{anyhow, Result};

struct Race {
    time: usize,
    to_beat: usize,
}

pub fn process(input: &str) -> Result<String> {
    let mut lines = input.trim().split('\n');
    let times = lines
        .next()
        .ok_or(anyhow!("No input"))?
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<usize>())
        .filter_map(Result::ok);

    let times = lines
        .next()
        .ok_or(anyhow!("No input"))?
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<usize>())
        .filter_map(Result::ok)
        .zip(times)
        .map(|(to_beat, time)| Race { time, to_beat })
        .collect::<Vec<_>>();

    let times = times
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|time_held| (race.time - time_held) * time_held > race.to_beat)
                .count()
        })
        .reduce(|acc, x| acc * x)
        .ok_or(anyhow!("reduce fucked up???"))?;

    Ok(times.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("288", process(INPUT)?);
        Ok(())
    }
}
