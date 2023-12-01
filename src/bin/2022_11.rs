use advent_of_code::read_input_blocks;

enum Part {
    A,
    B,
}

fn main() {
    do_the_rounds(Part::A);
    do_the_rounds(Part::B);
}

fn do_the_rounds(part: Part) {
    let mut troop = get_monkeys();
    let (rounds, manage_worry) = match part {
        Part::A => (20, false),
        Part::B => (10000, true),
    };

    for _ in 0..rounds {
        troop.take_round(manage_worry);
    }

    let mut monkeys_sorted_by_inspection_count = troop.monkeys;
    monkeys_sorted_by_inspection_count
        .sort_by(|a, b| b.items_inspected.partial_cmp(&a.items_inspected).unwrap());

    let monkey_business = &monkeys_sorted_by_inspection_count[0..2]
        .iter()
        .map(|monkey| monkey.items_inspected)
        .reduce(|accum, item| accum * item)
        .unwrap();

    println!("Monkey business is {}", monkey_business);
}

fn get_monkeys() -> Troop {
    let blocks = read_input_blocks(2022, 11);
    let mut worry_managing_number = 1;
    let monkeys = blocks
        .iter()
        .map(|block| {
            let parts = block.split('\n').collect::<Vec<&str>>();
            let items: Vec<Item> = parts[1]
                .split(':')
                .nth(1)
                .expect("item line surely contains a colon char")
                .split(',')
                .map(|str| str.trim())
                .map(String::from)
                .map(|item| Item(item.parse().unwrap()))
                .collect();

            let op_string = parts[2].split(' ').skip(6).collect::<Vec<&str>>();

            let op = Op {
                operator: op_string[0].into(),
                operand: op_string[1].trim().into(),
            };
            let divisible_by = parts[3]
                .split(' ')
                .last()
                .expect("4 elements")
                .trim()
                .parse::<usize>()
                .expect("parseable usize");

            worry_managing_number *= divisible_by;

            let test = Test {
                divisible_by,
                true_monkey: parts[4]
                    .split(' ')
                    .last()
                    .expect("usize at end element")
                    .trim()
                    .parse::<usize>()
                    .expect("parseable usize"),
                false_monkey: parts[5]
                    .split(' ')
                    .last()
                    .expect("usize at end element")
                    .trim()
                    .parse::<usize>()
                    .expect("parseable usize"),
            };

            Monkey {
                items_inspected: 0,
                items,
                op,
                test,
            }
        })
        .collect();
    Troop {
        monkeys,
        worry_managing_number,
    }
}

/// Because a troop is a group of monkeys!
#[derive(Debug)]
struct Troop {
    monkeys: Vec<Monkey>,
    worry_managing_number: usize,
}
impl Troop {
    fn take_round(&mut self, manage_worry: bool) {
        for from in 0..self.monkeys.len() {
            while self.get_num_of_items(from) > 0 {
                let to = self.test_item(from, 0, manage_worry);
                self.throw_item(0, from, to);
            }
        }
    }

    fn get_num_of_items(&self, monkey: usize) -> usize {
        self.monkeys[monkey].items.len()
    }

    fn test_item(&mut self, monkey: usize, item: usize, manage_worry: bool) -> usize {
        self.monkeys[monkey].test_item(item, self.worry_managing_number, manage_worry)
    }

    fn throw_item(&mut self, item: usize, from: usize, to: usize) {
        let mut monkey = self.monkeys.get_mut(from).unwrap();
        let item = monkey.items.remove(item);
        monkey = self.monkeys.get_mut(to).unwrap();
        monkey.items.push(item);
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items_inspected: usize,
    items: Vec<Item>,
    op: Op,
    test: Test,
}
impl Monkey {
    fn test_item(
        &mut self,
        index: usize,
        worry_managing_number: usize,
        manage_worry: bool,
    ) -> usize {
        self.items_inspected += 1;
        let item = &mut self.items[index];
        let op = &self.op;
        let test = &self.test;

        let new_worry = match op.operator {
            Operator::Add => match &op.operand {
                Operand::Item => item.0 + item.0,
                Operand::Value(v) => item.0 + v,
            },
            Operator::Multiply => match &op.operand {
                Operand::Item => item.0 * item.0,
                Operand::Value(v) => item.0 * v,
            },
        };

        if manage_worry {
            item.0 = new_worry % worry_managing_number;
        } else {
            item.0 = new_worry / 3;
        }

        if item.0 % self.test.divisible_by == 0 {
            test.true_monkey
        } else {
            test.false_monkey
        }
    }
}

#[derive(Debug, Clone)]
struct Op {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl From<&str> for Operator {
    fn from(str: &str) -> Self {
        match str {
            "+" => Operator::Add,
            _ => Operator::Multiply,
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Item,
    Value(usize),
}

impl From<&str> for Operand {
    fn from(str: &str) -> Self {
        match str {
            "old" => Operand::Item,
            _ => Operand::Value(str.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisible_by: usize,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone)]
struct Item(usize);
