// NOTE It took me a great while getting the reflection-loop right,
// but this finishes in about 500µs, which is a great runtime imo!
// 
// The algorithm places the reflection line, corrects for smudges 
// and reverts the correction and increments the reflection line
// if we find an invalid line comparison.
//
// When we reach outside of the lines vector AND the smudge is corrected,
// this counts as a valid reflection line, and we return the index of the reflection line
// (reflection line is imagined to be placed AFTER the index)

use std::collections::VecDeque;

use advent_of_code::read_input_blocks;

fn main() {
    let blocks = read_input_blocks(2023, 13);
    let start = std::time::Instant::now();

    let patterns = blocks
        .iter()
        .map(|block| {
            Pattern(
                block
                    .split('\n')
                    .filter(|part| !part.is_empty())
                    .map(String::from)
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut smudged_sum = 0;

    for pattern in patterns {
        let result = check(&pattern, false);
        sum += match result.0 {
            Reflection::Horizontal => result.1 * 100,
            Reflection::Vertical => result.1,
        };
        let result = check(&pattern, true);
        smudged_sum += match result.0 {
            Reflection::Horizontal => result.1 * 100,
            Reflection::Vertical => result.1,
        };
    }

    let elapsed = start.elapsed();
    println!("sum is {sum} (with smudge {smudged_sum})");
    println!("{elapsed:?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Reflection {
    Vertical,
    Horizontal,
}

fn check(pattern: &Pattern, smudged: bool) -> (Reflection, usize) {
    fn inner(elements: &[String], smudged: bool) -> usize {
        let mut queue = VecDeque::new();
        let mut pattern = elements.to_vec();

        queue.push_front((1, 0, 1, smudged));

        while !queue.is_empty() {
            let (mirror_line, left, right, smudge_remains) =
                queue.pop_front().expect("element exists");

            if mirror_line as usize == pattern.len() {
                return 0; // could not place reflection line
            }

            if smudge_remains {
                if left < 0 || right == pattern.len() {
                    queue.push_front((
                        mirror_line + 1,
                        mirror_line,
                        mirror_line as usize + 1,
                        true,
                    ));
                    // we found a reflection, but we haven't corrected the smudge
                    // keep looking
                    continue;
                }
            } else if left < 0 || right == pattern.len() {
                // checking out of bounds means we've found a valid reflection line
                return mirror_line as usize;
            }

            // check if reflection line is still valid
            let left_string = &pattern[left as usize];
            let right_string = &pattern[right];
            if smudge_remains {
                let mismatching = left_string
                    .chars()
                    .zip(right_string.chars())
                    .filter(|(a, b)| a != b)
                    .count();

                match mismatching {
                    0 => queue.push_front((mirror_line, left - 1, right + 1, true)),
                    1 => {
                        pattern[left as usize] = pattern[right].clone();
                        queue.push_front((mirror_line, left - 1, right + 1, false))
                    }
                    _ => queue.push_front((
                        mirror_line + 1,
                        mirror_line,
                        mirror_line as usize + 1,
                        smudged,
                    )),
                }
            } else {
                if left_string == right_string {
                    queue.push_front((mirror_line, left - 1, right + 1, false));
                } else {
                    if smudged {
                        pattern = elements.to_vec();
                    }
                    queue.push_front((
                        mirror_line + 1,
                        mirror_line,
                        mirror_line as usize + 1,
                        smudged,
                    ));
                }
            }
        }
        unreachable!()
    }

    let horizontal = inner(pattern.rows(), smudged);
    if horizontal > 0 {
        (Reflection::Horizontal, horizontal)
    } else {
        (Reflection::Vertical, inner(&pattern.cols(), smudged))
    }
}

#[derive(Debug)]
struct Pattern(Vec<String>);
impl Pattern {
    fn rows(&self) -> &[String] {
        &self.0
    }

    // NOTE can this be made without allocating?
    fn cols(&self) -> Vec<String> {
        let width = self.0.first().expect("at least one").len();
        let mut cols = vec![String::new(); width];
        self.0.iter().for_each(|string| {
            string.char_indices().for_each(|(index, char)| {
                cols[index].push(char);
            })
        });
        cols
    }
}
