use std::{fs, io};

fn main() {
    p1();
    p2();
}

fn p1() {
    let mut pos = 50;
    let mut count = 0;
    let content = fs::read_to_string("./inputs/day1.txt").unwrap();

    for line in content.lines() {
        let mut chars = line.chars();
        let dir = match chars.next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => todo!(),
        };

        let magnitude: i64 = chars.collect::<String>().parse().unwrap();
        pos = (pos + dir * magnitude) % 100;
        pos = if pos < 0 { 100 + pos } else { pos };
        if pos == 0 {
            count += 1;
        }
    }
    println!("p1: {count}");
}

fn p2() {
    let mut pos = 50;
    let mut count = 0;
    let content = fs::read_to_string("./inputs/day1.txt").unwrap();

    for line in content.lines() {
        let mut chars = line.chars();
        let (dir, ticks_to_zero) = match chars.next().unwrap() {
            'L' => (-1, pos),
            'R' => (1, 100 - pos),
            _ => todo!(),
        };

        let mut magnitude: i64 = chars.collect::<String>().parse().unwrap();
        if magnitude - ticks_to_zero >= 0 {
            let reached_zero = if ticks_to_zero > 0 { 1 } else { 0 };
            let wraps = (magnitude - ticks_to_zero) / 100;
            count += reached_zero + wraps;
            magnitude = (magnitude - ticks_to_zero) - 100 * wraps;
            pos = 0;
        }

        pos = pos + dir * magnitude;
        pos = if pos >= 0 { pos % 100 } else { 100 + pos };
    }

    println!("p2: {count}");
}
