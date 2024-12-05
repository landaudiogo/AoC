use std::collections::{HashMap, HashSet};

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

    for (i, curr) in sequence.iter().enumerate() {
        let bigger_list = &sequence[i + 1..];
        if bigger_list.len() == 0 {
            break;
        }

        for bigger in bigger_list {
            let bigger = node_index.get(bigger).unwrap();
            let mut visited = HashSet::new();
            let mut dfs = Dfs::new(&filtered, *bigger);
            while let Some(node) = dfs.next(&filtered) {
                if seq_set.get(&dig[node]).is_none() {
                    continue;
                }

                if visited.get(&dig[node]).is_some() {
                    continue;
                }

                if dig[node] == *curr {
                    return None;
                }
                visited.insert(&dig[node]);
            }
        }
    }

    Some(sequence[sequence.len() / 2])
}
