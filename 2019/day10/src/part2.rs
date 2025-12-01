use core::f64;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    io::BufRead,
};

use ordered_float::OrderedFloat;

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
            let o = (other.1 - base.1) as f64;
            let a = (other.0 - base.0) as f64;
            let norm = (o * o + a * a).sqrt();
            let angle = (o).atan2(-a);
            let angle = if angle >= 0f64 {
                angle
            } else {
                f64::consts::PI * 2f64 + angle
            };
            let angle = OrderedFloat(angle);
            let norm = OrderedFloat(norm);
            asteroid_los
                .entry(base)
                .or_insert_with(|| BTreeMap::new())
                .entry(angle.clone())
                .or_insert(BTreeMap::new())
                .insert(norm, other);
        }
    }

    let mut los_count = asteroid_los
        .iter()
        .map(|(key, val)| (val.len(), *key))
        .collect::<Vec<(usize, (i64, i64))>>();
    los_count.sort();
    let los = asteroid_los.remove(&los_count.last().unwrap().1).unwrap();
    let mut los = los
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<VecDeque<(OrderedFloat<f64>, BTreeMap<OrderedFloat<f64>, (i64, i64)>)>>();
    let mut asteroid = 0;
    while let Some((angle, mut coords)) = los.pop_front() {
        asteroid += 1;
        let coord = coords.pop_first().unwrap();
        if asteroid == 200 {
            println!("{:?}", coord.1 .1 * 100 + coord.1 .0);
        }
        if coords.len() > 0 {
            los.push_back((angle, coords));
        }
    }
}
