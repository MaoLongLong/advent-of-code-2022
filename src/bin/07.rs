use std::{cell::RefCell, rc::Rc};

struct Dir {
    _name: String,
    size: usize,
    totalsize: usize,
    sub: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(name: &str, parent: Option<Rc<RefCell<Dir>>>) -> Rc<RefCell<Dir>> {
        Rc::new(RefCell::new(Dir {
            _name: String::from(name),
            size: 0,
            totalsize: 0,
            sub: Vec::new(),
            parent,
        }))
    }
}

fn read_dirs(input: &str) -> Option<Rc<RefCell<Dir>>> {
    let mut cwd: Option<Rc<RefCell<Dir>>> = None;
    let mut root: Option<Rc<RefCell<Dir>>> = None;

    for line in input.lines() {
        if line.starts_with('$') {
            let args: Vec<&str> = line[2..].split(' ').collect();
            assert!(!args.is_empty());

            if args[0] == "ls" {
                continue;
            }

            let name = args[1];
            if name == "/" {
                assert!(cwd.is_none());
                cwd = Some(Dir::new(name, None));
                root = cwd.clone();
                continue;
            }

            let dir = cwd.take().unwrap();
            let mut dir_borrow = dir.borrow_mut();
            if name == ".." {
                dir_borrow.parent.as_ref().unwrap().borrow_mut().totalsize += dir_borrow.totalsize;
                cwd = dir_borrow.parent.clone();
            } else {
                let new_dir = Dir::new(name, Some(dir.clone()));
                dir_borrow.sub.push(new_dir.clone());
                cwd = Some(new_dir);
            }
        } else {
            assert!(cwd.is_some());
            let a: Vec<&str> = line.split(' ').collect();
            if a[0] == "dir" {
                continue;
            }
            let size: usize = a[0].parse().unwrap();
            let dir = cwd.as_ref().unwrap();
            let mut dir_borrow = dir.borrow_mut();
            dir_borrow.size += size;
            dir_borrow.totalsize += size;
        }
    }

    while let Some(dir) = cwd.take() {
        let dir_borrow = dir.borrow();
        match dir_borrow.parent.as_ref() {
            Some(parent) => parent.borrow_mut().totalsize += dir_borrow.totalsize,
            None => break,
        }
        cwd = dir_borrow.parent.clone();
    }

    root
}

fn part1(dir: &Rc<RefCell<Dir>>, answer1: &mut usize) {
    let dir_borrow = dir.borrow();
    for i in 0..dir_borrow.sub.len() {
        let size = dir_borrow.sub[i].borrow().totalsize;
        if size <= 100000 {
            *answer1 += size;
        }
        part1(&dir_borrow.sub[i], answer1);
    }
}

fn part2(dir: &Rc<RefCell<Dir>>, min_size: usize, answer2: &mut usize) {
    let dir_borrow = dir.borrow();
    for i in 0..dir_borrow.sub.len() {
        let size = dir_borrow.sub[i].borrow().totalsize;
        if size >= min_size && (*answer2 == 0 || size < *answer2) {
            *answer2 = size;
        }
        part2(&dir_borrow.sub[i], min_size, answer2);
    }
}

fn main() {
    let input = advent_of_code::read_file("inputs", 7);
    let root = read_dirs(&input).unwrap();

    let mut answer1 = 0;
    part1(&root, &mut answer1);
    println!("part1: {answer1}");

    let mut answer2 = 0;
    let min_size = 30000000 - (70000000 - root.borrow().totalsize);
    part2(&root, min_size, &mut answer2);
    println!("part2: {answer2}");
}
