use std::{collections::HashMap, io::BufRead};

use petgraph::graph::DiGraph;

use crate::common;

pub fn run<B: BufRead>(mut buf: B) -> i64 {
    let mut dig = DiGraph::<i64, i64>::new();
    let mut line = String::new();
    let mut node_index = HashMap::new();

    while let Ok(len) = buf.read_line(&mut line) {
        let rule = line.trim();
        if rule.len() == 0 || len == 0 {
            break;
        }

        let mut elements = rule.split("|");
        let from = elements.next().unwrap().parse::<i64>().unwrap();
        let to = elements.next().unwrap().parse::<i64>().unwrap();

        let from = *node_index.entry(from).or_insert_with(|| dig.add_node(from));
        let to = *node_index.entry(to).or_insert_with(|| dig.add_node(to));
        dig.add_edge(from, to, 1);

        line.truncate(0)
    }

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
