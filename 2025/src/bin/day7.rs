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
    p2(&input);
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

fn p2(input: &str) {
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
    println!("p2: {timelines}");
}
