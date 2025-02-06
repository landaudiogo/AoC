use std::{fs::File, io::BufReader};

mod common;
mod part1;
mod part2;

fn main() {
    println!(
        "{}",
        part1::run(BufReader::new(File::open("input").unwrap()))
    );
    println!(
        "{}",
        part2::run(BufReader::new(File::open("input").unwrap()))
    );
}
