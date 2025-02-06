use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn fully_connected(graph: Vec<Vec<u64>>, node_index: HashMap<String, usize>, nodes: usize) -> u64 {
    let mut total = 0;
    let index_node: HashMap<usize, String> =
        HashMap::from_iter(node_index.iter().map(|(k, v)| (*v, k.clone())));
    for (i, row) in graph.iter().enumerate() {
        let subset = &row[i..row.len()];
        let connected = subset
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 1)
            .map(|(j, _)| i + j)
            .collect::<Vec<usize>>();
        let pairs = connected.iter().combinations(nodes - 1);
        let pairs = Vec::from_iter(pairs);
        for mut indices in pairs {
            indices.push(&i);
            let mut fully_connected = Vec::new();
            for idx in indices.iter() {
                let sub: Vec<u64> = indices
                    .iter()
                    .filter(|i| *i != idx)
                    .map(|i| graph[**idx][**i])
                    .collect();
                fully_connected.push(sub);
            }
            let sum = fully_connected
                .iter()
                .fold(0, |acc, row| acc + row.iter().fold(0, |acc, v| acc + v));
            if sum as usize == indices.len() * (indices.len() - 1) {
                let mut nodes: Vec<&String> = indices
                    .iter()
                    .map(|idx| index_node.get(idx).unwrap())
                    .collect();
                nodes.sort();
                println!("{:?}", nodes.iter().join(","))
                // total += 1;
            }
        }
    }

    total
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    let mut node_map: HashMap<String, Vec<String>> = HashMap::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut hosts = line.split("-");
        let host1 = hosts.next().unwrap().trim();
        let host2 = hosts.next().unwrap().trim();
        node_map
            .entry(host1.into())
            .or_insert_with(|| Vec::new())
            .push(host2.into());
        node_map
            .entry(host2.into())
            .or_insert_with(|| Vec::new())
            .push(host1.into());

        line.truncate(0);
    }

    let mut filtered_nodes: HashSet<String> = HashSet::new();
    for node in node_map.keys() {
        filtered_nodes.insert(node.clone());
        for connected in node_map.get(node).unwrap() {
            filtered_nodes.insert(connected.clone());
        }
    }

    let node_index: HashMap<String, usize> = filtered_nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.clone(), i))
        .collect();

    let mut graph = Vec::with_capacity(filtered_nodes.len());
    for (i, node) in filtered_nodes.iter().enumerate() {
        let mut row: Vec<u64> = (0..filtered_nodes.len()).map(|_| 0).collect();
        for connected in node_map.get(node).unwrap() {
            if let Some(node_index) = node_index.get(connected) {
                row[*node_index] = 1;
            }
        }
        graph.push(row);
    }

    println!("{:?}", fully_connected(graph, node_index, 13));
}
