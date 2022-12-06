enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn game_result(s1: &Shape, s2: &Shape) -> i32 {
        match (s1, s2) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
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

fn main() {
    let input = advent_of_code::read_file("inputs", 2);

    let mut total_score = 0;
    for line in input.lines() {
        let (c1, c2) = line.split_once(' ').unwrap();
        let s1 = Shape::from(c1);
        let s2 = Shape::from(c2);
        total_score += Shape::game_result(&s1, &s2) + s2.score();
    }

    println!("{total_score}");
}
