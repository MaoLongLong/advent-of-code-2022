use std::collections::HashSet;

fn main() {
    let input = advent_of_code::read_file("inputs", 3);

    let mut sum: i32 = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let set = left.chars().collect::<HashSet<_>>();
        for c in right.chars() {
            if set.contains(&c) {
                sum += match c {
                    'a'..='z' => c as u8 - b'a' + 1,
                    'A'..='Z' => c as u8 - b'A' + 27,
                    _ => panic!("unknown char!!!"),
                } as i32;
                break;
            }
        }
    }

    println!("{sum}");
}
