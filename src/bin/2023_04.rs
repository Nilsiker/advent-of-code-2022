// NOTE Ok, this takes about 4s on my machine - that's HEAVY! Surely, there's a better way.


use advent_of_code::read_input_lines;

fn main() {
    let start = std::time::Instant::now();

    let cards: Vec<Card> = read_input_lines(2023, 4).iter().map(Card::from).collect();
    let score_sum: usize = cards.iter().map(|card| card.score()).sum();
    let mut copies: Vec<Card> = vec![];
    let mut copies_count = 0;

    for card in &cards {
        let winning_numbers = card.winning_numbers();
        for i in 1..=winning_numbers {
            if let Some(card) = cards.get(card.id + i - 1) {
                copies_count += 1;
                copies.push(card.clone())
            }
        }
    }

    while let Some(copy) = copies.pop() {
        let winning_numbers = copy.winning_numbers();

        for i in 1..=winning_numbers {
            if let Some(card) = cards.get(copy.id + i - 1) {
                copies_count += 1;
                copies.push(card.clone())
            }
        }
    }

    let elapsed = start.elapsed();

    println!("Part A would give a score sum of {score_sum:#?}.");
    println!(
        "Part B would give a total of {} cards.",
        cards.len() + copies_count
    );
    println!("{elapsed:?}");
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    key: Vec<usize>,
    numbers: Vec<usize>,
}
impl From<&String> for Card {
    fn from(value: &String) -> Self {
        let colon_index = value.find(':').expect("colon exists");
        let id = value[0..colon_index]
            .split(' ')
            .next_back()
            .expect("expect an id after Card")
            .parse()
            .expect("parseable to usize");

        let pipe_index = value.find('|').expect("pipe exists");
        let key: Vec<usize> = value[colon_index + 1..pipe_index]
            .split(' ')
            .filter(|str| !str.is_empty())
            .map(|str| str.parse().expect("parseable to usize"))
            .collect();

        let numbers: Vec<usize> = value[pipe_index + 1..]
            .split(' ')
            .filter(|str| !str.is_empty())
            .map(|str| str.parse().expect("parseable to usize"))
            .collect();

        Self { id, key, numbers }
    }
}
impl Card {
    fn score(&self) -> usize {
        let winners = self.winning_numbers();
        if winners == 0 {
            winners
        } else {
            1 << (winners - 1)
        }
    }

    fn winning_numbers(&self) -> usize {
        let mut winners = 0;
        for number in &self.numbers {
            if self.key.contains(number) {
                winners += 1;
            }
        }
        winners
    }
}
