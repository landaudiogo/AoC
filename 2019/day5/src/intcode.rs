use std::{
    error::Error,
    io::{BufRead, Write},
};

#[derive(Debug)]
enum Instruction<'a, 'b> {
    Sum {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    Mul {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    Jz {
        operands: (i64, i64),
        output: &'b mut usize,
    },
    Jnz {
        operands: (i64, i64),
        output: &'b mut usize,
    },
    Lt {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    Eq {
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

fn parameter_from_pc(program: &[i64], pc: usize, parameter: u32) -> i64 {
    match ParameterType::from_opcode(program[pc], parameter + 3) {
        ParameterType::Ref => program[program[pc + parameter as usize + 1] as usize],
        ParameterType::Val => program[pc + parameter as usize + 1],
    }
}

impl<'a, 'b> Instruction<'a, 'b> {
    fn from_pc(
        pc: &'b mut usize,
        program: &'a mut [i64],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let operator = program[*pc] % 100;
        match operator {
            1 => {
                let x = parameter_from_pc(program, *pc, 0);
                let y = parameter_from_pc(program, *pc, 1);

                let instruction = Self::Sum {
                    operands: (x, y),
                    output: &mut program[program[*pc + 3] as usize],
                };
                *pc += 4;
                Ok(instruction)
            }
            2 => {
                let x = parameter_from_pc(program, *pc, 0);
                let y = parameter_from_pc(program, *pc, 1);

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
                let x = parameter_from_pc(program, *pc, 0);
                let instruction = Self::Out(x);
                *pc += 2;
                Ok(instruction)
            }
            5 => {
                let x = parameter_from_pc(program, *pc, 0);
                let y = parameter_from_pc(program, *pc, 1);

                let instruction = Self::Jnz {
                    operands: (x, y),
                    output: pc,
                };
                Ok(instruction)
            }
            6 => {
                let x = parameter_from_pc(program, *pc, 0);
                let y = parameter_from_pc(program, *pc, 1);

                let instruction = Self::Jz {
                    operands: (x, y),
                    output: pc,
                };
                Ok(instruction)
            }
            7 => {
                let x = parameter_from_pc(program, *pc, 0);
                let y = parameter_from_pc(program, *pc, 1);

                let instruction = Self::Lt {
                    operands: (x, y),
                    output: &mut program[program[*pc + 3] as usize],
                };
                *pc += 4;
                Ok(instruction)
            }
            8 => {
                let x = parameter_from_pc(program, *pc, 0);
                let y = parameter_from_pc(program, *pc, 1);

                let instruction = Self::Eq {
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
            op => Err(format!("Invalid opcode {}", op).into()),
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
            Self::Jnz { operands, output } => {
                if operands.0 != 0 {
                    *output = operands.1 as usize;
                } else {
                    *output += 3;
                }
                Ok(())
            }
            Self::Jz { operands, output } => {
                if operands.0 == 0 {
                    *output = operands.1 as usize;
                } else {
                    *output += 3;
                }
                Ok(())
            }
            Self::Lt { operands, output } => {
                *output = (operands.0 < operands.1) as i64;
                Ok(())
            }
            Self::Eq { operands, output } => {
                *output = (operands.0 == operands.1) as i64;
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
