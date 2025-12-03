use std::{
    fmt::Display,
    io::BufRead,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::intcode::Program;

const ARESTA: i64 = 100 - 1;

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
        for row in self.inner.iter() {
            for cell in row {
                write!(f, "{cell}");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

type Coord = (i64, i64);

fn nw2sw(pos: Coord) -> Coord {
    (pos.0 + ARESTA, pos.1)
}

fn sw2nw(pos: Coord) -> Coord {
    (pos.0 - ARESTA, pos.1)
}

fn nw2ne(pos: Coord) -> Coord {
    (pos.0, pos.1 + ARESTA)
}

fn ne2nw(pos: Coord) -> Coord {
    (pos.0, pos.1 - ARESTA)
}

fn is_hit(pos: Coord, tx: &Sender<i64>, rx: &Receiver<i64>) -> bool {
    tx.send(pos.1).unwrap();
    tx.send(pos.0).unwrap();
    if rx.recv().unwrap() == 1 {
        true
    } else {
        false
    }
}

fn right_scan(pos: Coord, tx: &Sender<i64>, rx: &Receiver<i64>) -> Coord {
    for i in 0.. {
        if is_hit((pos.0, pos.1 + i), tx, rx) {
            return (pos.0, pos.1 + i);
        }
    }
    pos
}

fn down_scan(pos: Coord, tx: &Sender<i64>, rx: &Receiver<i64>) -> Coord {
    for i in 0.. {
        if is_hit((pos.0 + i, pos.1), tx, rx) {
            return (pos.0 + i, pos.1);
        }
    }
    pos
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

    let mut nw = (0, 0);
    loop {
        let mut sw = nw2sw(nw);
        if !is_hit(sw, &itx, &orx) {
            sw = right_scan(sw, &itx, &orx);
            nw = sw2nw(sw);
            nw = down_scan(nw, &itx, &orx);
            continue;
        }

        let mut ne = nw2ne(nw);
        if !is_hit(ne, &itx, &orx) {
            ne = down_scan(ne, &itx, &orx);
            nw = ne2nw(ne);
            nw = right_scan(nw, &itx, &orx);
            continue;
        } else {
            println!("p2: {:?} -> {:?}", nw, nw.1 * 10000 + nw.0);
            break;
        }
    }
}
