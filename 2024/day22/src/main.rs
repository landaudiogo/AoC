use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod part1;
mod part2;

fn main() {
    part1::run(BufReader::new(File::open("input").unwrap()));
    part2::run(BufReader::new(File::open("input").unwrap()));
}
