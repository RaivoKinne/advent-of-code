use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter(|character| character.is_digit(10));
            let first = it.next().expect("should be a number");
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet";

        assert_eq!("142", process(input)?);
        Ok(())
    }
}
