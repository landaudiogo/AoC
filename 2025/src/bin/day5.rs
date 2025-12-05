use std::{
    collections::BTreeMap,
    io::{self, BufRead, Read},
};

use anyhow::Result;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    solve(&input);
}

fn solve(input: &str) -> Result<()> {
    let mut fresh_ingredients: BTreeMap<u64, u64> = BTreeMap::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let range = line.trim();
        if line.len() == 0 {
            break;
        }

        let mut chunks = range.split("-");
        let mut start: u64 = chunks.next().unwrap().parse().unwrap();
        let mut end: u64 = chunks.next().unwrap().parse().unwrap();

        let prev = fresh_ingredients
            .range(..=start)
            .last()
            .map(|(k, v)| (*k, *v));
        if let Some((prev_start, prev_end)) = prev {
            if start >= prev_start && start <= prev_end {
                start = prev_start;
                fresh_ingredients.remove(&prev_start);
            }

            if end < prev_end {
                end = prev_end
            }
        }

        let next = fresh_ingredients
            .range(start..)
            .next()
            .map(|(k, v)| (*k, *v));
        if let Some((next_start, next_end)) = next {
            if end >= next_start {
                end = u64::max(end, next_end);
                fresh_ingredients.remove(&next_start);
            }
        }

        fresh_ingredients.insert(start, end);
    }

    let mut p1 = 0;
    for line in lines {
        let ingredient: u64 = line.trim().parse().unwrap();
        match fresh_ingredients.range(..=ingredient).last() {
            None => {}
            Some((start, end)) => {
                if *end >= ingredient {
                    p1 += 1;
                }
            }
        }
    }
    println!("p1: {p1}");

    let mut p2 = 0;
    for (start, end) in fresh_ingredients {
        p2 += (end - start) + 1;
    }
    println!("p2: {p2}");

    Ok(())
}
