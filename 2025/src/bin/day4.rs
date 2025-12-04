use anyhow::Result;
use std::{
    collections::{BTreeSet, HashSet},
    io::Stdin,
};

fn main() {
    solve(std::io::stdin());
}

const CARDINALS: [(i64, i64); 8] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // NW
];

type Position = (usize, usize);
type Cardinal = (i64, i64);

enum Cell {
    Roll(u64),
    Empty,
}

struct Grid {
    inner: Vec<Vec<Cell>>,
    removable: BTreeSet<Position>,
}

impl Grid {
    fn new(input: Stdin) -> Self {
        let mut removable = BTreeSet::new();
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.unwrap().trim().chars() {
                let cell = match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll(0),
                    c => panic!("invalid input char {c}"),
                };
                row.push(cell);
            }
            if row.len() == 0 {
                break;
            }
            grid.push(row);
        }

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if let Cell::Empty = grid[r][c] {
                    continue;
                }
                let surrounding_rolls = Self::count_surrounding_rolls(&grid, &(r, c));
                grid[r][c] = Cell::Roll(surrounding_rolls);
                if surrounding_rolls < 4 {
                    removable.insert((r, c));
                }
            }
        }
        Self {
            inner: grid,
            removable,
        }
    }

    fn count_surrounding_rolls(grid: &Vec<Vec<Cell>>, pos: &Position) -> u64 {
        let mut rolls = 0;
        for cardinal in CARDINALS {
            let Some(pos) = Self::move_position(grid, pos, cardinal) else {
                continue;
            };
            if let Cell::Roll(_) = grid[pos.0][pos.1] {
                rolls += 1;
            }
        }
        rolls
    }

    fn move_position(
        grid: &Vec<Vec<Cell>>,
        pos: &Position,
        cardinal: Cardinal,
    ) -> Option<Position> {
        let res = (pos.0 as i64 + cardinal.0, pos.1 as i64 + cardinal.1);
        let (nrows, ncols) = (grid.len() as i64, grid.len() as i64);
        if res.0 < 0 || res.0 >= nrows {
            None
        } else if res.1 < 0 || res.1 >= ncols {
            None
        } else {
            Some((res.0 as usize, res.1 as usize))
        }
    }

    fn remove_all(&mut self) -> u64 {
        let mut removed = 0;
        while let Some(pos) = self.removable.pop_first() {
            removed += 1;
            self.inner[pos.0][pos.1] = Cell::Empty;
            for cardinal in CARDINALS {
                let Some(neigh) = Self::move_position(&self.inner, &pos, cardinal) else {
                    continue;
                };

                if let Cell::Roll(cnt) = &mut self.inner[neigh.0][neigh.1] {
                    // println!(
                    //     "({}, {}) || ({}, {}) -> {cnt}",
                    //     pos.0, pos.1, neigh.0, neigh.1
                    // );
                    *cnt -= 1;
                    if *cnt < 4 {
                        self.removable.insert(neigh);
                    }
                }
            }
        }
        removed
    }
}

fn solve(input: Stdin) -> Result<()> {
    let mut total = 0;
    let mut grid = Grid::new(input);
    println!("p1: {}", grid.removable.len());
    println!("p2: {}", grid.remove_all());
    Ok(())
}
