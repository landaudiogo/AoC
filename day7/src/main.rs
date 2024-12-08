use std::{fs::File, io::BufReader};

mod common;
mod part1;
mod part2;

fn main() {
    println!(
        "p1 - {:?}",
        part1::run(BufReader::new(File::open("input").unwrap()))
    );
    println!(
        "p2 - {:?}",
        part2::run(BufReader::new(File::open("input").unwrap()))
    );
}
