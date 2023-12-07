use advent_of_code::read_input_lines;

const PART_B: bool = true;

fn main() {
    let lines = read_input_lines(2023, 7);
    let start = std::time::Instant::now();
    let hands = lines.into_iter().map(Hand::from).collect::<Vec<Hand>>();

    let mut parsed_hands: Vec<(Type, &Vec<Card>, usize)> = hands
        .iter()
        .map(|hand| (hand.get_type(), &hand.cards, hand.bid))
        .collect();

    parsed_hands.sort();

    let sum: usize = parsed_hands
        .iter()
        .enumerate()
        .map(|(index, h)| h.2 * (index + 1))
        .sum();

    let elapsed = start.elapsed();

    for hand in &parsed_hands {
        println!("{hand:?}\n");
    }
    println!("{sum}");
    // let sum: usize = sorted_hands
    //     .iter()
    //     .enumerate()
    //     .map(|(index, hand)| hand.bid * (index + 1))
    //     .sum();

    // println!("{sorted_hands:#?}");
    // println!("Part A gives a winning of {sum}");
    println!("{elapsed:?}");
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}
impl Hand {
    fn get_type(&self) -> Type {
        let Self { cards, .. } = self;

        let mut counts = std::collections::HashMap::new();

        for card in &self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let max = counts.iter().map(|e| e.1).max().expect("some max");
        let pairs = counts.iter().map(|e| e.1).filter(|n| **n == 2).count();
        println!("{max}");

        match max {
            4 => Type::FourOfAKind,
            3 => {
                if pairs > 0 {
                    Type::FullHouse
                } else {
                    Type::ThreeOfAKind
                }
            }
            2 => {
                if pairs == 2 {
                    Type::TwoPair
                } else {
                    Type::OnePair
                }
            }
            1 => Type::HighCard,
            _ => Type::FiveOfAKind,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_order = self
            .get_type()
            .partial_cmp(&other.get_type())
            .expect("can compare type");
        for (a, b) in self.cards.iter().zip(&other.cards) {
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                _ => return Some(type_order.then(a.cmp(&b))),
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Type> for usize {
    fn from(value: &Type) -> Self {
        match value {
            Type::FiveOfAKind => 7,
            Type::FourOfAKind => 6,
            Type::FullHouse => 5,
            Type::ThreeOfAKind => 4,
            Type::TwoPair => 3,
            Type::OnePair => 2,
            Type::HighCard => 1,
        }
    }
}

#[derive(Hash, Debug, Clone)]
struct Card(char);
impl Card {
    fn get_worth(&self) -> usize {
        match self.0 {
            'T' => 10,
            'J' => {
                if PART_B {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => self.0.to_digit(10).expect("parseable u32") as usize,
        }
    }
}
impl Eq for Card {}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_worth().partial_cmp(&other.get_worth())
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_worth().cmp(&other.get_worth())
    }
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        let mut parts = value.split(' ');
        let cards = parts
            .next()
            .expect("cards part")
            .chars()
            .map(|char| Card(char))
            .collect();

        let bid: usize = parts
            .next()
            .expect("bid part")
            .parse()
            .expect("parseable to usize");
        Self { cards, bid }
    }
}
