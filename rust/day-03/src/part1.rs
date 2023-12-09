use crate::schematic::Schematic;

use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let schematic = Schematic::parse(input);
    let positions = schematic.symbol_positions();
    let sum: u32 = positions
        .iter()
        .flat_map(|pos| schematic.adjacent_numbers(*pos))
        .sum();

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
        let res = process(INPUT)?;
        assert_eq!(res, "4361");
        Ok(())
    }
}
