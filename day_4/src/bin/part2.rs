use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let input = include_str!("./input_day_4.txt");
    let output = part2(input);
    // println!("{:?}", output);
}
#[derive(Debug, Clone)]
struct Game {
    id: usize,
    winning_numbers: Vec<usize>,
    playing_numbers: Vec<usize>,
    extra_times_to_run: usize,
}

impl Game {
    fn get_score(&self) -> usize {
        let hashed_w: HashSet<usize> = self.winning_numbers.iter().cloned().collect();
        let mut count: usize = 0;

        for num in hashed_w {
            if self.playing_numbers.contains(&num) {
                count += 1;
            }
        }
        count
    }
}
fn extract_number(input: &str) -> Result<usize, &'static str> {
    let parts = input.splitn(2, ':').collect::<Vec<&str>>();
    if parts.len() < 2 {
        return Err("Invalid input: ':' not found");
    }

    let left_part = parts[0].trim();
    let number_part = left_part
        .split_whitespace()
        .last()
        .ok_or("No number found")?;

    number_part
        .parse::<usize>()
        .map_err(|_| "Failed to parse number")
}
// Card   3: 40 25 13 65 86  5 35 87  9 30 | 87 76 30 93  5 45 16 40 48 89 78 59 18 12 71 85 66 21 80 28 50 60 49 72 27
fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        // Extract the winning_numbers string
        let game_number = extract_number(&line);
        // println!("{:?}", game_number);

        let game = line.split(": ").nth(1).ok_or("Game data not found")?;
        let winning_numbers = game.split('|').nth(0).ok_or("Winning numbers not found")?;

        // Extract the playing_numbers string and process it
        let playing_numbers = line.split('|').nth(1).ok_or("Playing numbers not found")?;

        // Split playing_numbers into Vec<&str>, parse each &str into usize, and collect into Vec<usize>
        let parsed_playing_numbers: Result<Vec<usize>, _> = playing_numbers
            .split_whitespace()
            .map(|num_str| num_str.parse::<usize>())
            .collect();

        // Use parsed_numbers as needed
        // println!("{:?}", parsed_playing_numbers);
        let parsed_wining_numbers: Result<Vec<usize>, _> = winning_numbers
            .split_whitespace()
            .map(|num_str| num_str.parse::<usize>())
            .collect();

        // Use parsed_numbers as needed
        // println!("{:?}", parsed_wining_numbers);
        games.push(Game {
            id: game_number.unwrap(),
            playing_numbers: parsed_playing_numbers.unwrap(),
            winning_numbers: parsed_wining_numbers.unwrap(),
            extra_times_to_run: 0,
        });
    }
    // for game in games {
    //     println!("{}", game.get_score());
    // }

    // for each game we want to know how many wins and the id
    // then if id is 1 and won 4 times
    // we want to run 2,3,4,5 once
    // make hashmap of all ids
    // with key value pair of id: 1, time_to_run:1

    let mut game_map: HashMap<usize, usize> = HashMap::new();
    for game in &games {
        game_map.insert(game.id, 1);
    }
    for game in &games {
        let t = game.clone();
        let mut input = 1;
        println!("game before {:?}", game_map[&t.id]);
        println!("game before {:?}", t.id);

        for tf in 1..=game_map[&t.id] {
            println!("game id inside loop {:?}", t.id);

            println!("id value value {:?}", game_map[&game.id]);
            input += 1;
            println!("map before {:?}", tf);
            increment_game_map(&mut game_map, game.id, game.get_score());
            // println!(" map after {:?}", game_map);
        }
    }

    let total_sum: usize = game_map.iter().map(|(_key, &value)| value).sum();

    println!("Total sum of values: {}", total_sum); // let sum: usize = games.iter().map(|game| game.get_score()).sum();
                                                    // println!("{}", sum);
    Ok("".to_string())
}
fn increment_game_map(
    game_map: &mut HashMap<usize, usize>,
    current_id: usize,
    increment_fields: usize,
) {
    // Starting from current_id + 1, increment the next 'increment_fields' keys
    for id in current_id + 1..=current_id + increment_fields {
        // If the key exists, increment its value; otherwise, insert it with a value of 1
        *game_map.entry(id).or_insert(0) += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn it_works() {
        let result = part1(INPUT);
        // assert!(result.is_ok());
        assert_eq!(result.unwrap(), "30".to_string());
    }
}
