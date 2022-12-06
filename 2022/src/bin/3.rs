use advent_of_code_2022::read_input_lines;

fn main() {
    let rucksacks = setup_rucksacks();

    let shared_item_priority_sum: u32 = rucksacks
        .iter()
        .map(|rucksack| rucksack.shared_item_priority)
        .sum();
    println!("Part 1: Shared priority sum is {shared_item_priority_sum}");

    let groups = setup_groups(&rucksacks);
    let mut badges_priority_sum = 0;

    for group in groups {
        let badge = group.get_badge_item();
        badges_priority_sum += get_priority(badge);
    }

    println!("Part 2: Shared priority sum for the group badges {badges_priority_sum}");
}

fn setup_rucksacks() -> Vec<Rucksack> {
    let lines = read_input_lines(3);
    let mut rucksacks = vec![];

    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut rucksack = Rucksack::new(chars);

        if let Some(item) = rucksack.find_shared_item() {
            rucksack.shared_item_priority = get_priority(item);
        }

        rucksacks.push(rucksack);
    }
    rucksacks
}

fn setup_groups(rucksacks: &Vec<Rucksack>) -> Vec<Group> {
    let mut groups = vec![];
    for i in (0..rucksacks.len()).step_by(3) {
        groups.push(Group {
            rucksacks: &rucksacks[i..=i + 2],
        });
    }
    groups
}

fn get_priority(char: char) -> u32 {
    if char.is_uppercase() {
        char as u32 - 38
    } else {
        char as u32 - 96
    }
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<char>,
    shared_item_priority: u32,
}
impl Rucksack {
    fn new(items: Vec<char>) -> Self {
        Self {
            items,
            shared_item_priority: 0,
        }
    }

    fn first_compartment(&self) -> &[char] {
        let items = &self.items;
        let len = items.len();
        &items[..len / 2]
    }

    fn second_compartment(&self) -> &[char] {
        let items = &self.items;
        let len = items.len();
        &items[len / 2..]
    }

    fn find_shared_item(&self) -> Option<char> {
        let shared = self
            .first_compartment()
            .iter()
            .filter(|char| self.second_compartment().contains(char))
            .collect::<Vec<&char>>();

        let item = shared.get(0);

        match item {
            Some(char) => Some(**char),
            None => None,
        }
    }

    fn all_items(&self) -> &Vec<char> {
        &self.items
    }
}

struct Group<'a> {
    rucksacks: &'a [Rucksack],
}
impl<'a> Group<'a> {
    fn get_badge_item(&'a self) -> char {
        let first = self.rucksacks[0].all_items();
        let second = self.rucksacks[1].all_items();
        let third = self.rucksacks[2].all_items();
        let intersection = first
            .iter()
            .filter(|c| second.contains(c))
            .filter(|c| third.contains(c))
            .collect::<Vec<&char>>();

        let badge = intersection.get(0);
        match badge {
            Some(badge) => **badge,
            None => panic!("This group has no badge?! It can't be!"),
        }
    }
}
