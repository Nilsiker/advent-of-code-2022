use advent_of_code_2022::read_input_lines;

fn main() {
    let lines = read_input_lines(10);
    let ops: Vec<Op> = lines
        .into_iter()
        .map(|line| Op::from(line.as_str()))
        .collect();

    let mut device = Device::new(ops);
    let mut cycle_count = 0;
    let mut samples = vec![];

    while !device.operations.is_empty() {
        cycle_count += 1;

        if (cycle_count - 20) % 40 == 0 {
            samples.push(cycle_count * device.register);
        }

        device.step();
    }

    println!(
        "\nThe signal strength sum of the cycle samples are: {}\n\nThe screen shows:\n{}\n",
        samples.iter().sum::<i32>(),
        device.get_screen_string()
    );
}

#[derive(Debug)]
struct Device {
    register: i32,
    operations: Vec<Operation>,
    screen: Vec<char>,
}
impl Device {
    fn new(mut ops: Vec<Op>) -> Self {
        let operations: Vec<Operation> =
            ops.into_iter().map(|op| Operation::new(op)).rev().collect();
        Self {
            register: 1,
            operations,
            screen: vec![],
        }
    }

    fn step(&mut self) {
        let char = self.determine_pixel();
        self.screen.push(char);
        if let Some(op) = self.operations.last_mut() {
            if op.step() {
                match op.kind {
                    Op::Noop => (),
                    Op::AddX(value) => self.register += value,
                }
                self.operations.pop();
            }
        }
    }

    fn determine_pixel(&self) -> char {
        let pixel_num_to_be_drawn = self.screen.len() % 40;

        let range = self.register - 1..=self.register + 1;
        if range.contains(&(pixel_num_to_be_drawn as i32)) {
            '#'
        } else {
            '.'
        }
    }

    fn get_screen_string(&self) -> String {
        self.screen
            .chunks(40)
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
#[derive(Debug)]
enum Op {
    Noop,
    AddX(i32),
}
impl From<&str> for Op {
    fn from(str: &str) -> Self {
        if str.starts_with("addx") {
            let parts = str.split(' ').collect::<Vec<&str>>();
            let value = parts[1].parse::<i32>().unwrap();
            Self::AddX(value)
        } else {
            Self::Noop
        }
    }
}
#[derive(Debug)]
struct Operation {
    kind: Op,
    cycles_left: usize,
}
impl Operation {
    fn new(op: Op) -> Self {
        let cycles_left = match &op {
            Op::Noop => 1,
            Op::AddX(_) => 2,
        };
        Self {
            kind: op,
            cycles_left,
        }
    }
    fn step(&mut self) -> bool {
        self.cycles_left -= 1;
        self.cycles_left == 0
    }
}
impl From<Op> for Operation {
    fn from(op: Op) -> Self {
        let cycles_left = match &op {
            Op::Noop => 1,
            Op::AddX(_) => 2,
        };
        Self {
            kind: op,
            cycles_left,
        }
    }
}
