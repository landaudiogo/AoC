use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    let mut bananas = HashMap::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut seen = HashSet::new();
        let mut secret = line.trim().parse::<u64>().unwrap();

        let secret_string = secret.to_string();
        let bytes = secret_string.as_bytes();
        let digit = (bytes[bytes.len() - 1] as char).to_digit(10).unwrap() as i64;
        let mut prices = Vec::new();
        prices.push(digit);
        for _ in 0..2000 {
            secret = (secret ^ (secret << 6)) % 16777216;
            secret = (secret ^ (secret >> 5)) % 16777216;
            secret = (secret ^ (secret << 11)) % 16777216;
            let secret = secret.to_string();
            let bytes = secret.as_bytes();
            let digit = (bytes[bytes.len() - 1] as char).to_digit(10).unwrap() as i64;
            prices.push(digit);
        }

        let diffs: Vec<i64> = prices.windows(2).map(|v| v[1] - v[0]).collect();
        for (i, quad) in diffs.windows(4).enumerate() {
            if seen.contains(quad) {
                continue;
            }

            *bananas.entry(Vec::from(quad)).or_insert(0) += prices[i + 4];
            seen.insert(quad);
        }

        line.truncate(0);
    }

    let mut highest: Option<(Vec<i64>, i64)> = None;
    for (k, v) in bananas {
        if let Some(ref max) = highest {
            if max.1 < v {
                highest = Some((k, v));
            }
        } else {
            highest = Some((k, v));
        }
    }

    println!("{:?}", highest);
}
