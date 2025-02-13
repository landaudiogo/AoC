use std::io::BufRead;

use regex::Regex;

#[derive(Debug)]
struct Moon {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            pos: (x, y, z),
            vel: (0, 0, 0),
        }
    }

    fn update_gravity(&mut self, other: &Moon) {
        if other.pos.0 > self.pos.0 {
            self.vel.0 += 1;
        } else if other.pos.0 < self.pos.0 {
            self.vel.0 -= 1;
        }

        if other.pos.1 > self.pos.1 {
            self.vel.1 += 1;
        } else if other.pos.1 < self.pos.1 {
            self.vel.1 -= 1;
        }

        if other.pos.2 > self.pos.2 {
            self.vel.2 += 1;
        } else if other.pos.2 < self.pos.2 {
            self.vel.2 -= 1;
        }
    }

    fn update_position(&mut self) {
        self.pos = (
            self.pos.0 + self.vel.0,
            self.pos.1 + self.vel.1,
            self.pos.2 + self.vel.2,
        );
    }

    fn potential(&self) -> i64 {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }

    fn kinetic(&self) -> i64 {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }
}

fn simulate(mut moons: Vec<Moon>, steps: u32) -> Vec<Moon> {
    for _ in 0..steps {
        for i in 0..moons.len() {
            let (left, right) = moons.split_at_mut(i + 1);
            let moon = left.last_mut().unwrap();

            for other in right {
                moon.update_gravity(other);
                other.update_gravity(moon);
            }

            moon.update_position();
        }
    }

    moons
}

pub fn run<B: BufRead>(mut buf: B) -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    let mut moons = Vec::new();
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let moon = line.trim();
        let captures = re.captures(moon).unwrap();
        let (x, y, z) = (&captures[1], &captures[2], &captures[3]);
        let (x, y, z): (i64, i64, i64) = (x.parse()?, y.parse()?, z.parse()?);
        moons.push(Moon::new(x, y, z));
        line.truncate(0);
    }

    let moons = simulate(moons, 1000);
    let total_energy = moons
        .iter()
        .fold(0, |acc, moon| acc + (moon.potential() * moon.kinetic()));

    dbg!(total_energy);

    Ok(())
}
