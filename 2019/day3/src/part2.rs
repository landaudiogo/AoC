use std::{
    collections::BTreeMap,
    io::BufRead,
    ops::Bound::{self, Included},
};

#[derive(Debug)]
enum Wire {
    Start(i64),
    End(i64),
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    buf.read_line(&mut line).unwrap();

    let mut vertical = BTreeMap::new();
    let mut horizontal = BTreeMap::new();
    let mut pos = (0, 0);
    let mut time = 0;

    for action in line.trim().split(",") {
        let mut action = action.chars();
        let direction = action.next().unwrap();
        let distance = action.collect::<String>().parse::<i64>().unwrap();
        match direction {
            'U' => {
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::End(time));
                pos = (pos.0 - distance, pos.1);
                time += distance;
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::Start(time));
            }
            'D' => {
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::Start(time));
                pos = (pos.0 + distance, pos.1);
                time += distance;
                vertical
                    .entry(pos.1)
                    .or_insert(BTreeMap::new())
                    .insert(pos.0, Wire::End(time));
            }
            'R' => {
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::Start(time));
                pos = (pos.0, pos.1 + distance);
                time += distance;
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::End(time));
            }
            'L' => {
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::End(time));
                pos = (pos.0, pos.1 - distance);
                time += distance;
                horizontal
                    .entry(pos.0)
                    .or_insert(BTreeMap::new())
                    .insert(pos.1, Wire::Start(time));
            }
            _ => panic!(),
        }
    }

    line.truncate(0);
    buf.read_line(&mut line).unwrap();
    let mut pos = (0, 0);
    let mut time = 0;
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
                    if let (Some((w1, Wire::Start(t1))), Some((w2, Wire::End(t2)))) = (prev, next) {
                        let (t1, w1) = if t1 < t2 { (t1, w1) } else { (t2, w2) };
                        mh_distances.push(t1 + (j - w1).abs() + time + (i - i1).abs());
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
                    if let (Some((w1, Wire::Start(t1))), Some((w2, Wire::End(t2)))) = (prev, next) {
                        let (t1, w1) = if t1 < t2 { (t1, w1) } else { (t2, w2) };
                        mh_distances.push(t1 + (j - w1).abs() + time + (i - i1).abs());
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
                    if let (Some((w1, Wire::Start(t1))), Some((w2, Wire::End(t2)))) = (prev, next) {
                        let (t1, w1) = if t1 < t2 { (t1, w1) } else { (t2, w2) };
                        mh_distances.push(t1 + (i - w1).abs() + time + (j - j1).abs());
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
                    if let (Some((w1, Wire::Start(t1))), Some((w2, Wire::End(t2)))) = (prev, next) {
                        let (t1, w1) = if t1 < t2 { (t1, w1) } else { (t2, w2) };
                        mh_distances.push(t1 + (i - w1).abs() + time + (j - j1).abs());
                    }
                }
                pos = (i, j2);
            }
            _ => panic!(),
        }
        time += distance;
    }

    mh_distances.sort();
    println!("{:?}", mh_distances[0]);
}
