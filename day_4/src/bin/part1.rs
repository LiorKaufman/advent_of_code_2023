use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let input = include_str!("./input_day_4.txt");
    let output = part1(input);
    // println!("{:?}", output);
}
#[derive(Debug, Clone)]
struct Game {
    winning_numbers: Vec<usize>,
    playing_numbers: Vec<usize>,
}

impl Game {
    fn get_score(&self) -> usize {
        let hashed_w: HashSet<usize> = self.winning_numbers.iter().cloned().collect();
        let mut count: usize = 0;
        let mut has_won_once = false;

        for num in hashed_w {
            if self.playing_numbers.contains(&num) {
                if !has_won_once {
                    has_won_once = true;
                    count += 1;
                } else {
                    count *= 2;
                }
            }
        }
        count
    }
}
// Card   3: 40 25 13 65 86  5 35 87  9 30 | 87 76 30 93  5 45 16 40 48 89 78 59 18 12 71 85 66 21 80 28 50 60 49 72 27
fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        // Extract the winning_numbers string
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
            playing_numbers: parsed_playing_numbers.unwrap(),
            winning_numbers: parsed_wining_numbers.unwrap(),
        });
    }
    // for game in games {
    //     println!("{}", game.get_score());
    // }
    let sum: usize = games.iter().map(|game| game.get_score()).sum();
    println!("{}", sum);
    Ok("".to_string())
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
        assert_eq!(result.unwrap(), "13".to_string());
    }
}
