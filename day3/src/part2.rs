use regex::Regex;
use std::io::BufRead;

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut sum = 0;
    let mut enabled = true;

    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    let mut s = String::new();
    buf.read_to_string(&mut s).unwrap();
    for captures in re.captures_iter(&s) {
        if captures[0].starts_with("don't") {
            enabled = false;
            continue;
        } else if captures[0].starts_with("do(") {
            enabled = true;
            continue;
        }

        if !enabled {
            continue;
        }

        let x = captures[2].parse::<u64>().unwrap();
        let y = captures[3].parse::<u64>().unwrap();
        sum += x * y;
    }

    sum
}
