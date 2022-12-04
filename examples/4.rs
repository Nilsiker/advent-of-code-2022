use std::ops::RangeInclusive;

use advent_of_code_2022::read_input;

fn main() {
    let pairs = setup_pairs();

    let mut fully_contained_count = 0;
    let mut partial_overlaps_count = 0;

    for (a, b) in pairs {
        // count full overlaps..
        if a.sections.contains(b.sections.start()) && a.sections.contains(b.sections.end()) {
            fully_contained_count += 1;
        } else if b.sections.contains(a.sections.start()) && b.sections.contains(a.sections.end()) {
            fully_contained_count += 1;
        }

        // count partial overlaps..
        if ranges_partially_overlap(a.sections, b.sections) {
            partial_overlaps_count += 1;
        }
    }

    println!("Pairs where sections overlap fully: {fully_contained_count}");
    println!("Pairs where sections overlap partially: {partial_overlaps_count}");
}

fn ranges_partially_overlap(first: RangeInclusive<u32>, other: RangeInclusive<u32>) -> bool {
    first.contains(other.start())
        || first.contains(other.end())
        || other.contains(first.start())
        || other.contains(first.end())
}

fn setup_pairs() -> Vec<(Elf, Elf)> {
    let lines = read_input(4);

    let mut pairs = vec![];
    for line in lines {
        let elf_sections = line.split(',').collect::<Vec<&str>>();
        let sections_1 = elf_sections[0]
            .split('-')
            .map(|c| c.parse::<u32>().expect("This sections manifest suck..."))
            .collect::<Vec<u32>>();

        let sections_2 = elf_sections[1]
            .split('-')
            .map(|c| c.parse::<u32>().expect("This sections manifest suck..."))
            .collect::<Vec<u32>>();
        pairs.push((
            Elf {
                sections: sections_1[0]..=sections_1[1],
            },
            Elf {
                sections: sections_2[0]..=sections_2[1],
            },
        ));
    }
    pairs
}

#[derive(Debug)]
struct Elf {
    sections: RangeInclusive<u32>,
}
