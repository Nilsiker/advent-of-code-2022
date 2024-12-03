use advent_of_code::read_input_lines;

fn main() {
    let input = read_input_lines(2024, 2);
    let start = std::time::Instant::now();

    let part_a: usize = input
        .iter()
        .map(Report::from)
        .filter(|report| {
            let steps = report.steps();

            steps.all_safe_ascending() || steps.all_safe_descending()
        })
        .count();

    let part_b = input
        .iter()
        .map(Report::from)
        .map(|report| report.with_gnarly_brute_force())
        .filter(|reports| {
            reports.iter().any(|report| {
                report.steps().all_safe_ascending() || report.steps().all_safe_descending()
            })
        })
        .count();

    println!("Elapsed: {:?}", start.elapsed());
    println!("Part A: {part_a}");
    println!("Part B: {part_b}")
}

struct Report {
    levels: Vec<i32>,
}

impl From<&String> for Report {
    fn from(value: &String) -> Self {
        Self {
            levels: value
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("parseable i32"))
                .collect(),
        }
    }
}

impl Report {
    fn steps(&self) -> Steps {
        let levels = &self.levels;
        let steps: Vec<i32> = levels
            .iter()
            .zip(levels[1..].iter())
            .map(|(a, b)| b - a)
            .collect();
        Steps(steps)
    }

    // NOTE: never got this one to work.
    fn _with_tolerance(&self) -> Report {
        let mut levels = self.levels.clone();
        if let Some(index) = levels
            .iter()
            .enumerate()
            .zip(levels[1..].iter())
            .find(|((_, a), b)| {
                let diff = b.abs_diff(**a);
                !(1..=3).contains(&diff)
            })
            .map(|((index, _), _)| index)
        {
            levels.remove(index);
        }

        Report { levels }
    }

    fn with_gnarly_brute_force(&self) -> Vec<Report> {
        let mut variants = vec![];
        for i in 0..self.levels.len() {
            let mut cloned = self.levels.clone();
            cloned.remove(i);
            variants.push(Report { levels: cloned });
        }

        variants
    }
}

#[derive(Clone, Debug)]
struct Steps(Vec<i32>);
impl Steps {
    fn all_safe_ascending(&self) -> bool {
        self.0.iter().all(|step| (1..=3).contains(step))
    }

    fn all_safe_descending(&self) -> bool {
        self.0.iter().all(|step| (-3..=-1).contains(step))
    }
}
