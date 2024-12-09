use std::{fs::File, io::BufReader};

mod part1;
mod part2;

fn main() {
    dbg!(part1::run(BufReader::new(File::open("input").unwrap())));
    dbg!(part2::run(BufReader::new(File::open("input").unwrap())));
}
