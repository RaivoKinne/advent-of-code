use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut output = 0;

    for game in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let pulls = game.split(": ").collect::<Vec<&str>>();

        for pull in pulls[1].split("; ") {
            let cubes = pull.split(", ").collect::<Vec<&str>>();

            for cube in cubes {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();

                match color {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    _ => panic!("Unknown cube color: {}", color),
                }
            }
        }

        output += min_red * min_green * min_blue;
    }

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
