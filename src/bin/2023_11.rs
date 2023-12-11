// NOTE I am very happy with this one.
//
// The key here was to not actually expand the star chart,
// but keep track of empty rows and columns, and count each step into these rows/cols
// as expansion_factor number of steps in worth.
//
// This way, the compute time is constant no matter how large the expansion factor gets.
//
// I save the empty row and col indices, optimizing for speed rather than memory.
//
// This finishes under 5ms on my ThinkPad T480, inckuding parsing.

use std::collections::HashSet;

use advent_of_code::read_input_lines;

fn main() {
    let start = std::time::Instant::now();
    let lines = read_input_lines(2023, 11);

    let height = lines.len();
    let width = lines[0].len();
    let mut data = vec![];
    let mut galaxy_xs: HashSet<usize> = HashSet::new();
    let mut galaxy_ys: HashSet<usize> = HashSet::new();

    lines.join("").chars().enumerate().for_each(|(x, c)| {
        let tile = Tile::from(c);
        if matches!(tile, Tile::Galaxy) {
            galaxy_xs.insert(x % width);
            galaxy_ys.insert(x / width);
        }
        data.push(tile)
    });

    let empty_rows = (0..height).filter(|y| !galaxy_ys.contains(y)).collect();

    let empty_cols = (0..width).filter(|x| !galaxy_xs.contains(x)).collect();

    let chart = Chart {
        data,
        empty_rows,
        empty_cols,
        width,
        height,
    };
    let (sum_a, sum_b) = chart.get_distance_sums();
    let elapsed = start.elapsed();

    println!("{chart}");
    println!("The sum of all minimal paths are {sum_a} with expansion factor 2");
    println!("The sum of all minimal paths are {sum_b} with expansion factor 1 000 000");
    println!("{elapsed:?}");
}

struct Chart {
    data: Vec<Tile>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    width: usize,
    height: usize,
}
impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.len() {
            if i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.data[i])?;
        }
        Ok(())
    }
}
impl Chart {
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.data.get(y * self.width + x)
    }

    fn measure_distance(
        &self,
        from: &Coord,
        to: &Coord,
        (expansion_factor_a, expansion_factor_b): (usize, usize),
    ) -> (usize, usize) {
        let x_traveled = if from.0 > to.0 {
            to.0..from.0
        } else {
            from.0..to.0
        };
        let y_traveled = if from.1 > to.1 {
            to.1..from.1
        } else {
            from.1..to.1
        };

        let expanded_rows_traveled = &self.empty_rows
            .iter()
            .filter(|i| y_traveled.contains(i))
            .count();
        let expanded_cols_traveled = &self.empty_cols
            .iter()
            .filter(|i| x_traveled.contains(i))
            .count();

        let regular_steps = x_traveled.count() + y_traveled.count();
        let expanded_steps_a = (expansion_factor_a * expanded_rows_traveled)
            + (expansion_factor_a * expanded_cols_traveled);
        let expanded_steps_b = (expansion_factor_b * expanded_rows_traveled)
            + (expansion_factor_b * expanded_cols_traveled);

        (
            regular_steps + expanded_steps_a,
            regular_steps + expanded_steps_b,
        )
    }

    fn get_distance_sums(&self) -> (usize, usize) {
        let mut coords = vec![];
        for y in 0..self.width {
            for x in 0..self.height {
                if matches!(self.get(x, y), Some(Tile::Galaxy)) {
                    coords.push(Coord(x, y));
                }
            }
        }

        let mut distances_a = vec![];
        let mut distances_b = vec![];

        for i in 0..coords.len() {
            let this = &coords[i];
            let coords_to_pair = &coords[i + 1..coords.len()];
            for other in coords_to_pair {
                // let distance = this.distance_to(other);
                let (distance_a, distance_b) = self.measure_distance(this, other, (1, 9999999));
                // println!("{this:?} -> {other:?} = {distance}");

                distances_a.push(distance_a);
                distances_b.push(distance_b);
            }
        }

        (distances_a.iter().sum(), distances_b.iter().sum())
    }
}

#[derive(Debug)]
struct Coord(usize, usize);

#[derive(Debug)]
enum Tile {
    Space,
    Galaxy,
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Tile::Galaxy => '✦',
                Tile::Space => ' ',
            }
        )
    }
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Galaxy,
            _ => Tile::Space,
        }
    }
}
