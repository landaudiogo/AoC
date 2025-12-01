use crate::intcode::Program;
use itertools::Itertools;
use std::{collections::HashMap, io::BufRead, sync::mpsc, thread};

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let iter = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
    let mut max = None;

    for s in (0..=4).permutations(5) {
        let mut txs = HashMap::new();
        let mut rxs = HashMap::new();
        (0..6).for_each(|i| {
            let (tx, rx) = mpsc::channel();
            if i < 5 {
                tx.send(s[i]).unwrap();
            }
            if i == 0 {
                tx.send(0).unwrap();
            }
            txs.insert(i, tx);
            rxs.insert(i, rx);
        });

        thread::scope(|sc| {
            for amp in 0..5 {
                let mut rx = rxs.remove(&amp).unwrap();
                let mut tx = txs.remove(&(amp + 1)).unwrap();
                let iter = iter.clone();
                sc.spawn(move || {
                    let mut amp = Program::new(iter, &mut rx, &mut tx);
                    amp.execute();
                });
            }

            let rxt = rxs.remove(&5).unwrap();
            let txa = txs.remove(&0).unwrap();
            while let Ok(thruster) = rxt.recv() {
                txa.send(thruster);
                if max.is_none() || thruster > max.unwrap() {
                    max = Some(thruster);
                }
            }
        });
    }
    println!("{:?}", max);
}
