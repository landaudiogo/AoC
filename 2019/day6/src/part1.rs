use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    let mut orbits = HashMap::new();
    let mut object_orbit = HashMap::new();

    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut objects = line.trim().split(")");
        let center = objects.next().unwrap();
        let orbit = objects.next().unwrap();

        orbits
            .entry(String::from(center))
            .or_insert_with(|| HashSet::new())
            .insert(String::from(orbit));
        object_orbit.insert(String::from(orbit), String::from(center));

        line.truncate(0);
    }

    let mut visit = VecDeque::new();
    let mut total = 0;
    orbits.get("COM").map(|orbits| {
        orbits.iter().for_each(|orbit| {
            visit.push_back((1, orbit.clone()));
        })
    });

    while let Some((depth, center)) = visit.pop_front() {
        total += depth;
        orbits.get(&center).map(|orbits| {
            orbits
                .iter()
                .for_each(|orbit| visit.push_back((depth + 1, orbit.clone())))
        });
    }

    println!("{}", total);
}
