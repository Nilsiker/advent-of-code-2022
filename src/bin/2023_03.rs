use advent_of_code::{read_input_lines, read_input_string};
use regex::Regex;

fn main() {
    let start = std::time::Instant::now();

    let (numbers, mut symbols) = get_numbers_and_symbols();

    let mut adjacent_numbers = vec![];
    for symbol in &mut symbols {
        for number in &numbers {
            if number.is_adjacent_to(symbol.x, symbol.y) {
                adjacent_numbers.push(number);
                symbol.adjacent_numbers.push(number);
            }
        }
    }

    let sum: usize = adjacent_numbers.iter().map(|number| number.value).sum();
    println!("The sum of all adjacent numbers is {sum}");

    let gear_ratio_sum: usize = symbols
        .iter()
        .filter_map(|symbol| symbol.get_gear_ratio())
        .sum();
    println!("The sum of all gear ratios is {gear_ratio_sum}");

    let elapsed = start.elapsed();
    println!("{elapsed:?}")
}

fn get_numbers_and_symbols() -> (Vec<EnginePartNumber>, Vec<Symbol<'static>>) {
    let numbers_regex = Regex::new(r"\b\d+\b").unwrap();
    let symbols_regex = Regex::new(r"[^\d.]").unwrap();

    let mut numbers = vec![];
    let mut symbols = vec![];
    read_input_lines(2023, 3)
        .into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            for mat in numbers_regex.find_iter(&line) {
                numbers.push(EnginePartNumber {
                    x_start: mat.start(),
                    x_end: mat.end() - 1,
                    y,
                    value: mat
                        .as_str()
                        .parse()
                        .expect("found number is parseable to usize"),
                });
            }

            for mat in symbols_regex.find_iter(&line) {
                symbols.push(Symbol {
                    value: mat.as_str().chars().next().unwrap(),
                    x: mat.start(),
                    y,
                    adjacent_numbers: vec![],
                });
            }
        });
    (numbers, symbols)
}

#[derive(Debug)]
struct EnginePartNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: usize,
}
impl EnginePartNumber {
    fn is_adjacent_to(&self, x: usize, y: usize) -> bool {
        (y - 1..=y + 1).contains(&self.y)
            && ((x - 1..=x + 1).contains(&self.x_start) || (x - 1..=x + 1).contains(&self.x_end))
    }
}

#[derive(Debug)]
struct Symbol<'a> {
    value: char,
    x: usize,
    y: usize,
    adjacent_numbers: Vec<&'a EnginePartNumber>,
}
impl<'a> Symbol<'a> {
    fn get_gear_ratio(&self) -> Option<usize> {
        if self.value == '*' && self.adjacent_numbers.len() == 2 {
            let ratio = self.adjacent_numbers.iter().map(|num| num.value).product();
            Some(ratio)
        } else {
            None
        }
    }
}
