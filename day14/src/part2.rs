// I could not for the life of me understand this problem, so I looked up what others had come up
// with.
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

static WIDTH: i64 = 101;
static HEIGHT: i64 = 103;

fn safety_factor(robots: &Vec<Robot>) -> u64 {
    let mut quadrants = [0; 4];
    for robot in robots {
        let position = robot.pos;
        if position.0 < WIDTH / 2 && position.1 < HEIGHT / 2 {
            quadrants[0] += 1;
        } else if position.0 < WIDTH / 2 && position.1 > HEIGHT / 2 {
            quadrants[1] += 1;
        } else if position.0 > WIDTH / 2 && position.1 < HEIGHT / 2 {
            quadrants[2] += 1;
        } else if position.0 > WIDTH / 2 && position.1 > HEIGHT / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
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

    let mut min = None;
    for second in 1..=(WIDTH * HEIGHT) {
        for robot in robots.iter_mut() {
            let pos = (robot.pos.0 + robot.vel.0, robot.pos.1 + robot.vel.1);
            let mut pos = (pos.0 % WIDTH, pos.1 % HEIGHT);
            pos.0 = if pos.0 >= 0 { pos.0 } else { WIDTH + pos.0 };
            pos.1 = if pos.1 >= 0 { pos.1 } else { HEIGHT + pos.1 };
            robot.pos = pos;
        }

        let sf = safety_factor(&robots);
        if let Some((min_sf, _)) = min {
            if sf < min_sf {
                min = Some((sf, second));
            }
        } else {
            min = Some((sf, second));
        }
    }

    min.unwrap().0
}
