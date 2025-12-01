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
    let mut visited = HashSet::new();
    orbits.get("YOU").map(|orbits| {
        orbits.iter().for_each(|orbit| {
            visit.push_back((0, orbit.clone()));
        })
    });
    object_orbit
        .get("YOU")
        .map(|center| visit.push_back((0, center.clone())));
    visited.insert(String::from("YOU"));

    while let Some((depth, obj)) = visit.pop_front() {
        if obj == "SAN" {
            println!("{} {}", obj, depth - 1);
            break;
        }

        if visited.get(&obj).is_some() {
            continue;
        } else {
            visited.insert(String::from(&obj));
        }

        orbits.get(&obj).map(|orbits| {
            orbits.iter().for_each(|orbit| {
                if visited.get(orbit).is_none() {
                    visit.push_back((depth + 1, orbit.clone()));
                }
            })
        });
        object_orbit.get(&obj).map(|center| {
            if visited.get(center).is_none() {
                visit.push_back((depth + 1, center.clone()));
            }
        });
    }
}
