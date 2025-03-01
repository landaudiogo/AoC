use std::{
    collections::VecDeque,
    fmt::Debug,
    io::Write,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Debug)]
enum Instruction<'a, In: Input + Debug, Out: Output + Debug> {
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
        output: &'a mut usize,
    },
    Jnz {
        operands: (i64, i64),
        output: &'a mut usize,
    },
    Rb {
        operand: i64,
        output: &'a mut usize,
    },
    Lt {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    Eq {
        operands: (i64, i64),
        output: &'a mut i64,
    },
    In {
        input: &'a mut In,
        output: &'a mut i64,
    },
    Out {
        output: &'a mut Out,
        val: i64,
    },
    Fin,
}

enum ParameterType {
    Ref,
    Val,
    Rel,
}

impl ParameterType {
    fn from_opcode(opcode: i64, digit: u32) -> Self {
        match (opcode / 10i64.pow(digit - 1)) % 10 {
            0 => Self::Ref,
            1 => Self::Val,
            2 => Self::Rel,
            _ => panic!("Invalid parameter type"),
        }
    }
}

fn parameter_from_pc(program: &mut [i64], pc: usize, rb: usize, parameter_idx: u32) -> &mut i64 {
    match ParameterType::from_opcode(program[pc], parameter_idx + 3) {
        ParameterType::Ref => &mut program[program[pc + parameter_idx as usize + 1] as usize],
        ParameterType::Val => &mut program[pc + parameter_idx as usize + 1],
        ParameterType::Rel => {
            &mut program[(rb as i64 + program[pc + parameter_idx as usize + 1]) as usize]
        }
    }
}

impl<'a, In, Out> Instruction<'a, In, Out>
where
    In: Input + Debug,
    Out: Output + Debug,
{
    fn from_pc(
        pc: &'a mut usize,
        rb: &'a mut usize,
        program: &'a mut [i64],
        input: &'a mut In,
        output: &'a mut Out,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let operator = program[*pc] % 100;
        match operator {
            1 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);
                let y = *parameter_from_pc(program, *pc, *rb, 1);
                let output = parameter_from_pc(program, *pc, *rb, 2);

                let instruction = Self::Sum {
                    operands: (x, y),
                    output,
                };
                *pc += 4;
                Ok(instruction)
            }
            2 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);
                let y = *parameter_from_pc(program, *pc, *rb, 1);
                let output = parameter_from_pc(program, *pc, *rb, 2);

                let instruction = Self::Mul {
                    operands: (x, y),
                    output,
                };
                *pc += 4;
                Ok(instruction)
            }
            3 => {
                let output = parameter_from_pc(program, *pc, *rb, 0);
                let instruction = Self::In { input, output };
                *pc += 2;
                Ok(instruction)
            }
            4 => {
                let val = *parameter_from_pc(program, *pc, *rb, 0);
                let instruction = Self::Out { val, output };
                *pc += 2;
                Ok(instruction)
            }
            5 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);
                let y = *parameter_from_pc(program, *pc, *rb, 1);

                let instruction = Self::Jnz {
                    operands: (x, y),
                    output: pc,
                };
                Ok(instruction)
            }
            6 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);
                let y = *parameter_from_pc(program, *pc, *rb, 1);

                let instruction = Self::Jz {
                    operands: (x, y),
                    output: pc,
                };
                Ok(instruction)
            }
            7 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);
                let y = *parameter_from_pc(program, *pc, *rb, 1);
                let output = parameter_from_pc(program, *pc, *rb, 2);

                let instruction = Self::Lt {
                    operands: (x, y),
                    output,
                };
                *pc += 4;
                Ok(instruction)
            }
            8 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);
                let y = *parameter_from_pc(program, *pc, *rb, 1);
                let output = parameter_from_pc(program, *pc, *rb, 2);

                let instruction = Self::Eq {
                    operands: (x, y),
                    output,
                };
                *pc += 4;
                Ok(instruction)
            }
            9 => {
                let x = *parameter_from_pc(program, *pc, *rb, 0);

                let instruction = Self::Rb {
                    operand: x,
                    output: rb,
                };
                *pc += 2;

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
            Self::In { input, output } => {
                *output = input.read_input();
                Ok(())
            }
            Self::Out { val, output } => {
                output.write_output(val);
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
            Self::Rb { operand, output } => {
                *output = ((*output as i64) + operand) as usize;
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

pub trait Input {
    fn read_input(&mut self) -> i64;
}

pub trait Output {
    fn write_output(&mut self, val: i64);
}

impl Input for std::io::Stdin {
    fn read_input(&mut self) -> i64 {
        let mut line = String::new();
        // print!("Provide Input: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    }
}

impl Output for std::io::Stdout {
    fn write_output(&mut self, val: i64) {
        println!("{}", val);
    }
}

impl Output for Sender<i64> {
    fn write_output(&mut self, val: i64) {
        self.send(val).unwrap();
    }
}

impl Input for Receiver<i64> {
    fn read_input(&mut self) -> i64 {
        self.recv().unwrap()
    }
}

pub struct Program<'a, 'b, In: Input, Out: Output> {
    inner: Vec<i64>,
    pc: usize,
    rb: usize,
    input: &'a mut In,
    output: &'b mut Out,
}

impl<'a, 'b, In: Input + Debug, Out: Output + Debug> Program<'a, 'b, In, Out> {
    pub fn new<I: Iterator<Item = i64>>(iter: I, input: &'a mut In, output: &'b mut Out) -> Self {
        let p = Vec::from_iter(iter);
        let mut inner = vec![0; 2 << 20];
        inner[..p.len()].clone_from_slice(&p);
        Self {
            inner,
            pc: 0,
            rb: 0,
            input,
            output,
        }
    }

    pub fn execute(&mut self) {
        while self.pc < self.inner.len() {
            let instruction = Instruction::from_pc(
                &mut self.pc,
                &mut self.rb,
                &mut self.inner,
                self.input,
                self.output,
            )
            .unwrap();
            if let Err(_e) = instruction.compute() {
                println!("{:?}", _e);
                break;
            }
        }
    }
}
