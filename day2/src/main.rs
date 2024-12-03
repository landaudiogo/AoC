use std::fs::File;
use std::io::BufReader;

mod common;
mod part1;
mod part2;

fn main() {
    let file = File::open("input").unwrap();
    let buf = BufReader::new(file);
    println!("p1 - {}", part1::run(buf));

    let file = File::open("input").unwrap();
    let buf = BufReader::new(file);
    println!("p2 - {}", part2::run(buf));
}
