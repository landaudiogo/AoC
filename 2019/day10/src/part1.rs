use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

pub fn run<B: BufRead>(mut buf: B) {
    let mut asteroid_locations = Vec::new();
    let mut asteroid_los = HashMap::new();
    let mut line = String::new();
    let mut row = 0;

    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        line.trim().chars().enumerate().for_each(|(col, v)| {
            if v == '#' {
                asteroid_locations.push((row as i64, col as i64));
            }
        });

        row += 1;
        line.truncate(0);
    }

    for i in 0..asteroid_locations.len() {
        let base = asteroid_locations[i];
        for j in 0..asteroid_locations.len() {
            if i == j {
                continue;
            }
            let other = asteroid_locations[j];
            let norm =
                (((other.0 - base.0).pow(2) + (other.1 - base.1).pow(2)) as f64).sqrt() as f64;
            let normalised = (
                format!("{:.5}", (other.0 - base.0) as f64 / norm),
                format!("{:.5}", (other.1 - base.1) as f64 / norm),
            );
            let insert_result = asteroid_los
                .entry(base)
                .or_insert_with(|| HashSet::new())
                .insert(normalised.clone());
        }
    }

    let mut los_count = asteroid_los
        .iter()
        .map(|(key, val)| (val.len(), *key))
        .collect::<Vec<(usize, (i64, i64))>>();
    los_count.sort();
    for (count, coord) in los_count {
        println!("{:?}: {:?}", coord, count);
    }
}
