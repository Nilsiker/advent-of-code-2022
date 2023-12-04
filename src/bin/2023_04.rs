use advent_of_code::read_input_lines;

fn main() {
    let start = std::time::Instant::now();

    let cards: Vec<Card> = read_input_lines(2023, 4).iter().map(Card::from).collect();
    let score_sum: usize = cards.iter().map(|card| card.score()).sum();

    let mut card_counters = vec![1usize; cards.len()];

    // NOTE: ignore last card, since it doesn't reward subsequent cards!
    for i in 0..card_counters.len() - 1 {
        let this_card_count = card_counters[i];
        let winning_numbers = cards[i].winning_numbers();
        let last_index = (i + winning_numbers).min(cards.len());
        card_counters[i + 1..=last_index]
            .iter_mut()
            .for_each(|count| *count += this_card_count);
    }

    let cards_sum: usize = card_counters.iter().map(|count| count).sum();
    let elapsed = start.elapsed();

    println!("Part A would give a score sum of {score_sum:#?}.");
    println!("Part B would give a total of {} cards.", cards_sum);
    println!("{elapsed:?}");
}

#[derive(Debug, Clone)]
struct Card {
    key: Vec<usize>,
    numbers: Vec<usize>,
}
impl From<&String> for Card {
    fn from(value: &String) -> Self {
        let colon_index = value.find(':').expect("colon exists");
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

        Self { key, numbers }
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
