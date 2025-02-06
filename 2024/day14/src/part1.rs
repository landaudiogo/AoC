use std::io::BufRead;

use regex::Regex;

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

impl Robot {
    fn new(line: &str) -> Self {
        let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        Self {
            pos: (
                captures[1].parse::<i64>().unwrap(),
                captures[2].parse::<i64>().unwrap(),
            ),
            vel: (
                captures[3].parse::<i64>().unwrap(),
                captures[4].parse::<i64>().unwrap(),
            ),
        }
    }
}

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut robots = Vec::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        robots.push(Robot::new(&line));

        line.truncate(0);
    }

    let seconds = 100;
    for robot in robots.iter_mut() {
        robot.pos = (
            robot.pos.0 + seconds * robot.vel.0,
            robot.pos.1 + seconds * robot.vel.1,
        );
    }
    let width = 101;
    let height = 103;

    let grid_positions = robots
        .iter()
        .map(|r| {
            let mut pos = (r.pos.0 % width, r.pos.1 % height);
            pos.0 = if pos.0 >= 0 { pos.0 } else { width + pos.0 };
            pos.1 = if pos.1 >= 0 { pos.1 } else { height + pos.1 };
            pos
        })
        .collect::<Vec<(i64, i64)>>();

    let mut quadrants = [0; 4];
    for position in grid_positions {
        if position.0 < width / 2 && position.1 < height / 2 {
            quadrants[0] += 1;
        } else if position.0 < width / 2 && position.1 > height / 2 {
            quadrants[1] += 1;
        } else if position.0 > width / 2 && position.1 < height / 2 {
            quadrants[2] += 1;
        } else if position.0 > width / 2 && position.1 > height / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}
