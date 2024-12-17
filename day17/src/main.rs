use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Opcode {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Combo(u64),
    Literal(u64),
}

impl Operand {
    fn get_literal(&self, ax: u64, bx: u64, cx: u64) -> u64 {
        match &self {
            Self::Literal(literal) => *literal,
            Self::Combo(val) => {
                let val = *val;
                if val <= 3 {
                    val
                } else if val == 4 {
                    ax
                } else if val == 5 {
                    bx
                } else if val == 6 {
                    cx
                } else {
                    panic!()
                }
            }
        }
    }
}

impl From<(Opcode, u64)> for Operand {
    fn from(value: (Opcode, u64)) -> Self {
        match value.0 {
            Opcode::Adv => Self::Combo(value.1),
            Opcode::Bxl => Self::Literal(value.1),
            Opcode::Bst => Self::Combo(value.1),
            Opcode::Jnz => Self::Literal(value.1),
            Opcode::Bxc => Self::Literal(value.1),
            Opcode::Out => Self::Combo(value.1),
            Opcode::Bdv => Self::Combo(value.1),
            Opcode::Cdv => Self::Combo(value.1),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl From<(u64, u64)> for Instruction {
    fn from(value: (u64, u64)) -> Self {
        let opcode = value.0;
        let opcode = Opcode::from(opcode);
        let operand = Operand::from((opcode, value.1));
        Self { opcode, operand }
    }
}

#[derive(Debug)]
struct Program {
    ip: usize,
    ax: u64,
    bx: u64,
    cx: u64,
    instructions: Vec<Instruction>,
    output: Vec<u64>,
    raw_program: Vec<u64>,
}

impl Program {
    fn new<B: BufRead>(mut buf: B) -> Self {
        let mut ax = None;
        let mut bx = None;
        let mut cx = None;
        let mut line = String::new();
        let re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 1 {
                break;
            }

            let captures = re.captures(&line).unwrap();
            match &captures[1] {
                "A" => ax = Some(captures[2].parse::<u64>().unwrap()),
                "B" => bx = Some(captures[2].parse::<u64>().unwrap()),
                "C" => cx = Some(captures[2].parse::<u64>().unwrap()),
                _ => panic!(),
            }

            line.truncate(0);
        }

        let mut instructions = Vec::new();
        let mut raw_program = Vec::new();
        let re = Regex::new(r"Program: (.*)").unwrap();
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            let program = &re.captures(&line).unwrap()[1];
            let mut program = program.split(",");
            while let (Some(opcode), Some(operand)) = (program.next(), program.next()) {
                let opcode = opcode.parse::<u64>().unwrap();
                raw_program.push(opcode);
                let operand = operand.parse::<u64>().unwrap();
                raw_program.push(operand);
                let instruction = Instruction::from((opcode, operand));
                instructions.push(instruction);
            }

            line.truncate(0);
        }

        Self {
            instructions,
            raw_program,
            ip: 0,
            ax: ax.unwrap(),
            bx: bx.unwrap(),
            cx: cx.unwrap(),
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            let instruction = &self.instructions[self.ip];
            match (&instruction.opcode, &instruction.operand) {
                (Opcode::Adv, operand) => {
                    self.ax =
                        self.ax / 2_u64.pow(operand.get_literal(self.ax, self.bx, self.cx) as u32);
                }
                (Opcode::Bdv, operand) => {
                    self.bx =
                        self.ax / 2_u64.pow(operand.get_literal(self.ax, self.bx, self.cx) as u32)
                }
                (Opcode::Cdv, operand) => {
                    self.cx =
                        self.ax / 2_u64.pow(operand.get_literal(self.ax, self.bx, self.cx) as u32)
                }
                (Opcode::Out, operand) => {
                    self.output
                        .push(operand.get_literal(self.ax, self.bx, self.cx) % 8);
                }
                (Opcode::Bxl, Operand::Literal(operand)) => {
                    self.bx = self.bx ^ operand;
                }
                (Opcode::Bst, operand) => {
                    self.bx = operand.get_literal(self.ax, self.bx, self.cx) % 8;
                }
                (Opcode::Jnz, Operand::Literal(operand)) => {
                    if self.ax != 0 {
                        self.ip = *operand as usize;
                        continue;
                    }
                }
                (Opcode::Bxc, _) => {
                    self.bx = self.bx ^ self.cx;
                }
                _ => panic!(),
            }
            self.ip += 1;
        }
    }
}

fn main() {
    let buf = BufReader::new(File::open("input").unwrap());
    let mut program = Program::new(buf);
    let program_len = program.raw_program.len();

    let mut visit = VecDeque::new();
    visit.push_back(0);
    while let Some(next) = visit.pop_front() {
        for i in next * 8..next * 8 + 8 {
            program.ax = i;
            program.bx = 0;
            program.cx = 0;
            program.ip = 0;
            program.output = Vec::new();
            program.run();

            let output_len = program.output.len();
            if program.output == program.raw_program[program_len - output_len..program_len] {
                if output_len == program_len {
                    println!("{i} -> {:?}", program.output);
                    return;
                }
                visit.push_back(i);
            }
        }
    }
}
