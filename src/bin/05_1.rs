#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

impl From<&[&str]> for Crates {
    fn from(value: &[&str]) -> Self {
        assert!(!value.is_empty());
        assert!(!value[0].is_empty());

        let a = value
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let m = a.len();
        let n = (a[0].len() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n];
        for i in (0..m - 1).rev() {
            for (j, stk) in stacks.iter_mut().enumerate() {
                let c = a[i][j * 4 + 1];
                if c != ' ' {
                    stk.push(c);
                }
            }
        }

        Crates { stacks }
    }
}

impl Crates {
    fn move_from_to(&mut self, n: usize, from: usize, to: usize) {
        for _ in 0..n {
            let tmp = self.stacks[from - 1].pop().unwrap();
            self.stacks[to - 1].push(tmp);
        }
    }

    fn top(&self) -> String {
        let mut answer = String::new();
        for s in &self.stacks {
            answer.push(s.last().copied().unwrap());
        }
        answer
    }
}

fn main() {
    let input = advent_of_code::read_file("inputs", 5);
    let lines = input.lines().collect::<Vec<_>>();
    let mut sep = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            sep = i;
            break;
        }
    }
    assert!(sep != 0);
    let mut c = Crates::from(&lines[..sep]);
    // println!("{c:?}");
    for line in &lines[sep + 1..] {
        let a: Vec<&str> = line.split(' ').collect();
        let n = a[1].parse().unwrap();
        let from = a[3].parse().unwrap();
        let to = a[5].parse().unwrap();
        c.move_from_to(n, from, to);
    }
    println!("{}", c.top());
}
