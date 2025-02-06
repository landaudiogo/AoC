use std::{io::BufRead, iter};

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut left = Vec::new();
    let mut right = Vec::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut elements = line.split_whitespace();
        left.push(elements.next().unwrap().parse::<i64>().unwrap());
        right.push(elements.next().unwrap().parse::<i64>().unwrap());
        line.truncate(0);
    }
    left.sort();
    right.sort();
    let sum = iter::zip(left, right)
        .map(|(a, b)| (a - b).abs() as u64)
        .reduce(|a, b| a + b);

    sum.unwrap()
}
