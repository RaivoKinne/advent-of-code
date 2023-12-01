use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
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
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
