use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
    ops::Bound::{self, Excluded, Included},
};

#[derive(Debug)]
enum Wire {
    Start,
    End,
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    buf.read_line(&mut line).unwrap();

    let mut vertical = BTreeMap::new();
    let mut horizontal = BTreeMap::new();
    let mut pos = (0, 0);

    for action in line.trim().split(",") {
        let mut action = action.chars();
        let direction = action.next().unwrap();
        let distance = action.collect::<String>().parse::<i64>().unwrap();
        match direction {
            'U' => {
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::End);
                pos = (pos.0 - distance, pos.1);
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::Start);
            }
            'D' => {
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::Start);
                pos = (pos.0 + distance, pos.1);
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::End);
            }
            'R' => {
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::Start);
                pos = (pos.0, pos.1 + distance);
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::End);
            }
            'L' => {
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::End);
                pos = (pos.0, pos.1 - distance);
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::Start);
            }
            _ => panic!(),
        }
    }

    line.truncate(0);
    buf.read_line(&mut line).unwrap();
    let mut pos = (0, 0);
    let mut mh_distances = Vec::new();

    for action in line.trim().split(",") {
        let mut action = action.chars();
        let direction = action.next().unwrap();
        let distance = action.collect::<String>().parse::<i64>().unwrap();
        match direction {
            'U' => {
                let i1 = pos.0;
                let i2 = pos.0 - distance;
                let j = pos.1;
                for (i, m) in horizontal.range((Included(i2), Included(i1))) {
                    let prev = m.range((Bound::Unbounded, Bound::Included(j))).next_back();
                    let next = m.range((Bound::Included(j), Bound::Unbounded)).next();
                    if let (Some((_, Wire::Start)), Some((_, Wire::End))) = (prev, next) {
                        mh_distances.push(i.abs() + j.abs());
                    }
                }
                pos = (i2, j);
            }
            'D' => {
                let i1 = pos.0;
                let i2 = pos.0 + distance;
                let j = pos.1;
                for (i, m) in horizontal.range((Included(i1), Included(i2))) {
                    let prev = m.range((Bound::Unbounded, Bound::Included(j))).next_back();
                    let next = m.range((Bound::Included(j), Bound::Unbounded)).next();
                    if let (Some((_, Wire::Start)), Some((_, Wire::End))) = (prev, next) {
                        mh_distances.push(i.abs() + j.abs());
                    }
                }
                pos = (i2, j);
            }
            'R' => {
                let j1 = pos.1;
                let j2 = pos.1 + distance;
                let i = pos.0;
                for (j, m) in vertical.range((Included(j1), Included(j2))) {
                    let prev = m.range((Bound::Unbounded, Bound::Included(i))).next_back();
                    let next = m.range((Bound::Included(i), Bound::Unbounded)).next();
                    if let (Some((_, Wire::Start)), Some((_, Wire::End))) = (prev, next) {
                        mh_distances.push(i.abs() + j.abs());
                    }
                }
                pos = (i, j2);
            }
            'L' => {
                let j1 = pos.1;
                let j2 = pos.1 - distance;
                let i = pos.0;
                for (j, m) in vertical.range((Included(j2), Included(j1))) {
                    let prev = m.range((Bound::Unbounded, Bound::Included(i))).next_back();
                    let next = m.range((Bound::Included(i), Bound::Unbounded)).next();
                    if let (Some((_, Wire::Start)), Some((_, Wire::End))) = (prev, next) {
                        mh_distances.push(i.abs() + j.abs());
                    }
                }
                pos = (i, j2);
            }
            _ => panic!(),
        }
    }

    mh_distances.sort();
    println!("{:?}", mh_distances[0]);
}
