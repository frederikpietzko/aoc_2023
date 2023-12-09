use anyhow::Result;

use crate::schematic::Schematic;

pub fn process(input: &str) -> Result<String> {
    let schematic = Schematic::parse(input);
    let sum = schematic.gears().iter().sum::<u32>();
    Ok(format!("{}", sum).to_string())
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
    #[test]
    fn test_process() -> Result<()> {
        assert_eq!("467835", process(INPUT)?);
        Ok(())
    }
}
