use std::{io::BufRead, sync::mpsc};

use crate::intcode::Program;

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program);
    let mut total = 0;

    for x in 0..50 {
        for y in 0..50 {
            let (itx, mut irx) = mpsc::channel();
            let (mut otx, orx) = mpsc::channel();
            let program = program.clone();
            std::thread::spawn(move || {
                let program = program.trim().split(",").map(|v| v.parse().unwrap());
                let mut program = Program::new(program, &mut irx, &mut otx);
                program.execute();
            });

            itx.send(x);
            itx.send(y);
            let Ok(motion) = orx.recv() else {
                panic!("should not reach here")
            };
            total += motion;
        }
    }
    println!("{total}");
}
