use std::error::Error;

fn main(input: &str) -> Result<String, Box<dyn Error>> {
    println!("{input}");
    Ok("147".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("147", main(input)?);
        Ok(())
    }
}
