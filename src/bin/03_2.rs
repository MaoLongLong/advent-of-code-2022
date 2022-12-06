#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let input = advent_of_code::read_file("inputs", 3);

    let rucksacks = input
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>());

    let sum: i32 = rucksacks
        .array_chunks()
        .map(|[x, y, z]| {
            let c = x
                .intersection(&y)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&z)
                .copied()
                .next()
                .unwrap();
            (match c {
                'a'..='z' => c as u8 - b'a' + 1,
                'A'..='Z' => c as u8 - b'A' + 27,
                _ => panic!("unknown char!!!"),
            } as i32)
        })
        .sum();

    println!("{sum}");
}
