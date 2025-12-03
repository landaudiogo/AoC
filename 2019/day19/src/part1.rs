use std::{fmt::Display, io::BufRead, sync::mpsc};

use crate::intcode::Program;

struct Grid {
    inner: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { inner: grid }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.inner.iter().enumerate() {
            write!(f, "{i} ");
            for cell in row {
                write!(f, "{cell}");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program);
    let (itx, mut irx) = mpsc::channel();
    let (mut otx, orx) = mpsc::channel();
    std::thread::spawn(move || {
        let program = program.trim().split(",").map(|v| v.parse().unwrap());
        let mut program = Program::new(program, &mut irx, &mut otx);

        loop {
            program.execute();
            program.reset();
        }
    });

    let mut total = 0;
    let mut grid = Vec::new();
    for y in 0..50 {
        let mut row = Vec::new();
        for x in 0..50 {
            itx.send(x);
            itx.send(y);
            let Ok(motion) = orx.recv() else {
                panic!("should not reach here")
            };
            total += motion;
            row.push(if motion == 1 { '#' } else { '.' });
        }
        grid.push(row);
    }
    let grid = Grid::new(grid);
    println!("{grid}");
    println!("p1: {total}");
}
