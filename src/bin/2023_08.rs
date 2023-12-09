// NOTE a weird one! Given the puzzle input, I noticed the end node is the end of the "loop".
// I took this as a general rule, making it a least common multiple problem.
// However, if there were multiple end nodes, you'd have to actually simulate this
// and I'm not even sure that is feasible to brute force!

use std::collections::HashMap;

use advent_of_code::read_input_lines;

fn main() {
    let start = std::time::Instant::now();
    let mut lines = read_input_lines(2023, 8)
        .into_iter()
        .filter(|line| !line.is_empty());
    let sequence: Vec<char> = lines
        .next()
        .expect("some algorith/sequence")
        .chars()
        .collect();

    let mut network = HashMap::new();
    for line in lines {
        let mut line = line.replace('(', "");
        line = line.replace(')', "");
        line = line.replace(',', "");
        line = line.replace('=', "");
        let mut parts = line.split(' ').filter(|part| !part.is_empty());
        network.insert(
            parts.next().expect("label").to_string(),
            (
                parts.next().expect("left").to_string(),
                parts.next().expect("right").to_string(),
            ),
        );
    }
    let steps = do_algorithm(&sequence, &network);
    let ghost_steps = do_ghost_algorithm(&sequence, &network);
    let elapsed = start.elapsed();
    println!("Camel: {steps}");
    println!("Ghosts: {ghost_steps}");
    println!("{elapsed:?}")
}

fn do_algorithm(sequence: &Vec<char>, map: &HashMap<String, (String, String)>) -> usize {
    let mut steps = 0;
    let mut node = "AAA";
    loop {
        for instruction in sequence {
            steps += 1;
            node = match instruction {
                'L' => &map.get(node).expect("exists").0,
                _ => &map.get(node).expect("exists").1,
            };
            if node == "ZZZ" {
                return steps;
            }
        }
    }
}

fn do_ghost_algorithm(sequence: &Vec<char>, map: &HashMap<String, (String, String)>) -> usize {
    let nodes: Vec<&String> = map.keys().filter(|key| key.ends_with('A')).collect();
    let steps_vec: Vec<usize> = nodes
        .iter()
        .map(|node| {
            let mut n = *node;
            let mut steps = 0;
            loop {
                for instruction in sequence {
                    n = match instruction {
                        'L' => &map.get(n).expect("exists").0,
                        _ => &map.get(n).expect("exists").1,
                    };
                    steps += 1;
                    if n.ends_with('Z') {
                        return steps;
                    }
                }
            }
        })
        .collect();

    let mut multiple: usize = steps_vec[0];
    for steps in steps_vec.iter().skip(1) {
        multiple = lcm(multiple, *steps)
    }
    multiple
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
