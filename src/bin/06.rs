const PACKET_LEN: usize = 4; // part2: 14

fn main() {
    let input = advent_of_code::read_file("inputs", 6);
    let mut freq = [0; 256];
    let mut distinct = 0;
    let a: Vec<char> = input.chars().collect();
    for i in 0..a.len() {
        if i >= PACKET_LEN {
            let j = a[i - PACKET_LEN] as usize;
            freq[j] -= 1;
            if freq[j] == 0 {
                distinct -= 1;
            }
        }
        let j = a[i] as usize;
        freq[j] += 1;
        if freq[j] == 1 {
            distinct += 1;
        }
        if distinct == PACKET_LEN {
            println!("{}", i + 1);
            return;
        }
    }
    unreachable!();
}
