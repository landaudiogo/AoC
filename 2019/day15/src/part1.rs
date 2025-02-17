use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::intcode::Program;

fn move_to_start(path: &[i64], itx: &mut Sender<i64>, orx: &Receiver<i64>) {
    for mv in path.iter().rev() {
        let cmp = match mv {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!(),
        };
        itx.send(cmp).unwrap();
        if orx.recv().unwrap() != 1 {
            panic!();
        }
    }
}

fn path_to_pos(path: &[i64]) -> (i64, i64) {
    let mut pos = (0, 0);
    for mv in path {
        match mv {
            1 => {
                pos.0 += -1;
            }
            2 => {
                pos.0 += 1;
            }
            3 => {
                pos.1 += -1;
            }
            4 => {
                pos.1 += 1;
            }
            _ => panic!(),
        }
    }
    pos
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();

    let (mut itx, mut irx) = mpsc::channel();
    let (mut otx, orx) = mpsc::channel();
    thread::spawn(move || {
        let program = program.trim().split(",").map(|v| v.parse().unwrap());
        let mut program = Program::new(program, &mut irx, &mut otx);
        program.execute();
    });

    let mut visit: VecDeque<Vec<i64>> = VecDeque::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut current = vec![];
    visit.push_back(vec![1]);
    visit.push_back(vec![2]);
    visit.push_back(vec![3]);
    visit.push_back(vec![4]);

    while let Some(path) = visit.pop_front() {
        visited.insert(path_to_pos(&path));
        move_to_start(&current, &mut itx, &orx);

        for mv in path[0..(path.len() - 1)].iter() {
            itx.send(*mv).unwrap();
            if orx.recv().unwrap() != 1 {
                panic!();
            }
        }
        current = Vec::from(&path[0..(path.len() - 1)]);

        itx.send(path[path.len() - 1]).unwrap();
        match orx.recv().unwrap() {
            0 => {}
            1 => {
                current = path.clone();
                for mv in [1, 2, 3, 4] {
                    let mut path = path.clone();
                    path.push(mv);
                    if visited.get(&path_to_pos(&path)).is_none() {
                        visit.push_back(path);
                    }
                }
            }
            2 => {
                println!("part1: {}", path.len());
                break;
            }
            _ => panic!(),
        }
    }
    // Cause intcode to terminate earlier
    itx.send(5);
}
