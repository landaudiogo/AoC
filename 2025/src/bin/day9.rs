use std::{
    collections::BTreeSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    p1(&input);
}

type Point = (i64, i64);

fn p1(input: &str) {
    let mut points = Vec::new();
    for line in input.lines() {
        let mut coordinates = line.split(",");
        let (x, y): (i64, i64) = (
            coordinates.next().unwrap().parse().unwrap(),
            coordinates.next().unwrap().parse().unwrap(),
        );
        points.push((x, y));
    }

    let mut areas = BTreeSet::new();
    for i in 0..points.len() {
        for j in i..points.len() {
            if i == j {
                continue;
            }
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            // println!(
            //     "{:?} {:?} => {:?}*{:?} => {:?}",
            //     points[i],
            //     points[j],
            //     ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
            // );
            areas.insert(((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1));
        }
    }

    println!("{:?}", areas.pop_last());
}
