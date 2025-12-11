use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    p1(&mut input);
    p2(&mut input);
}

fn p1(input: &str) {
    let mut edges = HashMap::new();

    for line in input.lines() {
        let Some(sep_idx) = line.find(":") else {
            panic!("Every line should have a colon seperator")
        };

        let origin = &line[..sep_idx];
        let connections = line[sep_idx + 2..].split_whitespace();
        for connection in connections {
            edges.entry(origin).or_insert(Vec::new()).push(connection);
        }
    }

    println!("p1: {}", paths_to(&edges, "you", "out"));
}

fn p2(input: &str) {
    let mut edges = HashMap::new();

    for line in input.lines() {
        let Some(sep_idx) = line.find(":") else {
            panic!("Every line should have a colon seperator")
        };

        let origin = &line[..sep_idx];
        let connections = line[sep_idx + 2..].split_whitespace();
        for connection in connections {
            edges.entry(origin).or_insert(Vec::new()).push(connection);
        }
    }

    let total_paths = paths_to(&edges, "svr", "fft")
        * paths_to(&edges, "fft", "dac")
        * paths_to(&edges, "dac", "out");
    println!("p2: {:?}", total_paths);
}

fn paths_to(edges: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> u64 {
    let mut seen: HashMap<&str, u64> = HashMap::new();
    let mut visit = VecDeque::new();
    visit.push_back(start);
    while let Some(node) = visit.pop_back() {
        if node == end {
            seen.insert(node, 1);
            continue;
        }

        if node == "out" {
            seen.insert(node, 0);
            continue;
        }

        let Some(connections) = edges.get(node) else {
            continue;
        };

        let mut total_paths = 0;
        let mut missing = false;
        for conn in connections {
            let Some(subpaths) = seen.get(conn) else {
                missing = true;
                visit.push_back(node);
                visit.push_back(conn);
                break;
            };
            total_paths += subpaths;
        }

        if !missing {
            seen.insert(node, total_paths);
        }
    }
    seen.remove(start).unwrap()
}
