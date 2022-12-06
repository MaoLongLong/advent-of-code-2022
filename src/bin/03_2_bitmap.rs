/// Inspired by: <https://github.com/antirez/adventofcode2022/blob/653f0e17e59e16117a397e0121385d466eb78f34/day-3/2.c>
fn main() {
    let input = advent_of_code::read_file("inputs", 3);

    let mut sum = 0;
    let mut seen = [0u8; 53];
    for (i, line) in input.lines().enumerate() {
        let elf_id = i % 3;
        for c in line.chars() {
            let p = match c {
                'a'..='z' => c as u8 - b'a' + 1,
                _ => c as u8 - b'A' + 27,
            } as usize;
            seen[p] |= 1 << elf_id;
            if seen[p] == 7 {
                sum += p;
                seen.fill(0);
                break;
            }
        }
    }

    println!("{sum}");
}
