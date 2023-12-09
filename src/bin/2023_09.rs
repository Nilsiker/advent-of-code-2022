// NOTE A quick solution finishing in about 1ms on my machine.
// You could probably the prediction sums in the calculate_changes fn (not a very good name)
// and get away with less cloning, making it a bit faster.
// Also, not using a VecDeque and some other storage solution probably speeds it up a lot!
// But I'm happy with this :)

use std::collections::VecDeque;

use advent_of_code::read_input_lines;

fn main() {
    let lines = read_input_lines(2023, 9);

    let start = std::time::Instant::now();

    let value_histories: Vec<ValueHistory> = lines
        .iter()
        .map(|line| {
            let values: VecDeque<isize> = line
                .split(' ')
                .map(|value_str| value_str.parse::<isize>().expect("parseable usize"))
                .collect();
            ValueHistory(values)
        })
        .collect();

    let mut sum = 0;
    let mut backwards_sum = 0;
    for history in value_histories {
        let changes_vec = history.calculate_changes();
        let prediction =
            history.0.back().expect("non-empty") + changes_vec[0].0.back().expect("non-empty");
        let backwards_prediction =
            history.0.front().expect("non-empty") - changes_vec[0].0.front().expect("non-empty");
        sum += prediction;
        backwards_sum += backwards_prediction;
    }
    let elapsed = start.elapsed();
    println!("Forward prediction sum is {sum} and backwards prediction sum is {backwards_sum}");
    println!("{elapsed:?}")
}

#[derive(Debug)]
struct ValueHistory(VecDeque<isize>);
impl ValueHistory {
    fn calculate_changes(&self) -> Vec<ValueHistory> {
        let mut values = self.0.clone();
        let mut all_changes = vec![];
        loop {
            let mut changes = VecDeque::new();
            for i in 0..(values.len() - 1) {
                let (a, b) = (values[i], values[i + 1]);
                changes.push_back(b - a);
            }
            let all_zeroes = changes.iter().all(|el| *el == 0);
            all_changes.push(ValueHistory(changes.clone()));
            if all_zeroes {
                break;
            }
            values = changes;
        }

        for i in (0..all_changes.len() - 1).rev() {
            let changes_len = all_changes[i].0.len() - 1;
            let next_changes_len = all_changes[i + 1].0.len() - 1;
            let prediction = all_changes[i].0[changes_len] + all_changes[i + 1].0[next_changes_len];
            let backwards_prediction = all_changes[i].0[0] - all_changes[i + 1].0[0];

            all_changes[i].0.push_back(prediction);
            all_changes[i].0.push_front(backwards_prediction);
        }
        all_changes
    }
}
