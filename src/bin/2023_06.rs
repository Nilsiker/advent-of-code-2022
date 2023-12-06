use advent_of_code::read_input_lines;

fn main() {
    let start = std::time::Instant::now();

    let lines = read_input_lines(2023, 6);
    let mut iter = lines.iter();
    let mut big_time_string = String::new();
    let mut big_record_string = String::new();

    let times = iter
        .next()
        .expect("has time line")
        .split(' ')
        .filter(|part| !part.is_empty())
        .skip(1);
    let records = iter
        .next()
        .expect("has record line")
        .split(' ')
        .filter(|part| !part.is_empty())
        .skip(1);

    let races = times
        .zip(records)
        .map(|(time_str, record_str)| {
            big_time_string += time_str;
            big_record_string += record_str;
            Race {
                time: time_str.parse().expect("usize parseable"),
                record: record_str.parse().expect("usize parseable"),
            }
        })
        .collect::<Vec<Race>>();

    let big_race = Race {
        time: big_time_string.parse().expect("parseable usize"),
        record: big_record_string.parse().expect("parseable usize"),
    };

    let product: usize = races
        .iter()
        .map(|race| race.find_ways_to_win_num())
        .product();
    let big_product: usize = big_race.find_ways_to_win_num();

    let elapsed = start.elapsed();
    println!("Product of all races is {product:#?}");
    println!("Ways to win big race is {big_product:#?}");
    println!("{elapsed:?}")
}

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}
impl Race {
    fn find_ways_to_win_num(&self) -> usize {
        let time = self.time as f32;
        let record = self.record as f32;

        let x1 = time / 2.0 - ((time / 2.0).powf(2.0) - (record + 1.0)).powf(0.5);
        let x2 = time / 2.0 + ((time / 2.0).powf(2.0) - (record + 1.0)).powf(0.5);
        ((x1.ceil() as usize)..=(x2.floor() as usize)).count()
    }
}
