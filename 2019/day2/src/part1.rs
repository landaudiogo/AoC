use std::{error::Error, io::BufRead};

#[derive(Debug)]
enum Op {
    Sum,
    Mul,
}

#[derive(Debug)]
struct Instruction {
    operands: [i64; 2],
    operator: Op,
}

impl Instruction {
    fn from_slice(slice: &[i64], program: &[i64]) -> Result<Self, Box<dyn std::error::Error>> {
        let operator = match slice[0] {
            1 => Op::Sum,
            2 => Op::Mul,
            99 => return Err("Fin".into()),
            _ => panic!(),
        };
        let operands = [program[slice[1] as usize], program[slice[2] as usize]];

        Ok(Self { operator, operands })
    }

    fn compute(&self) -> i64 {
        match self.operator {
            Op::Sum => self.operands[0] + self.operands[1],
            Op::Mul => self.operands[0] * self.operands[1],
        }
    }
}

#[derive(Debug)]
struct Program {
    inner: Vec<i64>,
}

impl Program {
    fn new<I: Iterator<Item = i64>>(iter: I) -> Self {
        Self {
            inner: Vec::from_iter(iter),
        }
    }

    pub fn execute(&mut self) {
        for i in (0..self.inner.len()).step_by(4) {
            let instruction = Instruction::from_slice(&self.inner[i..(i + 3)], &self.inner);
            if let Ok(instruction) = instruction {
                println!("{:?} {:?}", instruction, instruction.compute());
                let result_pos = self.inner[i + 3] as usize;
                self.inner[result_pos] = instruction.compute();
            } else {
                break;
            }
        }

        dbg!(self.inner[0]);
    }
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let iter = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
    let mut program = Program::new(iter);
    program.execute();
}
