use console::Term;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::{BufRead, Read, Stdin},
    sync::mpsc,
};

use crate::intcode::{Input, Program};

fn draw(mut elements: BTreeMap<(i64, i64), i64>) -> BTreeMap<(i64, i64), i64> {
    let last_coord = *elements.last_entry().unwrap().key();
    for i in 0..=last_coord.0 {
        for j in 0..=last_coord.1 {
            if let Some(item) = elements.get(&(i, j)) {
                match item {
                    0 => {
                        print!(" ");
                    }
                    1 => {
                        print!("#");
                    }
                    2 => {
                        print!("B");
                    }
                    3 => {
                        print!("_");
                    }
                    4 => {
                        print!("O");
                    }
                    _ => panic!(),
                }
            } else {
                panic!();
            }
        }
        println!();
    }

    elements
}

#[derive(Debug)]
struct ArrowPad {
    term: Term,
}

impl ArrowPad {
    fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }
}

impl Input for ArrowPad {
    fn read_input(&mut self) -> i64 {
        let mut res = 0;
        // loop {
        //     let c = self.term.read_char().unwrap();
        //     match c {
        //         'h' => {
        //             res = -1;
        //             break;
        //         }
        //         'j' => {
        //             res = 0;
        //             break;
        //         }
        //         'k' => {
        //             res = 0;
        //             break;
        //         }
        //         'l' => {
        //             res = 1;
        //             break;
        //         }
        //         'q' => {
        //             panic!();
        //         }
        //         _ => {
        //             continue;
        //         }
        //     }
        // }
        res
    }
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let mut program: Vec<i64> = program
        .trim()
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    program[0] = 2;

    // analysed with lldb
    let item_start = 639;
    let width = 42;
    let height = 24;
    for i in 0..width {
        program[item_start + 42 * 22 + i] = 3;
    }

    let (mut tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut arrowpad = ArrowPad::new();
        let mut program = Program::new(program.into_iter(), &mut arrowpad, &mut tx);
        program.execute();
    });

    // let mut elements = BTreeMap::new();
    while let (Ok(y), Ok(x), Ok(item)) = (rx.recv(), rx.recv(), rx.recv()) {
        if y == -1 && x == 0 {
            println!("score: {item}");
            continue;
        }

        // elements.insert((x, y), item);
        // elements = draw(elements);
    }
}
