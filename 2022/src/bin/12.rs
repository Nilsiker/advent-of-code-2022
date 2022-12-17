use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    time::SystemTime,
    usize,
};

use advent_of_code_2022::read_input_lines;

fn main() {
    let now = SystemTime::now();
    let data = read_input_lines(12);
    let MapData { map, start, end } = get_map(data);

    let steps = dijkstra(&map, start, |x, y| end.0 == x && end.1 == y);
    println!(
        "The highest peak {}",
        match steps {
            None => format!("can't be reached."),
            Some(steps) => format!("can be reached in {steps} steps."),
        }
    );
    println!("{:?}", now.elapsed().unwrap());
}

fn dijkstra<F>(map: &Vec<Vec<usize>>, start: (usize, usize), done_condition: F) -> Option<usize>
where
    F: Fn(usize, usize) -> bool,
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, start));
    visited.insert(start);

    let mut result = None;

    while let Some((dist, (x, y))) = queue.pop_front() {
        println!("on ({x},{y})={} - current total cost {dist}", char::from_u32(map[y][x] as u32).unwrap());
        for (nx, ny) in get_cardinals(
            (x as isize, y as isize),
            map[0].len() as isize,
            map.len() as isize,
        ) {
            if map[ny][nx] <= map[y][x].saturating_add(1) && visited.insert((nx, ny)) {
                if done_condition(nx, ny) && result.unwrap_or(usize::MAX) > dist+1 {
                    result = Some(dist + 1);
                }
                let d = if map[ny][nx] == 'a' as usize {0} else {dist+1};
                queue.push_back((d, (nx, ny)))
            }
        }
    }

    result
}

fn get_map(data: Vec<String>) -> MapData {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in data.iter().enumerate() {
        grid.push(
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
                .collect(),
        );
    }

    MapData {
        map: grid,
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
    map: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
}
impl MapData {
    fn get(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }
}
