// NOTE wildly inefficient solution to part B. 
// Probably, you could optimize the maps and ranges, to short-circuit/minimize a lot of the checks that are done.
// Might revisit this! :)

use std::ops::Range;

use advent_of_code::read_input_blocks;

fn main() {
    let start = std::time::Instant::now();
    let blocks = read_input_blocks(2023, 5);

    let mut iter = blocks.iter();

    let seeds_string = iter.next().expect("seeds section exists");

    let seeds: Vec<Seed> = seeds_string.split(' ').skip(1).map(Seed::from).collect();

    let seeds_ranges: Vec<SeedRange> = seeds_string
        .split(' ')
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| chunk.join(" "))
        .map(SeedRange::from)
        .collect();

    let pipeline = MapPipeline {
        maps: vec![
            build_map(iter.next().expect("soil map")),
            build_map(iter.next().expect("fertilizer map")),
            build_map(iter.next().expect("water map")),
            build_map(iter.next().expect("light map")),
            build_map(iter.next().expect("temp map")),
            build_map(iter.next().expect("humidity map")),
            build_map(iter.next().expect("location map")),
        ],
    };

    let min_location = seeds
        .iter()
        .map(|seed| pipeline.get_min_location(seed.0))
        .min()
        .expect("at least one location");

    let min_location_in_ranges = seeds_ranges
        .iter()
        .map(|range| pipeline.get_min_location_range(range))
        .min()
        .expect("some min");

    let elapsed = start.elapsed();
    println!("The minimum location would be {min_location}");
    println!("With ranges in mind, the minimum location is {min_location_in_ranges}");
    println!("{elapsed:?}");
}

fn build_map(block: &str) -> Map {
    let mut vec = vec![];
    block.split('\n').skip(1).for_each(|entry_line| {
        let mut parts = entry_line.split(' ').map(|part| part.trim());
        let destination_start: usize = parts
            .next()
            .expect("source start")
            .parse()
            .expect("parseable to usize");
        let source_start: usize = parts
            .next()
            .expect("source start")
            .parse()
            .expect("parseable to usize");
        let range: usize = parts
            .next()
            .expect("source start")
            .parse()
            .expect("parseable to usize");
        vec.push(MapEntry {
            source_start,
            destination_start,
            range,
        });
    });
    Map { entries: vec }
}

#[derive(Debug)]
struct Seed(usize);
impl From<&str> for Seed {
    fn from(value: &str) -> Self {
        Self(value.parse().expect("parseable to usize"))
    }
}

#[derive(Debug)]
struct SeedRange(Range<usize>);
impl From<String> for SeedRange {
    fn from(value: String) -> Self {
        let mut parts = value.split(' ');
        let start = parts
            .next()
            .expect("start")
            .parse::<usize>()
            .expect("parseable usize");
        let range = parts
            .next()
            .expect("range")
            .parse::<usize>()
            .expect("parseable usize");
        Self(start..start + range)
    }
}

struct MapPipeline {
    maps: Vec<Map>,
}
impl MapPipeline {
    fn get_min_location(&self, seed: usize) -> usize {
        let mut outputs: Vec<usize> = vec![seed];
        for map in &self.maps {
            outputs = outputs.iter().map(|output| map.output(*output)).collect();
        }
        *outputs.iter().min().expect("has at least one value")
    }

    fn get_min_location_range(&self, range: &SeedRange) -> usize {
        range
            .0
            .clone()
            .map(|seed| self.get_min_location(seed))
            .min()
            .expect("some min value")
    }
}

struct Map {
    entries: Vec<MapEntry>,
}
impl Map {
    fn output(&self, input: usize) -> usize {
        for entry in &self.entries {
            if let Some(output) = entry.get_output(input) {
                return output;
            }
        }

        input
    }
}

struct MapEntry {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl MapEntry {
    fn source_range(&self) -> Range<usize> {
        self.source_start..self.source_start + self.range
    }

    fn get_output(&self, input: usize) -> Option<usize> {
        if self.source_range().contains(&input) {
            let mapped = self.destination_start + input - self.source_start;
            Some(mapped)
        } else {
            None
        }
    }
}
