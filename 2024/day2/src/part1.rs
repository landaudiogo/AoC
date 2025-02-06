use std::io::BufRead;

use crate::common::validate_diff_pair;

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut sum = 0;

    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut levels = line.split_whitespace();
        let mut prev = levels.next().unwrap().parse::<i64>().unwrap();
        let diffs: Vec<_> = levels
            .into_iter()
            .map(|level| {
                let curr = level.parse::<i64>().unwrap();
                let res = curr - prev;
                prev = curr;
                res
            })
            .collect();

        let mut trend = None;
        let mut is_valid = true;
        for pair in diffs.windows(2) {
            if !validate_diff_pair(pair[0], pair[1], &mut trend) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            sum += 1
        }

        line.truncate(0)
    }

    sum
}
