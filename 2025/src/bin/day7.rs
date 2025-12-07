use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Cell {
    Beam(u64),
    Splitter,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '^' => Cell::Splitter,
            'S' => Cell::Beam(1),
            _ => panic!("invalid character {c}"),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    p1(&input);
    p2_1(&input);
    p2_2(&input);
}

fn p1(input: &str) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Cell::from(c));
        }
        grid.push(row);
    }

    let mut splits = 0;
    for r in 1..grid.len() {
        for c in 0..grid[0].len() {
            let above = grid[r - 1][c];
            let cell = grid[r][c];
            match (cell, above) {
                (Cell::Splitter, Cell::Beam(_)) => {
                    grid[r][c + 1] = Cell::Beam(1);
                    grid[r][c - 1] = Cell::Beam(1);
                    splits += 1;
                }
                (Cell::Empty, Cell::Beam(_)) => {
                    grid[r][c] = Cell::Beam(1);
                }
                _ => {}
            }
        }
    }
    println!("p1: {splits}");
}

fn p2_1(input: &str) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Cell::from(c));
        }
        grid.push(row);
    }

    let mut splits = 0;
    for r in 1..grid.len() {
        for c in 0..grid[0].len() {
            let above = grid[r - 1][c];
            let cell = grid[r][c];
            match (cell, above) {
                (Cell::Splitter, Cell::Beam(timelines)) => {
                    let (left, right) = (grid[r][c - 1], grid[r][c + 1]);
                    if let Cell::Beam(cnt) = left {
                        grid[r][c - 1] = Cell::Beam(cnt + timelines);
                    } else {
                        grid[r][c - 1] = Cell::Beam(timelines)
                    }

                    if let Cell::Beam(cnt) = right {
                        grid[r][c + 1] = Cell::Beam(cnt + timelines);
                    } else {
                        grid[r][c + 1] = Cell::Beam(timelines)
                    }
                }
                (Cell::Empty, Cell::Beam(timelines)) => {
                    grid[r][c] = Cell::Beam(timelines);
                }
                (Cell::Beam(cnt), Cell::Beam(timelines)) => {
                    grid[r][c] = Cell::Beam(cnt + timelines);
                }
                c => {
                    // println!("{:?}", c);
                }
            }
        }
    }

    let timelines = grid[grid.len() - 1].iter().fold(0, |acc, entry| {
        if let Cell::Beam(cnt) = entry {
            acc + cnt
        } else {
            acc
        }
    });
    println!("p2.1: {timelines}");
}

fn p2_2(input: &str) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Cell::from(c));
        }
        grid.push(row);
    }

    let mut total_timelines = 0;
    let mut timelines: HashMap<(usize, usize), u64> = HashMap::new();
    let mut visit = VecDeque::new();
    let (start, _) = grid[0]
        .iter()
        .enumerate()
        .find(|(i, c)| if let Cell::Beam(_) = c { true } else { false })
        .unwrap();
    visit.push_back((1, start));

    while let Some((r, c)) = visit.pop_back() {
        if r + 1 == grid.len() {
            timelines.insert((r, c), 1);
            continue;
        }

        if timelines.get(&(r, c)).is_some() {
            continue;
        }

        let next = grid[r + 1][c];

        if let Cell::Empty = next {
            let down = timelines.get(&(r + 1, c));
            if let Some(down) = down {
                timelines.insert((r, c), *down);
            } else {
                visit.push_back((r, c));
                visit.push_back((r + 1, c));
            }
            visit.push_back((r + 1, c));
        } else if let Cell::Splitter = next {
            let (left, right) = (
                timelines.get(&(r + 1, c - 1)),
                timelines.get(&(r + 1, c + 1)),
            );

            if let (Some(left), Some(right)) = (left, right) {
                timelines.insert((r, c), left + right);
            } else {
                visit.push_back((r, c));
                visit.push_back((r + 1, c + 1));
                visit.push_back((r + 1, c - 1));
            }
        }
    }

    println!("p2.2: {:?}", timelines.get(&(1, start)).unwrap());
}
