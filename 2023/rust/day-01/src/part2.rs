use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let output = input.lines().map(process_line).sum::<u32>();
    Ok(output.to_string())
}

fn process_line(line: &str) -> u32 {
    let mut numbers = Vec::new();

    for index in 0..line.len() {
        let remaining_line = &line[index..];
        let digit = match remaining_line {
            part if part.starts_with("one") => '1',
            part if part.starts_with("two") => '2',
            part if part.starts_with("three") => '3',
            part if part.starts_with("four") => '4',
            part if part.starts_with("five") => '5',
            part if part.starts_with("six") => '6',
            part if part.starts_with("seven") => '7',
            part if part.starts_with("eight") => '8',
            part if part.starts_with("nine") => '9',
            _ => remaining_line.chars().next().unwrap(),
        };

        if let Some(num) = digit.to_digit(10) {
            numbers.push(num);
        }
    }

    let first = numbers.first().expect("should be a number");

    match numbers.last() {
        Some(num) => format!("{}{}", first, num),
        None => format!("{}{}", first, first),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen";

        assert_eq!("281", process(input)?);
        Ok(())
    }
}
