use advent_of_code_2022::read_input_lines;

type Stacks = Vec<Vec<char>>;

fn main() {
    let lines = read_input_lines(5);
    let blank_line_index = find_blank_line_index(&lines);
    let crate_lines = &lines[0..blank_line_index].to_vec();
    let instruction_lines = &lines[blank_line_index + 1..].to_vec();

    let stacks = parse_crates(crate_lines);
    let instructions = parse_instructions(instruction_lines);

    perform_move(&stacks, &instructions, false);
    perform_move(&stacks, &instructions, true);
}

fn perform_move(stacks: &Stacks, instructions: &Vec<Instruction>, use_crate_mover_9001: bool) {
    let mut stacks = stacks.clone();
    let instructions = instructions.clone();
    for instruction in instructions {
        let Instruction { quantity, from, to } = instruction;
        let mut picked = stacks.pick(quantity, from);
        if !use_crate_mover_9001 {
            picked.reverse();
        }
        stacks.place(to, picked);
    }

    print!(
        "\nWith CrateMover{}: ",
        if use_crate_mover_9001 { 9001 } else { 9000 }
    );
    for stack in stacks {
        print!(" {:?} ", stack.last().unwrap());
    }
}

fn find_blank_line_index(lines: &Vec<String>) -> usize {
    let mut blank_line_index = lines.len();
    lines.iter().enumerate().for_each(|(index, line)| {
        if line.is_empty() {
            blank_line_index = index;
            return;
        }
    });

    blank_line_index
}

fn parse_crates(crate_data: &Vec<String>) -> Vec<Vec<char>> {
    let (number_line, crate_lines) = crate_data.split_last().unwrap();
    let number_of_stacks = number_line.split_whitespace().count();
    let mut crate_stacks = vec![vec![]; number_of_stacks];
    for line in crate_lines {
        let chars = line.chars().collect::<Vec<char>>();
        for i in (0..chars.len()).step_by(4) {
            let crate_char = &chars[i..i + 2].get(1).unwrap();
            if crate_char != &&' ' {
                crate_stacks[i / 4].push(**crate_char);
            }
        }
    }
    for stack in &mut crate_stacks {
        stack.reverse();
    }
    crate_stacks
}

fn parse_instructions(instruction_lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in instruction_lines {
        let parts = line.split(' ').into_iter().collect::<Vec<&str>>();
        instructions.push(Instruction {
            quantity: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap(),
            to: parts[5].parse::<usize>().unwrap(),
        });
    }

    instructions
}

#[derive(Debug, Clone)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

trait Relocate {
    fn pick(&mut self, quantity: usize, from: usize) -> Vec<char>;
    fn place(&mut self, to: usize, slice: Vec<char>);
}

impl Relocate for Stacks {
    fn pick(&mut self, quantity: usize, from: usize) -> Vec<char> {
        let stack = &mut self[from - 1];
        let len = stack.len();

        if len == quantity {
            stack.drain(0..).as_slice().to_owned()
        } else {
            stack.drain(len - quantity..len).as_slice().to_owned()
        }
    }

    fn place(&mut self, to: usize, crates: Vec<char>) {
        self[to - 1].extend(crates);
    }
}
