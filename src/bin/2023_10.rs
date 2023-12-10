// NOTE Oh boy, I struggled with this one.
// Even parsing the and determining the start pipe type was a hassle, but not too hard.
// Part 2 however, I hadn't exactly encountered a similar issue before.
// People were talking about expanding the grid, to account for "squeezes", but I didn't like the idea of that.
// u/Boojum over at reddit pointed out some great axioms, which helped me get part 2 right in the end

// Furthermore, I've seen people solving this with the shoelace formula and Pick's theorem.

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use advent_of_code::read_input_lines;

const THRESHOLD_PIPES: [Tile; 3] = [Tile::SouthEast, Tile::SouthWest, Tile::NorthSouth];

fn main() {
    let start_time = std::time::Instant::now();

    let (start, mut tiles) = parse_map();
    determine_start_pipe(&start, &mut tiles);
    let points = bfs(start, &tiles);
    let enclosed_tiles = get_enclosed_tile_count(&points, &tiles);

    let elapsed = start_time.elapsed();

    println!("{tiles}");
    println!("Furthest point is {} steps away.", points.len() / 2,);
    println!("Enclosed tiles amount to {}", enclosed_tiles);
    println!("{elapsed:?}");
}

fn bfs(start: Coord, pipes: &TileMap) -> Vec<Coord> {
    let mut visit = VecDeque::new();
    let mut visited = HashSet::new();

    visit.push_front(start);

    while !visit.is_empty() {
        let current = visit.pop_front().expect("existing coord");

        let current_pipe = pipes.get(current).expect("some pipe");
        let edges = current_pipe.get_edges(current);

        // println!("{current_pipe:?} ({current:?}): {edges:#?}");
        for coord in edges {
            if !visited.contains(&coord) {
                visit.push_back(coord);
            }
        }

        visited.insert(current);
    }

    visited.into_iter().collect::<Vec<Coord>>()
}

fn get_enclosed_tile_count(path: &[Coord], map: &TileMap) -> usize {
    let mut inside = false;
    let mut contained: usize = 0;

    for y in 0..map.0.len() {
        for x in 0..map.0[0].len() {
            let coord = Coord(x as isize, y as isize);
            let tile = map.get(coord).unwrap();

            if path.contains(&coord) && THRESHOLD_PIPES.contains(&tile) {
                inside = !inside;
            }

            if !path.contains(&coord) && inside {
                contained += 1;
            }
        }
    }

    contained
}

struct TileMap(Vec<Vec<Tile>>);
impl TileMap {
    fn get(&self, coord: Coord) -> Option<Tile> {
        let tile = self
            .0
            .get(coord.1 as usize)
            .and_then(|y| y.get(coord.0 as usize));

        tile.cloned()
    }
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        self.0
            .get_mut(coord.1 as usize)
            .and_then(|y| y.get_mut(coord.0 as usize))
    }
}
impl Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for tile in line {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn determine_start_pipe(start: &Coord, map: &mut TileMap) {
    let north_coord = Coord(start.0, start.1 - 1);
    let south_coord = Coord(start.0, start.1 + 1);
    let west_coord = Coord(start.0 - 1, start.1);
    let east_coord = Coord(start.0 + 1, start.1);
    let north = map.get(north_coord);
    let south = map.get(south_coord);
    let west = map.get(west_coord);
    let east = map.get(east_coord);

    let valid_north = if let Some(north) = north {
        north.can_connect(Direction::South)
    } else {
        false
    };
    let valid_south = if let Some(south) = south {
        south.can_connect(Direction::North)
    } else {
        false
    };
    let valid_west = if let Some(west) = west {
        west.can_connect(Direction::East)
    } else {
        false
    };
    let valid_east = if let Some(east) = east {
        east.can_connect(Direction::West)
    } else {
        false
    };

    let start_pipe = match (valid_north, valid_south, valid_west, valid_east) {
        (true, true, false, false) => Tile::NorthSouth,
        (true, false, true, false) => Tile::NorthWest,
        (true, false, false, true) => Tile::NorthEast,
        (false, true, true, false) => Tile::SouthWest,
        (false, true, false, true) => Tile::SouthEast,
        (false, false, true, true) => Tile::WestEast,
        _ => panic!("start node must connect to exactly two pipes!"),
    };
    if let Some(tile) = map.get_mut(*start) {
        *tile = start_pipe;
    }
}

fn parse_map() -> (Coord, TileMap) {
    let mut start: Option<Coord> = None;
    let mut map = vec![];
    read_input_lines(2023, 10)
        .into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            let mut tiles = vec![];
            let chars = line.chars();
            for (x, ch) in chars.enumerate() {
                let tile = match ch {
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    '-' => Tile::WestEast,
                    '|' => Tile::NorthSouth,
                    '7' => Tile::SouthWest,
                    'F' => Tile::SouthEast,
                    'S' => {
                        start = Some(Coord(x as isize, y as isize));
                        Tile::Start
                    }
                    _ => Tile::Ground,
                };
                tiles.push(tile);
            }
            map.push(tiles);
        });

    (start.expect("must find start coord"), TileMap(map))
}

enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Start,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    WestEast,
    NorthSouth,
    Ground,
}
impl Tile {
    fn can_connect(&self, direction: Direction) -> bool {
        match self {
            Tile::NorthEast => match direction {
                Direction::North => true,
                Direction::South => false,
                Direction::West => false,
                Direction::East => true,
            },
            Tile::NorthWest => match direction {
                Direction::North => true,
                Direction::South => false,
                Direction::West => true,
                Direction::East => false,
            },
            Tile::SouthEast => match direction {
                Direction::North => false,
                Direction::South => true,
                Direction::East => true,
                Direction::West => false,
            },
            Tile::SouthWest => match direction {
                Direction::North => false,
                Direction::South => true,
                Direction::East => false,
                Direction::West => true,
            },
            Tile::WestEast => match direction {
                Direction::North => false,
                Direction::South => false,
                Direction::East => true,
                Direction::West => true,
            },
            Tile::NorthSouth => match direction {
                Direction::North => true,
                Direction::South => true,
                Direction::East => false,
                Direction::West => false,
            },
            _ => match direction {
                Direction::North => false,
                Direction::South => false,
                Direction::West => false,
                Direction::East => false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coord(isize, isize);
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.1.cmp(&other.1) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Tile {
    fn get_edges(&self, coord: Coord) -> Vec<Coord> {
        let Coord(x, y) = coord;
        match self {
            Tile::NorthEast => vec![Coord(x + 1, y), Coord(x, y - 1)],
            Tile::NorthWest => vec![Coord(x - 1, y), Coord(x, y - 1)],
            Tile::SouthEast => vec![Coord(x + 1, y), Coord(x, y + 1)],
            Tile::SouthWest => vec![Coord(x - 1, y), Coord(x, y + 1)],
            Tile::WestEast => vec![Coord(x - 1, y), Coord(x + 1, y)],
            Tile::NorthSouth => vec![Coord(x, y + 1), Coord(x, y - 1)],
            _ => panic!("determine start pipe before calling get_edges!"),
        }
    }
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Start => 'S',
            Tile::NorthEast => '╚',
            Tile::NorthWest => '╝',
            Tile::SouthEast => '╔',
            Tile::SouthWest => '╗',
            Tile::WestEast => '═',
            Tile::NorthSouth => '║',
            Tile::Ground => '.',
        };
        write!(f, "{c}")
    }
}
