use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(puzzle_number: u8) -> Vec<String> {
    let Ok(file) = File::open(format!("./res/{puzzle_number}/input.txt")) else  {
        panic!("The elves must have misplaced the calories manifest...");
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
