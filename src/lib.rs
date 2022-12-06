use std::{env, fs};

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join(folder).join(format!("{day:02}.txt"));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
