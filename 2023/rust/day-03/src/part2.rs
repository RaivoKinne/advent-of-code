use anyhow::Result;

pub fn process(_input: &str) -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
