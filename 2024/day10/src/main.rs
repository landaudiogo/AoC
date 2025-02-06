use std::{fs::File, io::BufReader};

mod part2;
mod part1;
mod common;

fn main() {
    dbg!(part1::run(BufReader::new(File::open("input").unwrap())));
    dbg!(part2::run(BufReader::new(File::open("input").unwrap())));
}
