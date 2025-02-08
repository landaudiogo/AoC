use std::{
    error::Error,
    io::{BufRead, Write},
};

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
    In(&'a mut i64),
    Out(i64),
    Fin,
}

enum ParameterType {
    Ref,
    Val,
}

impl ParameterType {
    fn from_opcode(opcode: i64, digit: u32) -> Self {
        match (opcode / 10i64.pow(digit - 1)) % 10 {
            0 => Self::Ref,
            1 => Self::Val,
            _ => panic!("Invalid parameter type"),
        }
    }
}

impl<'a> Instruction<'a> {
    fn from_pc(pc: &mut usize, program: &'a mut [i64]) -> Result<Self, Box<dyn std::error::Error>> {
        let operator = program[*pc] % 100;
        match operator {
            1 => {
                let x = match ParameterType::from_opcode(program[*pc], 3) {
                    ParameterType::Ref => program[program[*pc + 1] as usize],
                    ParameterType::Val => program[*pc + 1],
                };

                let y = match ParameterType::from_opcode(program[*pc], 4) {
                    ParameterType::Ref => program[program[*pc + 2] as usize],
                    ParameterType::Val => program[*pc + 2],
                };

                let instruction = Self::Sum {
                    operands: (x, y),
                    output: &mut program[program[*pc + 3] as usize],
                };
                *pc += 4;
                Ok(instruction)
            }
            2 => {
                let x = match ParameterType::from_opcode(program[*pc], 3) {
                    ParameterType::Ref => program[program[*pc + 1] as usize],
                    ParameterType::Val => program[*pc + 1],
                };

                let y = match ParameterType::from_opcode(program[*pc], 4) {
                    ParameterType::Ref => program[program[*pc + 2] as usize],
                    ParameterType::Val => program[*pc + 2],
                };
                let instruction = Self::Mul {
                    operands: (x, y),
                    output: &mut program[program[*pc + 3] as usize],
                };
                *pc += 4;
                Ok(instruction)
            }
            3 => {
                let instruction = Self::In(&mut program[program[*pc + 1] as usize]);
                *pc += 2;
                Ok(instruction)
            }
            4 => {
                let instruction = Self::Out(program[program[*pc + 1] as usize]);
                *pc += 2;
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
            Self::In(address) => {
                let mut line = String::new();
                print!("Provide input: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut line).unwrap();
                *address = line.trim().parse::<i64>().unwrap();
                Ok(())
            }
            Self::Out(val) => {
                println!("{}", val);
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
}
