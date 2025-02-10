use crate::intcode::{Pipe, Program};
use itertools::Itertools;
use std::{collections::VecDeque, io::BufRead};

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let iter = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
    let mut max = None;

    for s in (0..5).permutations(5) {
        let mut pipea = Pipe::new_with_data(VecDeque::from([s[0], 0]));
        let mut pipeab = Pipe::new_with_data(VecDeque::from([s[1]]));
        let mut pipebc = Pipe::new_with_data(VecDeque::from([s[2]]));
        let mut pipecd = Pipe::new_with_data(VecDeque::from([s[3]]));
        let mut pipede = Pipe::new_with_data(VecDeque::from([s[4]]));
        let mut pipee = Pipe::new();
        let mut ampa = Program::new(iter.clone(), &mut pipea, &mut pipeab);
        ampa.execute();
        drop(ampa);
        let mut ampb = Program::new(iter.clone(), &mut pipeab, &mut pipebc);
        ampb.execute();
        drop(ampb);
        let mut ampc = Program::new(iter.clone(), &mut pipebc, &mut pipecd);
        ampc.execute();
        drop(ampc);
        let mut ampd = Program::new(iter.clone(), &mut pipecd, &mut pipede);
        ampd.execute();
        drop(ampd);
        let mut ampe = Program::new(iter.clone(), &mut pipede, &mut pipee);
        ampe.execute();
        drop(ampe);

        let thruster = pipee.inner[0];
        if thruster == 65210 {
            println!("{:?}", s);
        }
        if max.is_none() || thruster > max.unwrap() {
            max = Some(thruster);
        }
    }
    println!("{:?}", max);
}
