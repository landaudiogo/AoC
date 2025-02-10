use crate::intcode::Program;
use std::io::{self, BufRead};

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let iter = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut program = Program::new(iter, &mut stdin, &mut stdout);
    program.execute();
}
