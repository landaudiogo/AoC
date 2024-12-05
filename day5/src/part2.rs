use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use petgraph::{
    algo::toposort,
    graph::DiGraph,
    visit::{
        EdgeRef, IntoEdgeReferences, IntoNeighborsDirected, IntoNodeIdentifiers,
        IntoNodeReferences, NodeFiltered,
    },
    Direction, Graph,
};

use crate::common;

fn fix_sequence(dig: &DiGraph<i64, i64>, sequence: &Vec<i64>) -> i64 {
    let seq_set: HashSet<i64> = HashSet::from_iter(sequence.iter().map(|val| *val));
    let filtered = NodeFiltered::from_fn(dig, |node| seq_set.get(&dig[node]).is_some());

    let mut res = HashMap::new();
    for (node, _) in filtered.node_references() {
        let count = filtered
            .neighbors_directed(node, Direction::Incoming)
            .count();
        res.insert(count, dig[node]);
    }

    res.remove(&(res.len() / 2)).unwrap()
}

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

        if !common::check_sequence(&node_index, &dig, &sequence).is_some() {
            sum += fix_sequence(&dig, &sequence);
        }

        line.truncate(0);
    }

    sum
}
