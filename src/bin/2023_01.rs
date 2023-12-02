// NOTE that was unexpectedly hard for day 1!
// I first debated using the regex crate, but it turns out it does not support overlapping matches:
//      Consider the string "eightwo" - regex will only match "eight", since the iterator will then continue on "w"
// Because of this, I instead loop over all identifiable tokens, and update the "first" and "last" token respectively.
// However, this means I have to scan the lines twice, once from the start and once from the end. I suspect this is not necessary, with a more clever solution!

use advent_of_code::read_input_lines;

const TOKENS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() {
    let tokens_part_1 = &TOKENS[0..9];

    let sum_part_1: u32 = read_input_lines(2023, 1)
        .iter()
        .map(|line| get_first_and_last_token(line, tokens_part_1))
        .map(|tokens| str_to_str(tokens.0).to_string() + str_to_str(tokens.1))
        .map(|joined| joined.parse::<u32>().expect("parseable str"))
        .sum::<u32>();

    let sum_part_2: u32 = read_input_lines(2023, 1)
        .iter()
        .map(|line| get_first_and_last_token(line, &TOKENS))
        .map(|tokens| str_to_str(tokens.0).to_string() + str_to_str(tokens.1))
        .map(|joined| joined.parse::<u32>().expect("parseable str"))
        .sum::<u32>();

    println!("Part 1: All calibration values amount to {sum_part_1:?}");
    println!("Part 2: All calibration values amount to {sum_part_2:?}");
}

fn get_first_and_last_token(string: &str, tokens: &[&'static str]) -> (&'static str, &'static str) {
    let mut first_token: (usize, &str) = (usize::MAX, "");
    let mut last_token: (usize, &str) = (usize::MIN, "");
    tokens.iter().for_each(|token| {
        if let Some(index) = string.find(token) {
            if first_token.0 > index {
                first_token = (index, token);
            }
        }
        if let Some(index) = string.rfind(token) {
            if last_token.0 <= index {
                last_token = (index, token);
            }
        }
    });
    (first_token.1, last_token.1)
}

fn str_to_str(str: &str) -> &str {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => str,
    }
}
