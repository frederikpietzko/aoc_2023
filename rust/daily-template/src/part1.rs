use anyhow::Result;

pub fn process(_input: &str) -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> Result<()> {
        todo!("Write some tests!");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
