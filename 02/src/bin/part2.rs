use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("unknown shape!!!"),
        }
    }
}

enum Strategy {
    Lose,
    Draw,
    Win,
}

impl Strategy {
    fn get_shape(&self, opponent: Shape) -> Shape {
        match self {
            Strategy::Lose => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Strategy::Draw => opponent,
            Strategy::Win => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Strategy::Lose => 0,
            Strategy::Draw => 3,
            Strategy::Win => 6,
        }
    }
}

impl From<&str> for Strategy {
    fn from(s: &str) -> Self {
        match s {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("unknown strategy!!!"),
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open("./input")?;
    let reader = BufReader::new(f);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line?;
        let (c1, c2) = line.split_once(' ').unwrap();
        let opponent = Shape::from(c1);
        let strategy = Strategy::from(c2);
        total_score += strategy.get_score() + strategy.get_shape(opponent).get_score();
    }

    println!("{total_score}");

    Ok(())
}
