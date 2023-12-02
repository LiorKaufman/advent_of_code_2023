fn main() {
    let input = include_str!("./input_day_1.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> String {
    let mut results: Vec<String> = vec![];

    for line in input.lines() {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            for word in trimmed_line.split_whitespace() {
                let mut first_non_digit_str = String::new();
                let mut first_char_that_is_digit = None;
                let mut last_non_digit_str = String::new();
                let mut last_char_that_is_digit = None;

                // Process from start to find the first digit or non-digit word
                for char in word.chars() {
                    match char.is_digit(10) {
                        true => {
                            first_char_that_is_digit = Some(char.to_string());
                            break;
                        }
                        false => {
                            process_char(
                                char,
                                &mut first_non_digit_str,
                                &mut first_char_that_is_digit,
                            );
                            if first_char_that_is_digit.is_some() {
                                break;
                            }
                        }
                    }
                }

                // Process from end to find the last digit or non-digit word
                for char in word.chars().rev() {
                    match char.is_digit(10) {
                        true => {
                            last_char_that_is_digit = Some(char.to_string());
                            break;
                        }
                        false => {
                            process_char_reverse(
                                char,
                                &mut last_non_digit_str,
                                &mut last_char_that_is_digit,
                            );
                            if last_char_that_is_digit.is_some() {
                                break;
                            }
                        }
                    }
                }

                // Combine the results as needed
                if let (Some(first_digit), Some(last_digit)) =
                    (first_char_that_is_digit, last_char_that_is_digit)
                {
                    let combined = format!("{}{}", first_digit, last_digit);
                    results.push(combined);
                }
            }
        }
    }
    let sum: i32 = results.iter().filter_map(|s| s.parse::<i32>().ok()).sum();
    sum.to_string()
}

fn process_char(char: char, non_digit_str: &mut String, digit_found: &mut Option<String>) {
    non_digit_str.push(char);
    check_for_number_word(non_digit_str, digit_found);
}

fn process_char_reverse(char: char, non_digit_str: &mut String, digit_found: &mut Option<String>) {
    non_digit_str.insert(0, char);
    check_for_number_word(non_digit_str, digit_found);
}

fn check_for_number_word(non_digit_str: &mut String, digit_found: &mut Option<String>) {
    // A mapping from number words to their digit representations
    let number_words = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let number_map: std::collections::HashMap<_, _> = number_words.into_iter().collect();

    // Check if the non-digit string contains any of the number words
    for (number_word, digit) in &number_map {
        if non_digit_str.contains(number_word) {
            *digit_found = Some(digit.to_string());
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn it_works() {
        let result = part2(INPUT);
        assert_eq!(result, "281");
    }
}
