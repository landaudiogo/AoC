use std::{collections::HashSet, io::BufRead, sync::mpsc, thread};

use crate::intcode::Program;

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program);
    let (_itx, mut irx) = mpsc::channel();
    let (mut otx, orx) = mpsc::channel();
    thread::spawn(move || {
        let program = program.trim().split(",").map(|v| v.parse().unwrap());
        let mut program = Program::new(program, &mut irx, &mut otx);
        program.execute();
    });

    let mut map = Vec::new();
    map.push(Vec::new());
    while let Ok(v) = orx.recv() {
        let row = map.last_mut().unwrap();
        match v {
            35 => {
                row.push('#');
            }
            46 => {
                row.push('.');
            }
            10 => {
                map.push(Vec::new());
            }
            v => row.push(char::from_u32(v as u32).unwrap()),
            _ => panic!(),
        }
    }

    let map: Vec<Vec<char>> = map.into_iter().filter(|r| r.len() > 0).collect();
    let height = map.len() as usize;
    let width = map.first().unwrap().len() as usize;
    let mut intersections = HashSet::new();
    for i in 0..height {
        for j in 0..width {
            if map[i][j] != '#' {
                continue;
            }

            let dirs: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            let total = dirs.into_iter().fold(0, |acc, dir| {
                let n = (i as i64 + dir.0, j as i64 + dir.1);
                if n.0 < 0 || n.0 >= height as i64 || n.1 < 0 || n.1 >= width as i64 {
                    return acc;
                }
                let n = (n.0 as usize, n.1 as usize);
                if map[n.0][n.1] == '#' {
                    acc + 1
                } else {
                    acc
                }
            });
            if total == 4 {
                intersections.insert((i, j));
            }
        }
    }

    let mut score = 0;
    for inter in intersections {
        score += inter.0 * inter.1;
    }

    println!("p1: {}", score);
}
