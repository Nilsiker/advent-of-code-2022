use advent_of_code::read_input_string;
use regex::Regex;

/**
* NOTE: Solved using regex - I feel a bit cheaty.
* Then again, I need to get better at regex anyways.
* You could do this with some sort of stateful parser.
* I imagine that would be much more performant.
*/

fn main() {
    let start = std::time::Instant::now();
    let input = read_input_string(2024, 3);

    let ops = from(input);

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut enabled = true;

    ops.iter().map(Op::eval).for_each(|eval| match eval {
        Evaluation::Product(product) => {
            if enabled {
                sum_b += product
            }
            sum_a += product;
        }
        Evaluation::Enable => enabled = true,
        Evaluation::Disable => enabled = false,
    });

    println!("Elapsed: {:#?}", start.elapsed());
    println!("Part A: {sum_a}");
    println!("Part B: {sum_b}");
}

enum Op {
    Mul(usize, usize),
    Enable,
    Disable,
}
impl Op {
    fn eval(&self) -> Evaluation {
        match self {
            Op::Mul(x, y) => Evaluation::Product(x * y),
            Op::Enable => Evaluation::Enable,
            Op::Disable => Evaluation::Disable,
        }
    }
}

enum Evaluation {
    Product(usize),
    Enable,
    Disable,
}

fn from(string: String) -> Vec<Op> {
    let pattern = r"do\(\)|don't\(\)|mul\((\d*),(\d*)\)";
    let regex = Regex::new(pattern).expect("ok pattern");

    regex
        .find_iter(&string)
        .map(|m| {
            let str = m.as_str();
            if str.starts_with("mul") {
                // caution, hardheaded index finding below
                let x_index = str.find('(').expect("left paren") + 1;
                let comma_index = str.find(',').expect("comma");
                let y_index = comma_index + 1;
                let end_paren = str.find(")").expect("close paren");

                let x = m.as_str()[x_index..comma_index]
                    .parse()
                    .expect("parseable x");
                let y = m.as_str()[y_index..end_paren].parse().expect("parseable y");
                Op::Mul(x, y)
            } else if m.as_str().eq("do()") {
                Op::Enable
            } else {
                Op::Disable
            }
        })
        .collect()
}
