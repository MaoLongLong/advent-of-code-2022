use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let f = File::open("./input")?;
    let reader = BufReader::new(f);

    let mut sum: i32 = 0;
    for line in reader.lines() {
        let line = line?;
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

    println!("{}", sum);

    Ok(())
}
