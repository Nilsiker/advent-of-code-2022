use advent_of_code::read_input_lines;

fn main() {
    let input = read_input_lines(2024, 1);

    let start = std::time::Instant::now();
    let rows: Vec<Row> = input.into_iter().map(Row::from).collect();
    let mut rows_left_sorted = rows.clone();
    let mut rows_right_sorted = rows.clone();
    rows_left_sorted.sort_by(|a, b| a.left.cmp(&b.left));
    rows_right_sorted.sort_by(|a, b| a.right.cmp(&b.right));

    let mut similarity_score_sum: usize = 0;
    let difference_sum: usize = rows_left_sorted
        .iter()
        .zip(rows_right_sorted)
        .map(|(a, b)| {
            let count = rows.iter().filter(|row| a.left == row.right).count();
            let sim_score = count * a.left;
            similarity_score_sum += sim_score;
            a.left.abs_diff(b.right)
        })
        .sum();

    let elapsed = start.elapsed();

    println!("Part A: {difference_sum}");
    println!("Part B: {similarity_score_sum}");
    println!("Finished in {elapsed:?}");
}

#[derive(Clone, Debug)]
struct Row {
    left: usize,
    right: usize,
}
impl From<String> for Row {
    fn from(value: String) -> Self {
        let left_right: Vec<usize> = value
            .split_ascii_whitespace()
            .map(|s| s.parse().expect("parsable usize"))
            .collect();
        Self {
            left: left_right[0],
            right: left_right[1],
        }
    }
}
