use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    io::BufRead,
};

#[derive(Debug)]
struct Machine {
    prize: (i64, i64),
    a: (i64, i64),
    b: (i64, i64),
}

impl Machine {
    fn new<B: BufRead>(mut buf: B) -> Option<Self> {
        let mut line = String::new();
        let button_re = Regex::new(r"\+(\d+),.*\+(\d+)").unwrap();
        let prize_re = Regex::new(r"=(\d+),.*=(\d+)").unwrap();
        let mut a = (0, 0);
        let mut b = (0, 0);
        let mut prize = (0, 0);
        let mut i = 0;

        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 && i == 0 {
                return None;
            }

            if len == 1 || len == 0 {
                break;
            }

            match i {
                0 => {
                    let capture = button_re.captures_iter(&line).next().unwrap();
                    a = (
                        capture[1].parse::<i64>().unwrap(),
                        capture[2].parse::<i64>().unwrap(),
                    );
                }
                1 => {
                    let capture = button_re.captures_iter(&line).next().unwrap();
                    b = (
                        capture[1].parse::<i64>().unwrap(),
                        capture[2].parse::<i64>().unwrap(),
                    );
                }
                2 => {
                    let capture = prize_re.captures_iter(&line).next().unwrap();
                    prize = (
                        capture[1].parse::<i64>().unwrap(),
                        capture[2].parse::<i64>().unwrap(),
                    );
                }
                _ => {
                    panic! {}
                }
            }
            i += 1;
            line.truncate(0);
        }
        Some(Self {
            a,
            b,
            prize: (prize.0, prize.1),
        })
    }

    fn solve(&self) -> Option<i64> {
        let mut prize = self.prize;
        let mut atokens = 0;
        let mut potential = BTreeSet::new();

        while prize.0 >= 0 && prize.1 >= 0 {
            if Self::prize_divisible(prize, self.b) {
                let (divx, divy) = (prize.0 / self.b.0, prize.1 / self.b.1);
                if divx == divy {
                    let btokens = divx;
                    potential.insert(atokens * 3 + btokens * 1);
                }
            }
            prize = (prize.0 - self.a.0, prize.1 - self.a.1);
            atokens += 1;
        }

        if potential.len() == 0 {
            return None;
        }

        Some(potential.pop_first().unwrap())
    }

    fn prize_divisible(prize: (i64, i64), token: (i64, i64)) -> bool {
        (prize.0 % token.0 == 0) && (prize.1 % token.1 == 0)
    }
}

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut total = 0;
    while let Some(machine) = Machine::new(&mut buf) {
        if let Some(tokens) = machine.solve() {
            total += tokens;
        }
        line.truncate(0);
    }

    total as u64
}
