use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let f = File::open("./input")?;
    let reader = BufReader::new(f);
    let mut sum = 0;
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            heap.push(Reverse(sum));
            if heap.len() > 3 {
                heap.pop();
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    let m3 = heap.pop().unwrap().0;
    let m2 = heap.pop().unwrap().0;
    let m1 = heap.pop().unwrap().0;
    println!("answer1: {}", m1);
    println!("answer2: {}", m1 + m2 + m3);
    Ok(())
}
