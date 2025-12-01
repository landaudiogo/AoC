use std::{fs::File, io::BufReader};

mod intcode;

fn main() {
    intcode::run(BufReader::new(File::open("input").unwrap()));
}
