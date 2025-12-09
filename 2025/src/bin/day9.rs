use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    p1(&input);
    p2(&input);
}

type Point = (i64, i64);
type Area = (i64, Point, Point);

#[derive(Debug)]
enum NormalRotation {
    Right,
    Left,
}

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
            areas.insert(((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1));
        }
    }

    println!("p1: {}", areas.pop_last().unwrap());
}

fn p2(input: &str) {
    let mut points = Vec::new();
    for line in input.lines() {
        let mut coordinates = line.split(",");
        let (x, y): (i64, i64) = (
            coordinates.next().unwrap().parse().unwrap(),
            coordinates.next().unwrap().parse().unwrap(),
        );
        points.push((x, y));
    }
    points.push(points[0].clone());

    let mut vertical: BTreeSet<(i64, (i64, i64))> = BTreeSet::new();
    let mut horizontal: BTreeSet<(i64, (i64, i64))> = BTreeSet::new();

    for side in points.windows(2) {
        match (side[0], side[1]) {
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                vertical.insert((x1, (y1, y2)));
            }
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                horizontal.insert((y1, (x1, x2)));
            }
            _ => panic!("shouldn't be possible"),
        }
    }
    points.pop();

    let (_, (x1, x2)) = horizontal.last().unwrap();
    let rotation = if x2 > x1 {
        NormalRotation::Right
    } else {
        NormalRotation::Left
    };

    let mut areas: BTreeSet<Area> = BTreeSet::new();
    for i in 0..points.len() {
        for j in i..points.len() {
            if i == j {
                continue;
            }
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let area_ = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            let area = (area_, (x1, y1), (x2, y2));
            if let Some(area) = area_valid(area, &horizontal, &vertical, &rotation) {
                areas.insert(area);
            }
        }
    }

    println!("p2: {}", areas.pop_last().unwrap().0);
}

fn get_normal(vec: (i64, i64), rotation: &NormalRotation) -> (i64, i64) {
    let mag = (vec.0 + vec.1).abs();
    let vec = (vec.0 / mag, vec.1 / mag);
    match rotation {
        NormalRotation::Left => (vec.1, -vec.0),
        NormalRotation::Right => (-vec.1, vec.0),
    }
}

fn area_valid(
    area: Area,
    horizontal: &BTreeSet<(i64, (i64, i64))>,
    vertical: &BTreeSet<(i64, (i64, i64))>,
    rotation: &NormalRotation,
) -> Option<Area> {
    let (area_, (x1, y1), (x2, y2)) = area;
    let (x1, x2) = if x2 > x1 { (x1, x2) } else { (x2, x1) };
    let (y1, y2) = if y2 > y1 { (y1, y2) } else { (y2, y1) };

    let rect_vertical: BTreeMap<i64, (i64, i64)> = BTreeMap::from([(x1, (y1, y2)), (x2, (y2, y1))]);
    let rect_horizontal: BTreeMap<i64, (i64, i64)> =
        BTreeMap::from([(y2, (x1, x2)), (y1, (x2, x1))]);

    let range_start = (x1, (0, 0));
    let range_end = (x2 + 1, (0, 0));
    let contained_verticals = vertical
        .range(range_start..range_end)
        .filter(|(_, (s1, s2))| {
            let (s1, s2) = if s2 > s1 { (s1, s2) } else { (s2, s1) };
            ((*s2 - y1) > 0) && ((y2 - *s1) > 0)
        });
    for (v, (start, end)) in contained_verticals {
        if let Some(&(rstart, rend)) = rect_vertical.get(v) {
            let snorm = get_normal((end - start, 0), rotation);
            let rnorm = get_normal((rend - rstart, 0), &NormalRotation::Right);
            if snorm != rnorm {
                return None;
            }
        } else {
            return None;
        }
    }

    let range_start = (y1, (0, 0));
    let range_end = (y2 + 1, (0, 0));
    let contained_horizontals = horizontal
        .range(range_start..range_end)
        .filter(|(_, (s1, s2))| {
            let (s1, s2) = if s2 > s1 { (s1, s2) } else { (s2, s1) };
            ((*s2 - x1) > 0) && ((x2 - *s1) > 0)
        });
    for (h, (start, end)) in contained_horizontals {
        if let Some(&(rstart, rend)) = rect_horizontal.get(h) {
            let snorm = get_normal((0, end - start), rotation);
            let rnorm = get_normal((0, rend - rstart), &NormalRotation::Right);
            if snorm != rnorm {
                return None;
            }
        } else {
            return None;
        }
    }

    return Some(area);
}
