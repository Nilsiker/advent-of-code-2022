// NOTE: This one was very rough. After a lot of tips and tricks from the subreddit, I managed to grasp a solution.
// It is very bruteforcey, but uses a cache arrangement counts for all possible ? assumptions.
// Part A + B runs for about 150ms on my ThinkPad T480, which I'm pretty happy with given the

use std::collections::HashMap;

use advent_of_code::read_input_lines;

fn main() {
    let lines = read_input_lines(2023, 12);
    let start = std::time::Instant::now();

    let mut springs_vec = vec![];
    let mut groups_vec = vec![];
    let mut expanded_springs_vec = vec![];
    let mut expanded_groups_vec = vec![];

    lines.iter().for_each(|line| {
        let mut parts = line.split(' ');
        let springs = parts.next().unwrap();
        let groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        springs_vec.push(springs);
        groups_vec.push(groups.clone());
        expanded_springs_vec.push(vec![springs; 5].join("?"));
        expanded_groups_vec.push(groups.repeat(5));
    });
    let sum: usize = springs_vec
        .iter()
        .zip(groups_vec)
        .map(|(springs, groups)| get_arrangements_count_using_memo(springs, &groups, HashMap::new()))
        .sum();
    let expanded_sum: usize = expanded_springs_vec
        .iter()
        .zip(expanded_groups_vec)
        .map(|(springs, groups)| get_arrangements_count_using_memo(springs, &groups, HashMap::new()))
        .sum();
    let elapsed = start.elapsed();
    println!("Possible arrangements are {sum}");
    println!("Expanded, the sum is {expanded_sum}");
    println!("{elapsed:?}");
}

fn get_arrangements_count_using_memo(
    springs: &str,
    groups: &[usize],
    mut memo: HashMap<(usize, usize, usize), usize>,
) -> usize {
    recurse_check(&mut memo, springs, groups, 0, 0, 0)
}

fn recurse_check(
    memo: &mut HashMap<(usize, usize, usize), usize>,
    springs: &str,
    groups: &[usize],
    spring_index: usize,
    group_to_check_index: usize,
    current_group_size: usize,
) -> usize {
    // reached end of springs str
    if spring_index == springs.len() {
        // check if we've found all groups
        if group_to_check_index == groups.len() {
            return 1;
        }

        // the line ends with a "damaged" symbol and we've matched that last group
        if group_to_check_index == groups.len() - 1
            && groups[group_to_check_index] == current_group_size
        {
            return 1;
        }

        return 0;
    }

    let current_spring = springs.chars().nth(spring_index).unwrap();
    match current_spring {
        '.' => {
            // we haven't started a sequence yet, skipping!
            if current_group_size == 0 {
                return recurse_check(
                    memo,
                    springs,
                    groups,
                    spring_index + 1,
                    group_to_check_index,
                    current_group_size,
                );
            }

            // we stop a sequence that is incorrect
            if current_group_size != groups[group_to_check_index] {
                return 0;
            }

            // we stop a sequence that is CORRECT! Move on to next group
            return recurse_check(
                memo,
                springs,
                groups,
                spring_index + 1,
                group_to_check_index + 1,
                0,
            );
        }

        '#' => {
            // we either encounter a damaged spring when we're not expecting more groups
            // or we encounter a damaged spring that brings us over the expected group size
            if group_to_check_index == groups.len()
                || current_group_size + 1 > groups[group_to_check_index]
            {
                return 0;
            }

            // we're either in a sequence, or starting a sequence that is still potentially valid
            return recurse_check(
                memo,
                springs,
                groups,
                spring_index + 1,
                group_to_check_index,
                current_group_size + 1,
            );
        }

        // UNKNOWN
        _ => {
            // we encounter a determined unknown that we already have a cached answer for
            // for this exact spring index, group check and current group size
            // early returning
            if let Some(answer) =
                memo.get(&(spring_index, group_to_check_index, current_group_size))
            {
                return *answer;
            }

            // if we don't have a cached answer, we explore the unknown spring
            let mut ways = 0;

            // if we're not in a sequence, explore possible arrangements if this unknown is OPERATIONAL
            if current_group_size == 0 {
                ways += recurse_check(
                    memo,
                    springs,
                    groups,
                    spring_index + 1,
                    group_to_check_index,
                    current_group_size,
                );
            }

            // if we're still checking groups, and need more damaged springs in the current group
            // explore arrangements if this unknown is DAMAGED
            if group_to_check_index < groups.len()
                && current_group_size < groups[group_to_check_index]
            {
                ways += recurse_check(
                    memo,
                    springs,
                    groups,
                    spring_index + 1,
                    group_to_check_index,
                    current_group_size + 1,
                );
            }

            // if we're in a sequence and have all the damaged springs we need,
            // explore arrangements for the next group, resetting current group size
            if group_to_check_index < groups.len()
                && current_group_size == groups[group_to_check_index]
            {
                ways += recurse_check(
                    memo,
                    springs,
                    groups,
                    spring_index + 1,
                    group_to_check_index + 1,
                    0,
                );
            }

            // for all the ways found, cache the count for this specific spring, group to check, and current group size
            // to be used in future recursions to speed things up
            memo.insert(
                (spring_index, group_to_check_index, current_group_size),
                ways,
            );
            return ways;
        }
    };
}
