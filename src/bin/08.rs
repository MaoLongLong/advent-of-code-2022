/// See <https://stackoverflow.com/questions/13102786/two-dimensional-vectors-in-rust>
pub struct Vec2D<T> {
    vec: Vec<T>,
    size: usize,
}

impl<T> Vec2D<T> {
    pub fn new(vec: Vec<T>, size: usize) -> Self {
        assert!(vec.len() == size.pow(2));
        Self { vec, size }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let i = self.size * row;
        &self.vec[i + col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let i = self.size * row;
        &mut self.vec[i + col]
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.vec
    }
}

struct Map {
    size: i32,
    trees: Vec2D<u8>,
    visible: Vec2D<bool>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();
        let size = lines.len();
        let mut height = Vec2D::new(vec![0u8; size.pow(2)], size);
        for (i, line) in lines.into_iter().enumerate() {
            for (j, b) in line.bytes().into_iter().enumerate() {
                *height.get_mut(i, j) = b - b'0';
            }
        }
        Map {
            size: size as i32,
            trees: height,
            visible: Vec2D::new(vec![false; size.pow(2)], size),
        }
    }
}

impl Map {
    fn mark_visible(&mut self) {
        let mut helper = |mut x, mut y, dx, dy, nx, ny| {
            for _ in 0..self.size {
                let mut max_height = 0;
                let (tx, ty) = (x, y);
                loop {
                    if x < 0 || y < 0 || x >= self.size || y >= self.size {
                        break;
                    }
                    let h = self.trees.get(x as usize, y as usize);
                    if *h > max_height
                        || x == 0
                        || y == 0
                        || x == self.size - 1
                        || y == self.size - 1
                    {
                        max_height = *h;
                        *self.visible.get_mut(x as usize, y as usize) = true;
                    }
                    x += dx;
                    y += dy;
                }
                x = tx + nx;
                y = ty + ny;
            }
        };

        helper(1, 0, 0, 1, 1, 0);
        helper(0, 0, 1, 0, 0, 1);
        helper(self.size - 1, self.size - 1, 0, -1, -1, 0);
        helper(self.size - 1, self.size - 1, -1, 0, 0, -1);
    }

    fn visible_count(&self) -> usize {
        self.visible.inner().iter().filter(|x| **x).count()
    }

    fn distance(&self, mut x: i32, mut y: i32, dx: i32, dy: i32) -> usize {
        let mut dis = 0;
        let current = self.trees.get(x as usize, y as usize);
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= self.size || y >= self.size {
                break;
            }
            dis += 1;
            if self.trees.get(x as usize, y as usize) >= current {
                break;
            }
        }
        dis
    }

    fn score(&self, x: i32, y: i32) -> usize {
        self.distance(x, y, 0, 1)
            * self.distance(x, y, 0, -1)
            * self.distance(x, y, 1, 0)
            * self.distance(x, y, -1, 0)
    }

    fn best_score(&self) -> usize {
        let mut best = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if !self.visible.get(i as usize, j as usize) {
                    continue;
                }
                best = best.max(self.score(i, j));
            }
        }
        best
    }
}

fn main() {
    let input = advent_of_code::read_file("inputs", 8);

    let mut map = Map::from(input.as_str());
    map.mark_visible();

    println!("part1: {}", map.visible_count());
    println!("part2: {}", map.best_score());
}
