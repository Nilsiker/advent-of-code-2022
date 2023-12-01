use advent_of_code::read_input_lines;

fn main() {
    let grid = TreeGrid::new(read_input_lines(2022, 8));
    let width = grid.grid_width;

    let mut visible_counter = 4 * (width - 1);
    let mut best_score = 0;

    for x in 1..width - 1 {
        let col = grid.get_col(x);
        for y in 1..width - 1 {
            let row = grid.get_row(y);

            if visible_in_col(y, &col) || visible_in_row(x, &row) {
                visible_counter += 1;
            }

            best_score = best_score.max(scenic_score_from_coords(
                x,
                y,
                &grid.get_col(x),
                &grid.get_row(y),
            ));
        }
    }

    println!("trees visible from edge: {visible_counter}");
    println!("The most scenic location has a score of {}", best_score);
}

fn scenic_score_from_coords(x: usize, y: usize, col: &[usize], row: &[usize]) -> usize {
    let candidate = col[y];
    let up = &col[0..y];
    let down = &col[y + 1..col.len()];
    let left = &row[0..x];
    let right = &row[x + 1..row.len()];

    let mut up_score = 0;
    for tree in up.iter().rev() {
        up_score += 1;
        if *tree >= candidate {
            break;
        }
    }
    let mut down_score = 0;
    for tree in down {
        down_score += 1;
        if *tree >= candidate {
            break;
        }
    }
    let mut left_score = 0;
    for tree in left.iter().rev() {
        left_score += 1;
        if *tree >= candidate {
            break;
        }
    }
    let mut right_score = 0;
    for tree in right {
        right_score += 1;
        if *tree >= candidate {
            break;
        }
    }

    up_score * down_score * right_score * left_score
}

fn visible_in_col(index: usize, col: &[usize]) -> bool {
    let candidate = col[index];
    let up = &col[0..index];
    let down = &col[index + 1..col.len()];

    let mut max_up = 0;
    for height in up {
        max_up = max_up.max(*height);
    }

    let mut max_down = 0;
    for height in down {
        max_down = max_down.max(*height);
    }

    candidate > max_down || candidate > max_up
}
fn visible_in_row(index: usize, row: &[usize]) -> bool {
    let candidate = row[index];
    let left = &row[0..index];
    let right = &row[index + 1..row.len()];
    let mut max_left = 0;
    for height in left {
        max_left = max_left.max(*height);
    }

    let mut max_right = 0;
    for height in right {
        max_right = max_right.max(*height);
    }

    candidate > max_left || candidate > max_right
}

struct TreeGrid {
    grid_width: usize,
    heights: Vec<usize>,
}
impl TreeGrid {
    fn new(lines: Vec<String>) -> Self {
        let grid_width = lines[0].len();
        let heights: Vec<usize> = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect();
        Self {
            grid_width,
            heights,
        }
    }

    fn get_row(&self, index: usize) -> Vec<usize> {
        let Self {
            grid_width,
            heights,
        } = self;

        let start_index = grid_width * index;
        let end_index = grid_width * (index + 1);
        let range = start_index..end_index;

        heights[range].to_owned()
    }

    fn get_col(&self, index: usize) -> Vec<usize> {
        let Self {
            grid_width,
            heights,
        } = self;

        let mut v = vec![];
        for i in (index..heights.len()).step_by(*grid_width) {
            v.push(heights[i])
        }

        v
    }
}
