use advent_of_code::read_input_lines;

#[derive(Debug)]
struct Game {
    valid: bool,
    id: usize,
    showings: Vec<Showing>,
}
impl From<String> for Game {
    fn from(value: String) -> Self {
        let parts = value.split(':').collect::<Vec<&str>>();
        let game_part = parts[0];
        let showings_part = parts[1];

        let id = game_part.split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .expect("parsable usize from str");
        let showings_strs = showings_part
            .split(';')
            .map(&str::trim)
            .collect::<Vec<&str>>();

        let mut showings = vec![];
        showings_strs.iter().for_each(|showing_str| {
            let mut colors: (usize, usize, usize) = (0, 0, 0);
            for shown_color_part in showing_str.split(',').map(&str::trim) {
                let mut parts = shown_color_part.split(' ');
                let shown_color = (parts.next().unwrap(), parts.next().unwrap());
                let number_part = shown_color.0;
                let color_part = shown_color.1;
                match color_part {
                    "red" => colors.0 = number_part.parse().expect("str parsable to usize"),
                    "green" => colors.1 = number_part.parse().expect("str parsable to usize"),
                    "blue" => colors.2 = number_part.parse().expect("str parsable to usize"),
                    _ => panic!("should not happen ({color_part})"),
                }
            }
            showings.push(Showing::new(colors.0, colors.1, colors.2));
        });

        let valid = check_game_validity(&showings);

        Self {
            valid,
            id,
            showings,
        }
    }
}

#[derive(Debug)]
struct Showing(usize, usize, usize);
impl Showing {
    fn new(r: usize, g: usize, b: usize) -> Self {
        Self(r, g, b)
    }
}

fn main() {
    let lines = read_input_lines(2023, 2);

    let games = lines.into_iter().map(Game::from).collect::<Vec<Game>>();
    let sum_valid_games: usize = games
        .iter()
        .filter(|game| game.valid)
        .map(|game| game.id)
        .sum();

    println!("The sum of the the valid game IDs are {sum_valid_games}");

    let sum_of_powers: usize = games
        .iter()
        .map(|game| {
            let mut min_colors = (usize::MIN, usize::MIN, usize::MIN);
            for showing in &game.showings {
                min_colors.0 = min_colors.0.max(showing.0);
                min_colors.1 = min_colors.1.max(showing.1);
                min_colors.2 = min_colors.2.max(showing.2);
            }
            min_colors.0 * min_colors.1 * min_colors.2
        })
        .sum();

    println!("The sum of the powers are {sum_of_powers}");
}

const BAG_CONTENTS: (usize, usize, usize) = (12, 13, 14);
fn check_game_validity(showings: &Vec<Showing>) -> bool {
    for showing in showings {
        if !(showing.0 <= BAG_CONTENTS.0
            && showing.1 <= BAG_CONTENTS.1
            && showing.2 <= BAG_CONTENTS.2)
        {
            return false;
        }
    }
    true
}
