use advent_of_code::read_input_lines;

fn main() {
    let lines = read_input_lines(2023, 11);
    let start = std::time::Instant::now();
    let chart = Chart(
        lines
            .into_iter()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect(),
        0,
        0,
    );

    let (sum_a, sum_b) = chart.get_distance_sums();
    let elapsed = start.elapsed();

    println!("The sum of all minimal paths are {sum_a} with expansion factor 2");
    println!("The sum of all minimal paths are {sum_b} with expansion factor 1 000 000");
    println!("{elapsed:?}");
}

struct Chart(Vec<Vec<Tile>>, usize, usize);
impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Chart {
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.0.get(y).and_then(|row| row.get(x))
    }

    fn get_row_count(&self) -> usize {
        self.0.len()
    }

    fn get_column_count(&self) -> usize {
        self.0[0].len()
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
        // println!("{:?} -> {:?} = ({},{})", &from, &to, &x_traveled.count(), &y_traveled.count());

        let expanded_rows = self.get_empty_row_indices();
        let expanded_cols = self.get_empty_column_indices();

        let expanded_rows_traveled = expanded_rows
            .iter()
            .filter(|i| y_traveled.contains(i))
            .count();
        let expanded_cols_traveled = expanded_cols
            .iter()
            .filter(|i| x_traveled.contains(i))
            .count();

        (
            x_traveled.clone().count()
                + y_traveled.clone().count()
                + (expansion_factor_a * expanded_rows_traveled)
                + (expansion_factor_a * expanded_cols_traveled),
            x_traveled.count()
                + y_traveled.count()
                + (expansion_factor_b * expanded_rows_traveled)
                + (expansion_factor_b * expanded_cols_traveled),
        )
    }

    fn get_distance_sums(&self) -> (usize, usize) {
        let mut coords = vec![];
        for y in 0..self.get_row_count() {
            for x in 0..self.get_column_count() {
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

    fn get_empty_row_indices(&self) -> Vec<usize> {
        let mut indices = vec![];

        for y in 0..self.get_row_count() {
            if self.0[y].iter().all(|tile| matches!(tile, Tile::Space)) {
                indices.push(y);
            }
        }
        indices
    }

    fn get_empty_column_indices(&self) -> Vec<usize> {
        let mut indices = vec![];
        'outer: for x in 0..self.get_column_count() {
            for y in 0..self.get_row_count() {
                if matches!(self.get(x, y), Some(Tile::Galaxy)) {
                    continue 'outer;
                }
            }
            indices.push(x);
        }

        indices
    }
}

#[derive(Debug)]
struct Coord(usize, usize);

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
                Tile::Galaxy => '#',
                Tile::Space => '.',
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
