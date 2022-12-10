use std::collections::HashSet;

use advent_of_code_2022::read_input_lines;

fn main() {
    let mut two_knot_rope = Rope::new(2);
    let mut ten_knot_rope = Rope::new(10);
    let lines = read_input_lines(9);

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let dir: Dir = parts[0].into();
        let steps = parts[1].parse::<usize>().unwrap();
        for _ in 0..steps {
            two_knot_rope.move_head(&dir);
            ten_knot_rope.move_head(&dir);
        }
    }

    println!(
        "Tail of two-knot rope visited {} locations.",
        two_knot_rope.tail_visited_locations.len()
    );
    println!(
        "Tail of ten-knot rope visited {} locations.",
        ten_knot_rope.tail_visited_locations.len()
    );
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
    tail_visited_locations: HashSet<Knot>,
}
impl Rope {
    fn new(knots: usize) -> Self {
        let mut set = HashSet::new();
        let knots = vec![Knot::new(0, 0); knots];
        set.insert(Knot { x: 0, y: 0 });
        Self {
            knots,
            tail_visited_locations: set,
        }
    }

    fn head_mut(&mut self) -> &mut Knot {
        self.knots.first_mut().unwrap()
    }

    fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }

    fn move_head(&mut self, dir: &Dir) {
        self.head_mut().translate_with_dir(dir);
        for i in 0..self.knots.len() - 1 {
            let slices = self.knots.split_at_mut(i + 1);

            let lead = slices.0.last().unwrap();
            let next = slices.1.first_mut().unwrap();
            next.adjust_knot(lead);
        }
        self.tail_visited_locations.insert(*self.tail());
    }
}

#[derive(Debug)]
enum Dir {
    None,
    U,
    D,
    L,
    R,
    UL,
    UR,
    DL,
    DR,
}
impl From<&str> for Dir {
    fn from(str: &str) -> Self {
        match str {
            "R" => Dir::R,
            "L" => Dir::L,
            "U" => Dir::U,
            "D" => Dir::D,
            _ => Dir::None,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Default, Debug, Copy, Clone)]
struct Knot {
    x: i32,
    y: i32,
}
impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
    fn translate_with_dir(&mut self, dir: &Dir) {
        match dir {
            Dir::None => (),
            Dir::U => self.translate(0, 1),
            Dir::D => self.translate(0, -1),
            Dir::L => self.translate(-1, 0),
            Dir::R => self.translate(1, 0),
            Dir::UL => {
                self.translate(-1, 1);
            }
            Dir::UR => {
                self.translate(1, 1);
            }
            Dir::DL => {
                self.translate(-1, -1);
            }
            Dir::DR => {
                self.translate(1, -1);
            }
        }
    }

    fn is_adjacent(&self, other: &Knot) -> bool {
        (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2
    }

    fn get_direction_to(&self, other: &Knot) -> Dir {
        let Self { x, y } = self;

        if self.is_adjacent(other) {
            return Dir::None;
        }
        // this is very unreadable...
        match x.cmp(&other.x) {
            std::cmp::Ordering::Less => match y.cmp(&other.y) {
                std::cmp::Ordering::Less => Dir::UR,
                std::cmp::Ordering::Equal => Dir::R,
                std::cmp::Ordering::Greater => Dir::DR,
            },
            std::cmp::Ordering::Equal => match y.cmp(&other.y) {
                std::cmp::Ordering::Greater => Dir::D,
                _ => Dir::U,
            },
            std::cmp::Ordering::Greater => match y.cmp(&other.y) {
                std::cmp::Ordering::Less => Dir::UL,
                std::cmp::Ordering::Equal => Dir::L,
                std::cmp::Ordering::Greater => Dir::DL,
            },
        }
    }

    fn adjust_knot(&mut self, lead: &Knot) {
        let dir = self.get_direction_to(lead);
        self.translate_with_dir(&dir);
    }
}
