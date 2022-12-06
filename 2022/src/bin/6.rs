use std::collections::HashSet;

use advent_of_code_2022::read_input_string;

fn main() {
    let data = read_input_string(6);
    let packet_marker_length = 4;
    let message_marker_length = 14;
    
    for i in 0..data.chars().count() {
        let candidate_marker = &data[i..i + packet_marker_length]
            .chars()
            .collect::<HashSet<char>>();
        if candidate_marker.len() == packet_marker_length {
            println!("First message marker at {}", i + packet_marker_length);
            break;
        }
    }
    for i in 0..data.chars().count() {
        let candidate_marker = &data[i..i + message_marker_length]
            .chars()
            .collect::<HashSet<char>>();
        if candidate_marker.len() == message_marker_length {
            println!("First message marker at {}", i + message_marker_length);
            break;
        }
    }
}
