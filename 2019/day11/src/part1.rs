use crate::intcode::Program;
use std::{
    collections::HashMap,
    io::BufRead,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

struct Plate {
    pos: (i64, i64),
    orientation: (i64, i64),
    painted: HashMap<(i64, i64), i64>,
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl Plate {
    fn new(tx: Sender<i64>, rx: Receiver<i64>) -> Self {
        Self {
            pos: (0, 0),
            orientation: (-1, 0),
            painted: HashMap::new(),
            tx,
            rx,
        }
    }

    fn interact(&mut self) {
        loop {
            if let Err(_) = self.tx.send(*self.painted.get(&self.pos).unwrap_or(&0)) {
                break;
            }

            if let Ok(color) = self.rx.recv() {
                self.painted.insert(self.pos, color);
            } else {
                break;
            }

            let rotation = if let Ok(rotation) = self.rx.recv() {
                rotation
            } else {
                break;
            };

            match rotation {
                0 => {
                    self.orientation = (self.orientation.1 * -1, self.orientation.0 * 1);
                    self.pos = (
                        self.pos.0 + self.orientation.0,
                        self.pos.1 + self.orientation.1,
                    );
                }
                1 => {
                    self.orientation = (self.orientation.1 * 1, self.orientation.0 * -1);
                    self.pos = (
                        self.pos.0 + self.orientation.0,
                        self.pos.1 + self.orientation.1,
                    );
                }
                _ => panic!("Invalid rotation"),
            }
        }
    }
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let (txi, mut rxi) = mpsc::channel();
    let (mut txo, rxo) = mpsc::channel();
    thread::spawn(move || {
        let iter = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
        let mut program = Program::new(iter, &mut rxi, &mut txo);
        program.execute();
    });

    let mut plate = Plate::new(txi, rxo);
    plate.interact();
    println!("{}", plate.painted.len());
}
