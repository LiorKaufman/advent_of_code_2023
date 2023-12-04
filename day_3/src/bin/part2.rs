fn main() {
    let input = include_str!("./input_day_3.txt");
    let output = part1(input);
    println!("{}", output);
}
#[derive(Debug, Clone)]
struct Coordinates {
    start_row: usize,
    start_col: usize,
}

#[derive(Debug, Clone)]
struct Number {
    value: Vec<char>,
    coordinates: Vec<Coordinates>,
}
#[derive(Debug)]

struct Symbol {
    coordinates: Coordinates,
    adjacent_coordinates: Vec<Coordinates>,
}
fn surrounding_coordinates(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<Coordinates> {
    let mut coords = Vec::new();

    // Check above
    if row > 0 {
        coords.push(Coordinates {
            start_row: row - 1,
            start_col: col,
        });

        // Check diagonally upper-left
        if col > 0 {
            coords.push(Coordinates {
                start_row: row - 1,
                start_col: col - 1,
            });
        }

        // Check diagonally upper-right
        if col + 1 < max_col {
            coords.push(Coordinates {
                start_row: row - 1,
                start_col: col + 1,
            });
        }
    }

    // Check below
    if row + 1 < max_row {
        coords.push(Coordinates {
            start_row: row + 1,
            start_col: col,
        });

        // Check diagonally lower-left
        if col > 0 {
            coords.push(Coordinates {
                start_row: row + 1,
                start_col: col - 1,
            });
        }

        // Check diagonally lower-right
        if col + 1 < max_col {
            coords.push(Coordinates {
                start_row: row + 1,
                start_col: col + 1,
            });
        }
    }

    // Check left
    if col > 0 {
        coords.push(Coordinates {
            start_row: row,
            start_col: col - 1,
        });
    }

    // Check right
    if col + 1 < max_col {
        coords.push(Coordinates {
            start_row: row,
            start_col: col + 1,
        });
    }

    coords
}

fn find_matching_numbers(numbers: Vec<Number>, allsymbols: Vec<Symbol>) -> String {
    let mut num_gears: Vec<Vec<usize>> = Vec::new();
    for symbol in &allsymbols {
        let mut touching_numbers = Vec::new();

        for number in &numbers {
            let mut is_touching = false;

            for num_coord in &number.coordinates {
                for sym_coord in &symbol.adjacent_coordinates {
                    if num_coord.start_row == sym_coord.start_row
                        && num_coord.start_col == sym_coord.start_col
                    {
                        is_touching = true;
                        break; // No need to check other coordinates if one match is found
                    }
                }
            }

            if is_touching {
                // Convert Vec<char> to String and then try to parse it as usize
                if let Ok(num_value) = number.value.iter().collect::<String>().parse::<usize>() {
                    touching_numbers.push(num_value);
                }
            }
        }

        if touching_numbers.len() >= 2 {
            num_gears.push(touching_numbers); // Add to num_gears if at least 2 numbers are touching the symbol
        }
    }
    for number in &num_gears {
        println!("{:?} ", number);
    }
    let sum: usize = num_gears
        .iter()
        .map(|inner_vec| inner_vec.iter().product::<usize>())
        .sum();
    sum.to_string()
}
fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let max_row = lines.len();
    let mut numbers = Vec::new();
    let mut allSymbols: Vec<Symbol> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.trim().chars().collect();
        let mut col = 0;
        let maxCol = chars.len();
        while col < chars.len() {
            match chars[col] {
                // If the character is a digit
                c if c.is_digit(10) => {
                    let mut number_value = Vec::new();
                    let mut num_coords: Vec<Coordinates> = Vec::new();

                    while col < chars.len() && chars[col].is_digit(10) {
                        number_value.push(chars[col]);
                        num_coords.push(Coordinates {
                            start_row: row,
                            start_col: col,
                        });
                        col += 1;
                    }

                    let number = Number {
                        value: number_value,
                        coordinates: num_coords,
                    };
                    numbers.push(number);
                }
                // If the character is a symbol (you can list specific symbols here)
                '*' => {
                    let adjacent_coordinates = surrounding_coordinates(row, col, max_row, maxCol);
                    allSymbols.push(Symbol {
                        coordinates: Coordinates {
                            start_row: row,
                            start_col: col,
                        },
                        adjacent_coordinates,
                    });
                    // Handle symbol logic here
                    col += 1; // Move to the next column
                }
                // If the character is a dot or anything else
                _ => {
                    col += 1; // Move to the next column
                }
            }
        }
    }
    // for number in &numbers {
    //     println!(
    //         "{:?}: {:?}",
    //         number.value.iter().collect::<String>(),
    //         number.coordinates
    //     );
    // }
    // for sym in &allSymbols {
    //     println!("{:?}", sym.coordinates);
    // }
    let f = find_matching_numbers(numbers, allSymbols);
    println!("{}", f);
    f.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
467..114..
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
        assert_eq!(result, "467835");
    }
}
