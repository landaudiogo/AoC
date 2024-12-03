use std::fs::File;
use std::io::BufReader;

mod part1;
mod part2;

fn main() {
    let buf = BufReader::new(File::open("input").expect("file"));
    println!("p1 - {:?}", part1::run(buf));

    let buf = BufReader::new(File::open("input").expect("file"));
    println!("p2 - {:?}", part2::run(buf));
}
