use std::io::BufRead;

use crate::common;

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut sum = 0;
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let colon_idx = line.chars().position(|c| c == ':').unwrap();
        let result = line
            .chars()
            .take(colon_idx)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let remainder: String = line.chars().skip(colon_idx + 2).collect();
        let remainder = remainder.trim();
        let operands: Vec<u64> = remainder
            .split(" ")
            .map(|operand| operand.parse::<u64>().unwrap())
            .collect();

        sum += common::search_combination(result, operands, 3);

        line.truncate(0);
    }

    sum
}
