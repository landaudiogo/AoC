use std::io;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Cell {
    Beam,
    Splitter,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '^' => Cell::Splitter,
            'S' => Cell::Beam,
            _ => panic!("invalid character {c}"),
        }
    }
}

fn main() {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
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
            let mut cell = &mut grid[r][c];
            if cell == &Cell::Splitter && above == Cell::Beam {
                grid[r][c + 1] = Cell::Beam;
                grid[r][c - 1] = Cell::Beam;
                splits += 1;
            } else if cell == &Cell::Empty && above == Cell::Beam {
                *cell = Cell::Beam;
            }
        }
    }
    println!("{splits}");
}
