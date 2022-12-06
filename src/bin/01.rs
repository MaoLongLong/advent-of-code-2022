use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let input = advent_of_code::read_file("inputs", 1);

    let mut sum = 0;
    let mut heap = BinaryHeap::new();
    let mut it = input.lines().peekable();
    while let Some(line) = it.next() {
        if it.peek().is_none() || line.is_empty() {
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
    println!("answer1: {m1}");
    println!("answer2: {}", m1 + m2 + m3);
}
