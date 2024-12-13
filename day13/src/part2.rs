use nalgebra::{Matrix2, Vector2};
use regex::Regex;
use std::io::BufRead;

#[derive(Debug)]
struct Machine {
    prize: (f64, f64),
    a: (f64, f64),
    b: (f64, f64),
}

impl Machine {
    fn new<B: BufRead>(mut buf: B) -> Option<Self> {
        let mut line = String::new();
        let button_re = Regex::new(r"\+(\d+),.*\+(\d+)").unwrap();
        let prize_re = Regex::new(r"=(\d+),.*=(\d+)").unwrap();
        let mut a = (0.0, 0.0);
        let mut b = (0.0, 0.0);
        let mut prize = (0.0, 0.0);
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
                        capture[1].parse::<f64>().unwrap(),
                        capture[2].parse::<f64>().unwrap(),
                    );
                }
                1 => {
                    let capture = button_re.captures_iter(&line).next().unwrap();
                    b = (
                        capture[1].parse::<f64>().unwrap(),
                        capture[2].parse::<f64>().unwrap(),
                    );
                }
                2 => {
                    let capture = prize_re.captures_iter(&line).next().unwrap();
                    prize = (
                        capture[1].parse::<f64>().unwrap(),
                        capture[2].parse::<f64>().unwrap(),
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
            prize: (prize.0 + 10000000000000.0, prize.1 + 10000000000000.0),
        })
    }

    fn solve(&self) -> Option<u64> {
        let a = Matrix2::new(self.a.0, self.b.0, self.a.1, self.b.1);
        let a_inv = a.try_inverse().unwrap();
        let b = Vector2::new(self.prize.0, self.prize.1);
        let x = a_inv * b;
        let x_ = Vector2::new(x[0].round(), x[1].round());
        if (a * x - a * x_).magnitude_squared() < 1e-3 {
            return Some(x_[0] as u64 * 3 + x_[1] as u64 * 1);
        }

        None
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
