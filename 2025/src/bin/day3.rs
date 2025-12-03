use std::{collections::BTreeMap, fs};

fn main() {
    p1();
    p2();
}

fn solve(batteries: &[i64], digits: usize) -> i64 {
    let mut battery = Vec::new();
    for digit in 0..digits {
        let mut max = None;
        let next = battery.last().map(|(_, idx)| *idx as i64).unwrap_or(-1) + 1;
        for (i, val) in batteries[(next as usize)..batteries.len() - (digits - digit - 1)]
            .iter()
            .enumerate()
        {
            max = match max {
                None => Some((val, next + i as i64)),
                Some(max) => {
                    if val > max.0 {
                        Some((val, next + i as i64))
                    } else {
                        Some(max)
                    }
                }
            };
        }
        battery.push(max.unwrap());
    }

    let mut jolts = 0;
    for (idx, v) in battery.into_iter().enumerate() {
        jolts += *v.0 * 10_i64.pow((digits - idx - 1) as u32);
    }
    jolts
}

fn p1() {
    let content = fs::read_to_string("./inputs/day3.txt").unwrap();
    let mut total = 0;
    for line in content.lines() {
        let values: Vec<i64> = line.chars().map(|c| c as i64 - '0' as i64).collect();
        total += solve(&values, 2);
    }
    println!("p1: {:?}", total);
}

fn p2() {
    let content = fs::read_to_string("./inputs/day3.txt").unwrap();
    let mut total = 0;
    for line in content.lines() {
        let values: Vec<i64> = line.chars().map(|c| c as i64 - '0' as i64).collect();
        total += solve(&values, 12);
    }
    println!("p2: {:?}", total);
}
