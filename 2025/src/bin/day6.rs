use std::io::{self, Read};

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "*" => Op::Mul,
            "+" => Op::Add,
            _ => panic!("invalid value {value}"),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut operands: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<Op> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for operand in line.trim().split_whitespace() {
            let Ok(operand) = operand.parse() else { break };
            row.push(operand);
        }

        if row.len() > 0 {
            operands.push(row);
        } else {
            for operator in line.trim().split_whitespace() {
                operators.push(Op::from(operator));
            }
        }
    }

    let mut total = 0;
    for (c, op) in operators.into_iter().enumerate() {
        let mut tmp = if let Op::Add = op { 0 } else { 1 };
        for r in 0..operands.len() {
            tmp = if let Op::Add = op {
                tmp + operands[r][c]
            } else {
                tmp * operands[r][c]
            };
        }
        total += tmp;
    }

    println!("{total}");
    // println!("{:?}", operands[0]);
    // println!("{:?}", operators);
}
