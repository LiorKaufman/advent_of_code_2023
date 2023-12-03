fn main() {
    let input = include_str!("./input_day_3.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> String {
    let mut results: Vec<String> = vec![];

    for line in input.lines() {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            for word in trimmed_line.split_whitespace() {
                let mut first_char_that_is_digit = None;
                let mut last_char_that_is_digit = None;

                // Process from start to find the first digit or non-digit word
                for char in word.chars() {
                    if char.is_digit(10) {
                        first_char_that_is_digit = Some(char.to_string());
                        break;
                    }
                }

                // Process from end to find the last digit or non-digit word
                for char in word.chars().rev() {
                    if char.is_digit(10) {
                        last_char_that_is_digit = Some(char.to_string());
                        break;
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn it_works() {
        let result = part1(INPUT);
        assert_eq!(result, "4361");
    }
}
