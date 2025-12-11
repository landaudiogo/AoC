use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    p1(&mut input);
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

    let mut total = 0;
    let mut visit = VecDeque::new();
    edges
        .get("you")
        .unwrap()
        .into_iter()
        .for_each(|conn| visit.push_back((*conn, String::from(*conn))));
    while let Some((node, path)) = visit.pop_front() {
        if node == "out" {
            total += 1;
        }

        let Some(connections) = edges.get(node) else {
            continue;
        };
        for conn in connections {
            let path = path.clone();
            visit.push_back((conn, path + "," + conn));
        }
    }
    println!("p1: {total}")
}
