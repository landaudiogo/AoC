use std::{collections::HashMap, io::BufRead};

use daggy::{
    petgraph::visit::{Dfs, Walker},
    Dag, NodeIndex,
};
use regex::Regex;

#[derive(Debug, Clone)]
enum Node {
    Literal(Option<u64>),
    Op(Op),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Op {
    And,
    Xor,
    Or,
    Computed,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Equation {
    operands: Vec<String>,
    operator: Op,
}

impl Equation {
    fn new(nodes: &mut [String], operator: Op) -> Self {
        nodes.sort();
        Self {
            operands: Vec::from(nodes),
            operator,
        }
    }
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
    let re = Regex::new(r"^(\w+): (\d+)").unwrap();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 1 {
            break;
        }

        let captures = re.captures(&line).unwrap();
        let node = String::from(&captures[1]);
        let literal = captures[2].parse::<u64>().unwrap();

        line.truncate(0);
    }

    let mut line = String::new();
    let mut node_eq = HashMap::new();
    let mut eq_node = HashMap::new();
    let re = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)").unwrap();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }
        let captures = re.captures(&line).unwrap();
        let n1 = String::from(&captures[1]);
        let op = Op::from(&captures[2]);
        let n2 = String::from(&captures[3]);
        let out = String::from(&captures[4]);
        let eq = Equation::new(&mut [n1, n2], op);
        eq_node.insert(eq.clone(), out.clone());
        node_eq.insert(out, eq);

        line.truncate(0);
    }
    dbg!(&node_eq["z01"]);
    let mut carries = HashMap::new();
    carries.insert(
        0,
        eq_node[&Equation::new(&mut ["x00".into(), "y00".into()], Op::And)].clone(),
    );

    for i in 1..44 {
        let curr = format!("{:<02}", i);
        let x = String::from("x") + &curr;
        let y = String::from("y") + &curr;
        let leq = Equation::new(&mut [x.clone(), y.clone()], Op::Xor);
        let lnode = &eq_node[&leq];
        println!("input xor: {}", lnode);
        let leq = Equation::new(&mut [lnode.clone(), carries[&(i - 1)].clone()], Op::And);
        let lnode = &eq_node[&leq];
        println!("carry & input: {}", lnode);
        let req = Equation::new(&mut [x.clone(), y.clone()], Op::And);
        let rnode = &eq_node[&req];
        println!("input and: {}", rnode);
        let eq = Equation::new(&mut [lnode.clone(), rnode.clone()], Op::Or);
        carries.insert(i, eq_node[&eq].clone());
        println!("c{:0>2}: {:?}", i, carries[&i]);

        // println!("{} = carry-1 ^ ({} ^ {})", z, x, y);
    }
    // dbg!(dag);
    // dbg!(output_nodes);
}
