use std::{collections::HashMap, io::BufRead};

use regex::Regex;

#[derive(Debug)]
struct Moon {
    ini: (i64, i64, i64),
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            ini: (x, y, z),
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

    fn is_x_initial(&self) -> bool {
        (self.pos.0 == self.ini.0) && self.vel.0 == 0
    }

    fn is_y_initial(&self) -> bool {
        (self.pos.1 == self.ini.1) && self.vel.1 == 0
    }
    fn is_z_initial(&self) -> bool {
        (self.pos.2 == self.ini.2) && self.vel.2 == 0
    }
}

fn find_loop(mut moons: Vec<Moon>) -> HashMap<String, usize> {
    let mut loops = HashMap::new();
    let mut step = 0;
    while loops.len() < 3 {
        for i in 0..moons.len() {
            let (left, right) = moons.split_at_mut(i + 1);
            let moon = left.last_mut().unwrap();

            for other in right {
                moon.update_gravity(other);
                other.update_gravity(moon);
            }

            moon.update_position();
        }

        let all_x_ini = moons
            .iter()
            .fold(true, |acc, moon| acc && moon.is_x_initial());
        if all_x_ini {
            loops.entry("x".into()).or_insert(step + 1);
        }
        let all_y_ini = moons
            .iter()
            .fold(true, |acc, moon| acc && moon.is_y_initial());
        if all_y_ini {
            loops.entry("y".into()).or_insert(step + 1);
        }
        let all_z_ini = moons
            .iter()
            .fold(true, |acc, moon| acc && moon.is_z_initial());
        if all_z_ini {
            loops.entry("z".into()).or_insert(step + 1);
        }

        step += 1;
    }
    loops
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

    let loops: Vec<usize> = find_loop(moons).values().map(|v| *v).collect();
    let epoch = lcm(&loops);
    dbg!(epoch);

    Ok(())
}
