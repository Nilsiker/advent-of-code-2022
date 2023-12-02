use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub fn read_input_lines(year: u16, puzzle_number: u8) -> Vec<String> {
    let Ok(file) = File::open(format!("./inputs/{year}_{puzzle_number:02}.txt")) else {
        panic!("The elves must have misplaced the input data...");
    };

    BufReader::new(file)
        .lines()
        .map(|element| {
            if let Ok(a) = element {
                a
            } else {
                panic!("The input is corrupt?! NOOOOO!")
            }
        })
        .collect()
}

pub fn read_input_blocks(year: u16, puzzle_number: u8) -> Vec<String> {
    let string = read_input_string(year, puzzle_number);

    let blocks: Vec<String> = string.split("\r\n\r\n").map(String::from).collect();
    blocks
}

pub fn read_input_string(year: u16, puzzle_number: u8) -> String {
    let Ok(file) = File::open(format!("./inputs/{year}_{puzzle_number:02}.txt")) else {
        panic!("The elves must have misplaced the input data...");
    };

    let mut buf: String = String::new();
    BufReader::new(file)
        .read_to_string(&mut buf)
        .unwrap_or_else(|e| {
            panic!("error parsing day {puzzle_number} input as string.\nError: {e}")
        });
    buf
}
