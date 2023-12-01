// NOTE never finished!

#![allow(unused)]

fn main() {
    todo!();
    let input = get_packet_pairs();
    println!("{:#?}", input);
}

fn get_packet_pairs() -> Vec<(Packet, Packet)> {
    let pairs = advent_of_code::read_input_string(2022, 13)
        .split("\n\n")
        .map(|pair| {
            let parts = pair.split("\n").collect::<Vec<&str>>();
            let first: Packet = parts[0].into();
            let second: Packet = parts[0].into();
            (first, second)
        })
        .collect::<Vec<(Packet, Packet)>>();

    pairs
}

#[derive(Debug)]
enum Packet {
    Integer(Option<i32>),
    List(Vec<Box<Packet>>),
}

impl From<&str> for Packet {
    fn from(str: &str) -> Self {
        Packet::Integer(Some(0))
    }
}
