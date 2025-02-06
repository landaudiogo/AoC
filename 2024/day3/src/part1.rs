use regex::Regex;
use std::io::BufRead;

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut s = String::new();
    buf.read_to_string(&mut s).unwrap();
    for captures in re.captures_iter(&s) {
        let x = captures[1].parse::<u64>().unwrap();
        let y = captures[2].parse::<u64>().unwrap();
        sum += x * y;
    }

    sum
}
