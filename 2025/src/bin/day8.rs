use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{self, Read},
};

type Point = (i64, i64, i64);
type CircuitId = (i64, i64, i64);

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    p1(&input);
    p2(&input)
}

fn p1(input: &str) {
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        let mut coordinates = line.split(",");
        let point = (
            coordinates.next().unwrap().parse().unwrap(),
            coordinates.next().unwrap().parse().unwrap(),
            coordinates.next().unwrap().parse().unwrap(),
        );

        points.push(point);
    }

    let mut diff: BTreeSet<(i64, Point, Point)> = BTreeSet::new();
    for start_idx in 0..points.len() {
        for end_idx in (start_idx + 1)..points.len() {
            if start_idx == end_idx {
                continue;
            }

            let (start, end) = (points[start_idx], points[end_idx]);
            let distance =
                (end.0 - start.0).pow(2) + (end.1 - start.1).pow(2) + (end.2 - start.2).pow(2);
            diff.insert((distance, start, end));
        }
    }

    let mut circuits: HashMap<CircuitId, HashSet<Point>> = HashMap::new();
    let mut point_circuit_map: HashMap<Point, CircuitId> = HashMap::new();

    let mut ops = 0;
    while let Some((_, start, end)) = diff.pop_first() {
        if ops == 1000 {
            break;
        }

        let (map_start, map_end) = (point_circuit_map.get(&start), point_circuit_map.get(&end));
        match (map_start, map_end) {
            (Some(cid), None) => {
                let cid = (*cid).clone();
                point_circuit_map.insert(end, cid);
                circuits.get_mut(&cid).unwrap().insert(end);
            }
            (None, Some(cid)) => {
                let cid = (*cid).clone();
                point_circuit_map.insert(start, cid);
                circuits.get_mut(&cid).unwrap().insert(start);
            }
            (Some(cid_start), Some(cid_end)) if cid_start == cid_end => {}
            (Some(cid_start), Some(cid_end)) => {
                let cid_start = *cid_start;
                let end_circuit = circuits.remove(cid_end).unwrap();
                let start_circuit = circuits.get_mut(&cid_start).unwrap();
                for point in end_circuit {
                    start_circuit.insert(point);
                    point_circuit_map.insert(point, cid_start);
                }
            }
            (None, None) => {
                let circuit = HashSet::from([start, end]);
                let cid = start;
                point_circuit_map.insert(start, cid);
                point_circuit_map.insert(end, cid);
                circuits.insert(cid, circuit);
            }
        }

        ops += 1;
    }

    let mut circuits: Vec<usize> = circuits
        .into_iter()
        .map(|(_, circuit)| circuit.len())
        .collect();
    circuits.sort_by(|a, b| b.cmp(&a));

    let res = circuits[0] * circuits[1] * circuits[2];
    println!("p1: {res:?}");
}

fn p2(input: &str) {
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        let mut coordinates = line.split(",");
        let point = (
            coordinates.next().unwrap().parse().unwrap(),
            coordinates.next().unwrap().parse().unwrap(),
            coordinates.next().unwrap().parse().unwrap(),
        );

        points.push(point);
    }

    let mut diff: BTreeSet<(i64, Point, Point)> = BTreeSet::new();
    for start_idx in 0..points.len() {
        for end_idx in (start_idx + 1)..points.len() {
            if start_idx == end_idx {
                continue;
            }

            let (start, end) = (points[start_idx], points[end_idx]);
            let distance =
                (end.0 - start.0).pow(2) + (end.1 - start.1).pow(2) + (end.2 - start.2).pow(2);
            diff.insert((distance, start, end));
        }
    }

    let mut circuits: HashMap<CircuitId, HashSet<Point>> = HashMap::new();
    let mut point_circuit_map: HashMap<Point, CircuitId> = HashMap::new();

    let mut ops = 0;
    while let Some((_, start, end)) = diff.pop_first() {
        let (map_start, map_end) = (point_circuit_map.get(&start), point_circuit_map.get(&end));
        match (map_start, map_end) {
            (Some(cid), None) => {
                let cid = (*cid).clone();
                point_circuit_map.insert(end, cid);
                circuits.get_mut(&cid).unwrap().insert(end);
            }
            (None, Some(cid)) => {
                let cid = (*cid).clone();
                point_circuit_map.insert(start, cid);
                circuits.get_mut(&cid).unwrap().insert(start);
            }
            (Some(cid_start), Some(cid_end)) if cid_start == cid_end => {}
            (Some(cid_start), Some(cid_end)) => {
                let cid_start = *cid_start;
                let end_circuit = circuits.remove(cid_end).unwrap();
                let start_circuit = circuits.get_mut(&cid_start).unwrap();
                for point in end_circuit {
                    start_circuit.insert(point);
                    point_circuit_map.insert(point, cid_start);
                }
            }
            (None, None) => {
                let circuit = HashSet::from([start, end]);
                let cid = start;
                point_circuit_map.insert(start, cid);
                point_circuit_map.insert(end, cid);
                circuits.insert(cid, circuit);
            }
        }
        if (point_circuit_map.len() == points.len()) && (circuits.len() == 1) {
            println!("p2: {:?}", start.0 * end.0);
            break;
        }
    }
}
