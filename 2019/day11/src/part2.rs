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
        let painted = HashMap::from([((0, 0), 1)]);
        Self {
            pos: (0, 0),
            orientation: (-1, 0),
            painted,
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

    fn draw(&self) {
        let mut pixels: Vec<((i64, i64), i64)> = self
            .painted
            .iter()
            .map(|(coord, paint)| (*coord, *paint))
            .collect();
        pixels.sort();
        let (farthest_coord, _) = pixels.last().unwrap();
        for row in 0..=farthest_coord.0 {
            for col in 0..=farthest_coord.1 {
                let paint = *self.painted.get(&(row, col)).unwrap_or(&0);
                let c = if paint == 0 { ' ' } else { '█' };
                print!("{c}");
            }
            println!();
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
    plate.draw();
}
