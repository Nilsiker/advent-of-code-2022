use std::collections::HashMap;

use advent_of_code::read_input_lines;

fn main() {
    let lines = read_input_lines(2023, 12);
    let start = std::time::Instant::now();
    let mut springs = vec![];
    let mut groups = vec![];
    let mut springs_unfolded = vec![];
    let mut groups_unfolded = vec![];

    let mut memo = HashMap::new();

    lines.iter().for_each(|line| {
        let mut parts = line.split_whitespace();
        let springs_str = parts.next().unwrap();
        let groups_vec = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let springs_unfolded_str = format!("{springs_str}?").repeat(4);
        let groups_unfolded_vec = groups_vec.repeat(4);
        springs.push(springs_str);
        groups.push(groups_vec);

        springs_unfolded.push(springs_unfolded_str);
        groups_unfolded.push(groups_unfolded_vec);
    });

    let mut sum_a = 0;
    for i in 0..springs.len() {
        // println!("Part A: calculating {i}...");
        sum_a += brute_force_arrangement_count(springs[i], &groups[i], &mut memo);
    }
    let mut sum_b = 0;
    // for i in 0..springs_unfolded.len() {
    //     println!("Part B: calculating {i}...");
    //     sum_b +=
    //         brute_force_arrangement_count(&springs_unfolded[i], &groups_unfolded[i], &mut memo);
    // }

    let elapsed = start.elapsed();
    println!("Part A: {sum_a}");
    println!("Part B: {sum_b}");
    println!("{elapsed:?}");
}

fn brute_force_arrangement_count(
    springs: &str,
    groups: &[usize],
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    let mut sum = 0;
    let variants = get_all_possible_variants(springs);
    for variant in variants {
        let sections = variant
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if sections.len() != groups.len() {
            continue;
        }
        if sections.iter().zip(groups).all(|(el,group_size)| el.len()==*group_size) {
            sum +=1;
        }
    }
    sum
}

fn get_all_possible_variants(springs: &str) -> Vec<String> {
    let mut variants = vec![];
    if springs.contains('?') {
        let damaged = springs.replacen('?', "#", 1);
        let working = springs.replacen('?', ".", 1);
        variants.append(&mut get_all_possible_variants(&damaged));
        variants.append(&mut get_all_possible_variants(&working));
    } else {
        variants.push(springs.to_string());
    }
    variants
}
