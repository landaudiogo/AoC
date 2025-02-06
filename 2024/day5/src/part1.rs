use std::{collections::HashMap, io::BufRead};

use crate::common;

pub fn run<B: BufRead>(mut buf: B) -> i64 {
    let mut node_index = HashMap::new();
    let dig = common::create_graph(&mut buf, &mut node_index);

    let mut line = String::new();
    let mut sum = 0;
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let sequence: Vec<i64> = line
            .trim()
            .split(",")
            .map(|val| val.parse::<i64>().unwrap())
            .collect();

        sum += common::check_sequence(&node_index, &dig, &sequence)
            .map(|mid| mid)
            .unwrap_or(0);

        line.truncate(0);
    }

    sum
}
