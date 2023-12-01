use advent_of_code::read_input_lines;

fn main() {
    let lines = read_input_lines(2022, 2);
    let mut score = 0;
    let mut correct_score = 0;

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();

        // First strat understanding
        let my_shape = Shape::from_abc_xyz(parts[1]);
        let opponent_shape = Shape::from_abc_xyz(parts[0]);
        let outcome = my_shape.against(&opponent_shape);

        score += u32::from(my_shape);
        score += u32::from(outcome);

        // Second strat understanding
        let wanted_outcome: Outcome = parts[1].into();
        let my_correct_shape =
            Shape::from_opponent_move_and_outcome(&opponent_shape, &wanted_outcome);

        correct_score += u32::from(my_correct_shape);
        correct_score += u32::from(wanted_outcome);
    }
    println!("If XYZ stood for the opponent's move, my score would be: {score}");

    println!("Now that I know IT DOESN'T, my score would be: {correct_score}");
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}
impl Shape {
    fn from_abc_xyz(string: &str) -> Self {
        match string {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Wait, this strategy guide sucks!"),
        }
    }

    fn from_opponent_move_and_outcome(opponent_move: &Shape, wanted_outcome: &Outcome) -> Self {
        match opponent_move {
            Self::Rock => match wanted_outcome {
                Outcome::Loss => Self::Scissors,
                Outcome::Draw => Self::Rock,
                Outcome::Win => Self::Paper,
            },
            Self::Paper => match wanted_outcome {
                Outcome::Loss => Self::Rock,
                Outcome::Draw => Self::Paper,
                Outcome::Win => Self::Scissors,
            },
            Self::Scissors => match wanted_outcome {
                Outcome::Loss => Self::Paper,
                Outcome::Draw => Self::Scissors,
                Outcome::Win => Self::Rock,
            },
        }
    }
}

impl Shape {
    fn against(&self, shape: &Shape) -> Outcome {
        match self {
            Shape::Rock => match shape {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match shape {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
            Shape::Scissors => match shape {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }
}
impl From<Shape> for u32 {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}
impl From<&str> for Outcome {
    fn from(string: &str) -> Self {
        match string {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Yep, this strategy guide is busted :("),
        }
    }
}
impl From<Outcome> for u32 {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}
