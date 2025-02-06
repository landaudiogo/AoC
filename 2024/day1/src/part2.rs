use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut left = HashSet::new();
    let mut right = HashMap::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut elements = line.split_whitespace();
        left.insert(elements.next().unwrap().parse::<u64>().unwrap());
        let elem = right
            .entry(elements.next().unwrap().parse::<u64>().unwrap())
            .or_insert(0 as u64);
        *elem += 1;

        line.truncate(0);
    }
    let sum = left
        .into_iter()
        .map(|elem| right.get_mut(&elem).map_or(0, |count| *count * elem))
        .reduce(|a, b| a + b);

    sum.unwrap()
}
