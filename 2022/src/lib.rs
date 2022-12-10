use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub fn read_input_lines(puzzle_number: u8) -> Vec<String> {
    let Ok(file) = File::open(format!("./inputs/{puzzle_number}.txt")) else  {
        panic!("The elves must have misplaced the input data...");
    };

    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|element| {
            if let Ok(a) = element {
                a
            } else {
                panic!("The input is corrupt?! NOOOOO!")
            }
        })
        .collect()
}

pub fn read_input_string(puzzle_number: u8) -> String {
    let Ok(file) = File::open(format!("./inputs/{puzzle_number}.txt")) else  {
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
