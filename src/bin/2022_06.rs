use std::collections::HashSet;

use advent_of_code::read_input_string;

fn main() {
    let data = read_input_string(2022, 6);

    if let Some(index) = find_end_of_segment_with_duplicate_characters(&data, 4) {
        println!("\nFirst packet marker ends at {}", index);
    }

    if let Some(index) = find_end_of_segment_with_duplicate_characters(&data, 14) {
        println!("First message marker ends at {}\n", index)
    }
}

fn find_end_of_segment_with_duplicate_characters(
    data: &str,
    segment_length: usize,
) -> Option<usize> {
    let mut set = HashSet::<char>::new();

    for i in 0..data.chars().count() {
        let slice = &data[i..i + segment_length];
        slice.chars().for_each(|char| {
            set.insert(char);
        });
        if set.len() == segment_length {
            return Some(i + segment_length);
        } else {
            set.clear();
        }
    }
    None
}
