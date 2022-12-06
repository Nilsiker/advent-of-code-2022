use advent_of_code_2022::read_input_string;

fn main() {
    let data = read_input_string(6);
    let packet_marker_length = 4;
    let message_marker_length = 14;
    for i in 0..data.chars().count() {
        if let None = unique(&data[i..i + packet_marker_length]) {
            println!("First packet marker at {}", i + packet_marker_length);
            break;
        }
    }
    for i in 0..data.chars().count() {
        if let None = unique(&data[i..i + message_marker_length]) {
            println!("First message marker at {}", i + message_marker_length);
            break;
        }
    }
}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}
