use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let output = input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let sets = line.split(": ").collect::<Vec<&str>>();
            let cubes = sets[1]
                .split("; ")
                .map(|cube| {
                    let pulls = cube.split(", ").collect::<Vec<&str>>();
                    if is_possible_game(&pulls) {
                        Some(index + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<usize>>>();

            if cubes.iter().all(|pull| pull.is_some()) {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    Ok(output.to_string())
}

fn is_possible_game(pulls: &[&str]) -> bool {
    let mut cube_counts = [0; 3];

    for cube in pulls {
        let (count, color) = cube.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();

        match color {
            "red" => cube_counts[0] += count,
            "green" => cube_counts[1] += count,
            "blue" => cube_counts[2] += count,
            _ => panic!("Unknown cube color: {}", color),
        }
    }
    cube_counts[0] <= 12 && cube_counts[1] <= 13 && cube_counts[2] <= 14
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

        assert_eq!("8", process(input)?);
        Ok(())
    }
}
