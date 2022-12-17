
fn main() {
    let input = get_packet_pairs();
    println!("{:#?}", input);
}

fn get_packet_pairs() -> Vec<(Packet,Packet)> {
    let pairs = include_str!("../../inputs/13.txt")
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
