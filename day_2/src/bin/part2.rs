fn main() {
    let input = include_str!("./input_day_2.txt");
    let output = part2(input);
    println!("{}", output);
}
#[derive(Debug)]
struct Cube {
    color: String,
    amount: usize,
}
fn parse_line(mut line: String) -> usize {
    let game_loc = line.find(':').unwrap();
    let first_white_space = line.find(' ').unwrap();
    let game: String = line[first_white_space..game_loc].trim().to_string();
    let clean_line = line[game_loc + 1..].to_string();
    let sets: Vec<_> = clean_line.split(';').collect();
    let valid_game = check_line(sets);
    valid_game
}

fn check_line(rounds: Vec<&str>) -> usize {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let mut round_power: usize = 0;
    let cubes: Vec<Vec<Cube>> = rounds
        .iter() // Iterate over the vector of strings
        .map(|&input| {
            input
                .split(',') // Split each string by commas
                .filter_map(|part| {
                    let parts: Vec<&str> = part.trim().split_whitespace().collect();
                    if parts.len() == 2 {
                        let amount = parts[0].parse::<usize>().ok()?;
                        let color = parts[1].to_string();
                        Some(Cube { color, amount })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Cube>>() // Collect cubes from each string
        })
        .collect();
    for cube_vec in &cubes {
        for cube in cube_vec {
            match cube.color.as_str() {
                "red" => {
                    if cube.amount > max_red {
                        max_red = cube.amount
                    }
                }
                "green" => {
                    if cube.amount > max_green {
                        max_green = cube.amount
                    }
                }
                "blue" => {
                    if cube.amount > max_blue {
                        max_blue = cube.amount
                    }
                }
                &_ => todo!(),
            };
        }
    }

    round_power = max_red * max_green * max_blue;
    round_power
}
fn part2(input: &str) -> String {
    let mut results: Vec<usize> = vec![];

    for line in input.lines() {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            let x = parse_line(trimmed_line.to_string());
            results.push(x);
        }
    }
    let sum: usize = results.iter().sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn it_works() {
        let result = part2(INPUT);
        assert_eq!(result, "2286");
    }
}
