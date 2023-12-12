use std::collections::HashMap;

use advent_of_code::read_input_lines;

fn main() {
    let lines = read_input_lines(2023, 12);
    let start = std::time::Instant::now();
    let mut springs = vec![];
    let mut groups = vec![];
    let mut springs_unfolded = vec![];
    let mut groups_unfolded = vec![];

    // let mut memo = HashMap::new();

    lines.iter().for_each(|line| {
        let mut parts = line.split_whitespace();
        let springs_str = parts.next().unwrap();
        let groups_vec = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let springs_unfolded_str = format!("{springs_str}?").repeat(5);
        let groups_unfolded_vec = groups_vec.repeat(5);
        springs.push(springs_str);
        groups.push(groups_vec);

        springs_unfolded.push(springs_unfolded_str);
        groups_unfolded.push(groups_unfolded_vec);
    });

    let mut sum = 0;
    for i in 0..springs.len() {
        sum += brute_force_arrangement_count(springs[i], &groups[i]);
    }

    // for i in 0..springs_unfolded.len() {
    //     sum += brute_force_arrangement_count(&springs_unfolded[i], &groups_unfolded[i]);
    // }

    let elapsed = start.elapsed();
    println!("{sum}");
    println!("{elapsed:?}");
}

fn is_possible_arrangement(spring_str: &str, groups: &[usize]) -> bool {
    let springs = spring_str
        .split('.')
        .filter(|a| !a.is_empty())
        .collect::<Vec<_>>();

    // early return false if the parts aren't as many as the groups (needs more dotage!)
    if springs.len() != groups.len() {
        return false;
    }

    springs
        .iter()
        .zip(groups.iter())
        .all(|(spring, instr)| spring.len() == *instr)
}

fn brute_force_arrangement_count(spring_str: &str, groups: &[usize]) -> usize {
    if spring_str.contains('?') {
        let working_springs = spring_str.replacen('?', ".", 1);
        let damaged_spring = spring_str.replacen('?', "#", 1);
        brute_force_arrangement_count(&working_springs, groups)
            + brute_force_arrangement_count(&damaged_spring, groups)
    } else {
        if is_possible_arrangement(spring_str, groups) {
            1
        } else {
            0
        }
    }
}
