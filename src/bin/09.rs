use std::collections::HashSet;

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

struct Grid {
    knots: Vec<Pos>,
    visited: HashSet<Pos>,
}

impl Grid {
    fn new(n: usize) -> Self {
        let start_pos: Pos = Default::default();
        Grid {
            knots: vec![start_pos; n],
            visited: HashSet::from([start_pos]),
        }
    }

    fn simulate(&mut self, vx: i32, vy: i32, count: i32) {
        for _ in 0..count {
            self.knots[0].x += vx;
            self.knots[0].y += vy;

            for i in 1..self.knots.len() {
                let dx = self.knots[i - 1].x - self.knots[i].x;
                let dy = self.knots[i - 1].y - self.knots[i].y;

                if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
                    self.knots[i].x += dx / 2;
                    self.knots[i].y += dy / 2;
                } else if dx == 2 || dx == -2 {
                    self.knots[i].x += dx / 2;
                    self.knots[i].y = self.knots[i - 1].y;
                } else if dy == 2 || dy == -2 {
                    self.knots[i].x = self.knots[i - 1].x;
                    self.knots[i].y += dy / 2;
                } else {
                    break;
                }
            }

            self.visited.insert(*self.knots.last().unwrap());
        }
    }

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

fn main() {
    let input = advent_of_code::read_file("inputs", 9);

    // let mut grid = Grid::new(2);
    let mut grid = Grid::new(10);

    for line in input.lines() {
        let a: Vec<&str> = line.split(' ').collect();
        assert!(a.len() == 2);

        let (vx, vy) = match a[0] {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        grid.simulate(vx, vy, a[1].parse().unwrap());
    }

    println!("{}", grid.visited_count());
}
