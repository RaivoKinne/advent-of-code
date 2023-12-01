use anyhow::Result;

pub fn process(_input: &str) -> Result<String> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
