use std::{collections::HashMap, io::BufRead};

use daggy::{petgraph::visit::Walker, Dag};
use regex::Regex;

#[derive(Debug, Clone)]
enum Node {
    Literal(Option<u64>),
    Op(Op),
}

#[derive(Debug, Clone)]
enum Op {
    And,
    Xor,
    Or,
    Computed,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "XOR" => Op::Xor,
            "OR" => Op::Or,
            "AND" => Op::And,
            _ => panic! {},
        }
    }
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    let mut dag: Dag<Node, ()> = Dag::new();
    let mut nodes = HashMap::new();
    let re = Regex::new(r"^(\w+): (\d+)").unwrap();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 1 {
            break;
        }

        let captures = re.captures(&line).unwrap();
        let node = String::from(&captures[1]);
        let literal = captures[2].parse::<u64>().unwrap();
        let literal = Node::Literal(Some(literal));
        let node_index = dag.add_node(literal.clone());
        nodes.insert(node, node_index);

        line.truncate(0);
    }

    let mut visit = Vec::new();
    let mut output_nodes = Vec::new();
    let mut line = String::new();
    let re = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)").unwrap();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }
        let captures = re.captures(&line).unwrap();
        let n1 = String::from(&captures[1]);
        let n1_index = if let Some(node_index) = nodes.get(&n1) {
            node_index.clone()
        } else {
            let node_index = dag.add_node(Node::Literal(None));
            nodes.insert(n1, node_index);
            node_index
        };

        let n2 = String::from(&captures[3]);
        let n2_index = if let Some(node_index) = nodes.get(&n2) {
            node_index.clone()
        } else {
            let node_index = dag.add_node(Node::Literal(None));
            nodes.insert(n2, node_index);
            node_index
        };

        let op = Op::from(&captures[2]);
        let op_index = dag.add_node(Node::Op(op));

        let out = String::from(&captures[4]);
        let out_index = if let Some(node_index) = nodes.get(&out) {
            node_index.clone()
        } else {
            let node_index = dag.add_node(Node::Literal(None));
            nodes.insert(out.clone(), node_index);
            node_index
        };
        if out.starts_with("z") {
            visit.push(out_index);
            output_nodes.push(out.clone());
        }
        nodes.insert(out, out_index);

        dag.add_edge(op_index, n1_index, ());
        dag.add_edge(op_index, n2_index, ());
        dag.add_edge(out_index, op_index, ());

        line.truncate(0);
    }

    // dbg!(dag);
    // dbg!(nodes);
    // return;
    while let Some(nidx) = visit.pop() {
        println!("{:?}: {:?}", nidx, &dag[nidx]);
        match &dag[nidx] {
            Node::Op(Op::Xor) => {
                let mut children_walker = dag.children(nidx);
                let mut incomplete = false;
                let mut children = Vec::new();
                while let Some((_, cidx)) = children_walker.walk_next(&dag) {
                    match dag[cidx] {
                        Node::Literal(None) => {
                            if !incomplete {
                                visit.push(nidx);
                                incomplete = true;
                            }
                            visit.push(cidx);
                        }
                        Node::Literal(Some(val)) => {
                            children.push(val);
                        }
                        _ => panic! {},
                    }
                }

                if !incomplete {
                    let res = children[0] ^ children[1];
                    let mut parent_walker = dag.parents(nidx);
                    let (_, pidx) = parent_walker.walk_next(&dag).unwrap();
                    dag[pidx] = Node::Literal(Some(res));
                    dag[nidx] = Node::Op(Op::Computed);
                }
            }
            Node::Op(Op::And) => {
                let mut children_walker = dag.children(nidx);
                let mut incomplete = false;
                let mut children = Vec::new();
                while let Some((_, cidx)) = children_walker.walk_next(&dag) {
                    match dag[cidx] {
                        Node::Literal(None) => {
                            if !incomplete {
                                visit.push(nidx);
                                incomplete = true;
                            }
                            visit.push(cidx);
                        }
                        Node::Literal(Some(val)) => {
                            children.push(val);
                        }
                        _ => panic! {},
                    }
                }

                if !incomplete {
                    let res = children[0] & children[1];
                    let mut parent_walker = dag.parents(nidx);
                    let (_, pidx) = parent_walker.walk_next(&dag).unwrap();
                    dag[pidx] = Node::Literal(Some(res));
                    dag[nidx] = Node::Op(Op::Computed);
                }
            }
            Node::Op(Op::Or) => {
                let mut children_walker = dag.children(nidx);
                let mut incomplete = false;
                let mut children = Vec::new();
                while let Some((_, cidx)) = children_walker.walk_next(&dag) {
                    match dag[cidx] {
                        Node::Literal(None) => {
                            if !incomplete {
                                visit.push(nidx);
                                incomplete = true;
                            }
                            visit.push(cidx);
                        }
                        Node::Literal(Some(val)) => {
                            children.push(val);
                        }
                        _ => panic! {},
                    }
                }

                if !incomplete {
                    let res = children[0] | children[1];
                    let mut parent_walker = dag.parents(nidx);
                    let (_, pidx) = parent_walker.walk_next(&dag).unwrap();
                    dag[pidx] = Node::Literal(Some(res));
                    dag[nidx] = Node::Op(Op::Computed);
                }
            }
            Node::Literal(None) => {
                let mut children_walker = dag.children(nidx);
                let mut incomplete = false;
                while let Some((_, cidx)) = children_walker.walk_next(&dag) {
                    match &dag[cidx] {
                        Node::Op(Op::Computed) => panic! {},
                        Node::Op(op) => {
                            if !incomplete {
                                visit.push(nidx);
                                incomplete = true;
                            }
                            visit.push(cidx);
                        }
                        _ => panic! {},
                    }
                }
            }
            _ => {}
        }
    }
    output_nodes.sort();
    for i in (0..45).rev() {
        let x = format!("x{:>02}", i);
        let nidx = nodes[&x];
        match dag[nidx] {
            Node::Literal(Some(b)) => {
                print!("{}", b);
            }
            _ => {}
        }
    }
    println!();
    for i in (0..45).rev() {
        let y = format!("y{:>02}", i);
        let nidx = nodes[&y];
        match dag[nidx] {
            Node::Literal(Some(b)) => {
                print!("{}", b);
            }
            _ => {}
        }
    }
    println!();
    for node in output_nodes.iter().rev() {
        let nidx = nodes[node];
        match dag[nidx] {
            Node::Literal(Some(b)) => {
                print!("{}", b);
            }
            _ => {}
        }
    }
    println!()
}
