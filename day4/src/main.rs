use std::{fs::File, io::BufReader};

mod part1;

fn main() {
    println!(
        "{}",
        part1::run(BufReader::new(File::open("input").unwrap()))
    )
}
