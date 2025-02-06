use std::{fs::File, io::BufReader};

mod part1;

fn main() {
    let ans = part1::run(BufReader::new(File::open("input").unwrap()));
    println!("p1 - {:?}", ans.0);
    println!("p2 - {:?}", ans.1);
}
