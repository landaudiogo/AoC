use std::{error::Error, io::BufRead};

#[derive(Debug)]
enum Instruction<'a> {
    Sum {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    Mul {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    Fin,
}

impl<'a> Instruction<'a> {
    fn from_pc(pc: &mut usize, program: &'a mut [i64]) -> Result<Self, Box<dyn std::error::Error>> {
        match program[*pc] {
            1 => {
                let x = program[program[*pc + 1] as usize];
                let y = program[program[*pc + 2] as usize];
                let instruction = Self::Sum {
                    operands: (x, y),
                    output: &mut program[program[*pc + 3] as usize],
                };
                *pc += 4;
                Ok(instruction)
            }
            2 => {
                let x = program[program[*pc + 1] as usize];
                let y = program[program[*pc + 2] as usize];
                let instruction = Self::Mul {
                    operands: (x, y),
                    output: &mut program[program[*pc + 3] as usize],
                };
                *pc += 4;
                Ok(instruction)
            }
            99 => {
                *pc += 1;
                Ok(Self::Fin)
            }
            _ => Err("Invalid opcode".into()),
        }
    }

    fn compute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Sum { operands, output } => {
                *output = operands.0 + operands.1;
                Ok(())
            }
            Self::Mul { operands, output } => {
                *output = operands.0 * operands.1;
                Ok(())
            }
            Self::Fin => Err("Fin".into()),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    inner: Vec<i64>,
    pc: usize,
}

impl Program {
    fn new<I: Iterator<Item = i64>>(iter: I) -> Self {
        Self {
            inner: Vec::from_iter(iter),
            pc: 0,
        }
    }

    pub fn execute(&mut self) {
        while self.pc < self.inner.len() {
            let instruction = Instruction::from_pc(&mut self.pc, &mut self.inner).unwrap();
            if let Err(_e) = instruction.compute() {
                break;
            }
        }
    }
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut program = String::new();
    buf.read_to_string(&mut program).unwrap();
    let iter = program.trim().split(",").map(|v| v.parse::<i64>().unwrap());
    let mut program = Program::new(iter);
    program.execute();
    dbg!(program.inner[0]);
}
