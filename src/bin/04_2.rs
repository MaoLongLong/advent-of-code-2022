use std::mem;

fn split_pairs(line: &str) -> ((i32, i32), (i32, i32)) {
    let (p1, p2) = line.split_once(',').unwrap();
    let (x1, y1) = p1.split_once('-').unwrap();
    let (x2, y2) = p2.split_once('-').unwrap();
    (
        (x1.parse().unwrap(), y1.parse().unwrap()),
        (x2.parse().unwrap(), y2.parse().unwrap()),
    )
}

fn main() {
    let input = advent_of_code::read_file("inputs", 4);

    let answer = input
        .lines()
        .filter(|line| {
            let (mut p1, mut p2) = split_pairs(line);
            if p2.0 < p1.0 {
                mem::swap(&mut p1, &mut p2);
            }
            p2.0 <= p1.1
        })
        .count();

    println!("{answer}");
}
