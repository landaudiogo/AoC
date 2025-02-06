use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::{Dfs, NodeFiltered},
};

pub fn check_sequence(
    node_index: &HashMap<i64, NodeIndex>,
    dig: &DiGraph<i64, i64>,
    sequence: &Vec<i64>,
) -> Option<i64> {
    let seq_set: HashSet<i64> = HashSet::from_iter(sequence.iter().map(|val| *val));
    let filtered = NodeFiltered::from_fn(dig, |node| seq_set.get(&dig[node]).is_some());

    for (i, smaller) in sequence.iter().enumerate() {
        for bigger in &sequence[i + 1..] {
            let bigger = node_index.get(bigger).unwrap();
            let mut dfs = Dfs::new(&filtered, *bigger);
            while let Some(node) = dfs.next(&filtered) {
                if dig[node] == *smaller {
                    return None;
                }
            }
        }
    }

    Some(sequence[sequence.len() / 2])
}

pub fn create_graph<B: BufRead>(
    mut buf: B,
    node_index: &mut HashMap<i64, NodeIndex>,
) -> DiGraph<i64, i64> {
    let mut line = String::new();
    let mut dig = DiGraph::<i64, i64>::new();

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
    dig
}
