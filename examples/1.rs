use advent_of_code_2022::read_input;

fn main() {
    let lines = read_input(1);

    let mut elves: Vec<Elf> = vec![Elf::default()];

    for line in lines {
        match line.parse::<u32>() {
            Ok(calories) => elves.last_mut().unwrap().calories_held += calories,
            Err(_) => {
                elves.push(Elf::default());
            }
        }
    }

    elves.sort_by(|a, b| b.calories_held.partial_cmp(&a.calories_held).unwrap());
    let max = elves
        .iter()
        .map(|elf| elf.calories_held)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let sum = &elves[0..=2]
        .iter()
        .map(|elf| elf.calories_held)
        .sum::<u32>();

    println!("Most calories held by single elf: {max}");
    println!("Calories held by three elves carrying most calories: {sum}");
}

#[derive(Default, Debug)]
struct Elf {
    calories_held: u32,
}
