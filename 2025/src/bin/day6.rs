use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
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
    p1(&input);
    p2(&input);
}

fn p1(input: &str) {
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

    println!("p1: {total}");
}

fn p2(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let operators: String = grid.pop().unwrap().iter().collect();
    let operators: Vec<Op> = operators
        .split_whitespace()
        .map(|c_op| Op::from(c_op))
        .collect();

    let mut op_idx = 0;
    let mut op = operators[op_idx];
    let mut tmp = if let Op::Add = op { 0 } else { 1 };
    let mut total = 0;

    for c in 0..grid[0].len() {
        let mut col = String::new();
        for r in 0..(grid.len()) {
            if grid[r][c] == ' ' {
                continue;
            }
            col.push(grid[r][c]);
        }

        if col.len() == 0 {
            op_idx += 1;
            op = operators[op_idx];
            total += tmp;
            tmp = if let Op::Add = op { 0 } else { 1 };
            continue;
        }

        let val: i64 = col.parse().unwrap();
        tmp = if let Op::Add = op {
            tmp + val
        } else {
            tmp * val
        };
    }
    total += tmp;

    println!("p2: {total}");
}
