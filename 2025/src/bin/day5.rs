use std::{collections::BTreeMap, io};

use anyhow::Result;

fn main() {
    p1();
}

fn p1() -> Result<()> {
    let mut fresh_ingredients: BTreeMap<u64, u64> = BTreeMap::new();
    for line in io::stdin().lines() {
        let line = line?;
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

    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let ingredient: u64 = line.trim().parse().unwrap();
        match fresh_ingredients.range(..=ingredient).last() {
            None => {}
            Some((start, end)) => {
                if *end >= ingredient {
                    total += 1;
                }
            }
        }
    }
    println!("p1: {total}");

    Ok(())
}
