use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

lazy_static! {
    static ref GAME_RESULT: HashMap<(Shape, Shape), i32> = {
        let mut m = HashMap::new();
        m.insert((Shape::Rock, Shape::Rock), 3);
        m.insert((Shape::Rock, Shape::Paper), 6);
        m.insert((Shape::Rock, Shape::Scissors), 0);
        m.insert((Shape::Paper, Shape::Rock), 0);
        m.insert((Shape::Paper, Shape::Paper), 3);
        m.insert((Shape::Paper, Shape::Scissors), 6);
        m.insert((Shape::Scissors, Shape::Rock), 6);
        m.insert((Shape::Scissors, Shape::Paper), 0);
        m.insert((Shape::Scissors, Shape::Scissors), 3);
        m
    };
}

impl Shape {
    fn score(&self) -> i32 {
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
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("unknown shape!!!"),
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
        let s1 = Shape::from(c1);
        let s2 = Shape::from(c2);
        total_score += GAME_RESULT.get(&(s1, s2)).unwrap() + s2.score();
    }

    println!("{}", total_score);

    Ok(())
}
