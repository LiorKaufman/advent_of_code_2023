use core::num;
use std::{ops::Range, usize, vec};
// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4
// The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

// The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

// Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

// Consider again the example seed-to-soil map:

// 50 98 2
// 52 50 48
// The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

// The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

// Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

// So, the entire list of seed numbers and their corresponding soil numbers looks like this:

// seed  soil
// 0     0
// 1     1
// ...   ...
// 48    48
// 49    49
// 50    52
// 51    53
// ...   ...
// 96    98
// 97    99
// 98    50
// 99    51

// After we parse our data into differenet hashsets based on differnet data mapping provded
// We want to create a range based on 0 to the max seed value in the seeds
// Then create a hashset based on a range of

// range 10-20
// range2 14-22
// source 13
// 0ffset = 13 - 10 = 3

// 14+3 17
#[derive(Debug)]
pub struct SeedMap {
    mappings: Vec<(Range<usize>, Range<usize>)>,
}

impl SeedMap {
    fn translate(&self, source: usize) -> usize {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}
fn main() {
    let input = include_str!("./input_day_5.txt");
    let output = part1(input);
    println!("{}", output);
}
fn parse_name(input: &str) -> &str {
    println!("input parse name {}", input);
    let output = input;
    output
}
fn parse_line(input: &str) -> (Range<usize>, Range<usize>) {
    // println!("input parse line {}", input);
    let mut z: Vec<&str> = input.split(' ').collect::<Vec<&str>>();

    let parsed: Vec<usize> = z
        .iter_mut()
        .map(|num: &mut &str| num.parse::<usize>().unwrap())
        .collect();
    let destination = parsed[0];
    let source = parsed[1];
    let delta = parsed[2];
    // println!("destination {:?}", destination);
    // println!("source {:?}", source);

    (source..(source + delta), destination..(destination + delta))
}
fn part1(input: &str) -> String {
    let lines: Vec<_> = input.trim().lines().filter(|&x| !x.is_empty()).collect();
    let seeds: Vec<usize> = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    // If another section is found, stop collecting numbers
    // println!("{:?}", seeds);

    // println!("{:?}", lines);
    let mut mappings: Vec<(Range<usize>, Range<usize>)> = vec![];
    let mut all_mappings: Vec<SeedMap> = vec![];
    for line in lines {
        // Skip header lines
        if line.contains(":") {
            if mappings.is_empty() {
            } else {
                all_mappings.push(SeedMap {
                    mappings: mappings.clone(),
                })
            }
            mappings = vec![];
        }
        // Parse mapping
        else {
            let r = parse_line(line);
            // println!(" the line as vec{:?}", r);
            mappings.push(r);
        }
    }

    println!("{:?}", all_mappings);
    let t = translate_seed_maps(seeds, all_mappings);
}
// pub fn translate(&self, source: usize) -> usize {
//     let valid_mapping = self
//         .mappings
//         .iter()
//         .find(|(source_range, _)| source_range.contains(&source));

//     let Some((source_range, destination_range)) = valid_mapping else {
//         return source;
//     };
//     // println!("{:?}", valid_mapping);

//     let offset = source - source_range.start;
//     println!("{}", offset);
//     destination_range.start + offset
// }
pub fn translate_seed_maps(seeds: Vec<usize>, maps: Vec<SeedMap>) -> String {
    let x = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, mapp| mapp.translate(seed)))
        .collect::<Vec<_>>();
    println!("{:?}", x);
    x.iter().min().expect("should have min number").to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    ";

    #[test]
    fn it_works() {
        let result = part1(INPUT);
        assert_eq!(result, "35");
    }
}
