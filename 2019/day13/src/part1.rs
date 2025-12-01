use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    sync::mpsc,
};

use crate::intcode::Program;

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let program = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
    let mut stdin = std::io::stdin();
    let (mut tx, rx) = mpsc::channel();
    let mut program = Program::new(program, &mut stdin, &mut tx);
    program.execute();
    drop(tx);

    let mut blocks = HashSet::new();
    while let (Ok(x), Ok(y), Ok(item)) = (rx.recv(), rx.recv(), rx.recv()) {
        if item == 2 {
            blocks.insert((x, y));
        }
    }

    println!("{}", blocks.len());
}
