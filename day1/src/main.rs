mod part1;
mod part2;

use std::{fs::File, io::BufReader};

fn main() {
    println!(
        "p1 - {:?}",
        part1::run(BufReader::new(File::open("input").expect("file")))
    );
    println!(
        "p2 - {:?}",
        part2::run(BufReader::new(File::open("input").expect("file")))
    )
}
