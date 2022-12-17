use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use advent_of_code_2022::read_input_lines;

fn main() {
    let data = read_input_lines(12);
    let MapData {
        grid: map,
        width,
        start,
        end,
    } = get_map(data);

    let steps = dijkstra(
        &map,
        width,
        start,
        |x, y, nx, ny| map[ny * width + nx] <= map[y * width + x].saturating_add(1),
        |x, y| end.0 == x && end.1 == y,
    );
    
    match steps {
        None => println!("The highest peak can't be reached."),
        Some(steps) => println!("The highest peak can be reached in {steps} steps."),
    };

    // part 2
    let steps = dijkstra(
        &map,
        width,
        end,
        |x, y, nx, ny| map[ny * width + nx] >= map[y * width + x].saturating_sub(1),
        |x, y| map[y * width + x] == 'a' as usize,
    );
    
    match steps {
        None => println!("The highest peak can't be reached."),
        Some(steps) => println!("From closest 'a' elevation, the highest peak can be reached in {steps} steps."),
    };
}

fn dijkstra<F, S>(
    map: &Vec<usize>,
    width: usize,
    start: (usize, usize),
    step_condition: S,
    done_condition: F,
) -> Option<usize>
where
    F: Fn(usize, usize) -> bool,
    S: Fn(usize, usize, usize, usize) -> bool,
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, start));
    visited.insert(start);

    while let Some((dist, (x, y))) = queue.pop_front() {
        for (nx, ny) in get_cardinals(
            (x as isize, y as isize),
            width as isize,
            map.len() as isize / width as isize,
        ) {
            if step_condition(x, y, nx, ny) && visited.insert((nx, ny)) {
                if done_condition(nx, ny) {
                    return Some(dist + 1);
                }
                queue.push_back((dist + 1, (nx, ny)))
            }
        }
    }

    None
}

fn get_map(data: Vec<String>) -> MapData {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let width = data[0].len();
    let mut nested_grid = vec![];

    for (y, line) in data.iter().enumerate() {
        nested_grid.push(
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    'S' => {
                        start = (x, y);
                        'a' as usize
                    }
                    'E' => {
                        end = (x, y);
                        'z' as usize
                    }
                    _ => char as usize,
                })
                .collect::<Vec<usize>>(),
        );
    }
    let grid = nested_grid.into_iter().flatten().collect::<Vec<usize>>();

    MapData {
        grid,
        width,
        start,
        end,
    }
}

fn get_cardinals((x, y): (isize, isize), x_bound: isize, y_bound: isize) -> Vec<(usize, usize)> {
    let mut cardinals = vec![];
    for (nx, ny) in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
        if nx >= 0 && ny >= 0 && nx < x_bound && ny < y_bound {
            cardinals.push((nx as usize, ny as usize))
        }
    }
    cardinals
}

#[derive(Debug)]
struct MapData {
    grid: Vec<usize>,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}
