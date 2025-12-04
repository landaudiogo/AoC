use anyhow::Result;
use std::io::Stdin;

fn main() {
    p1(std::io::stdin());
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
    Roll,
    Empty,
}

struct Grid {
    inner: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(input: Stdin) -> Self {
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.unwrap().trim().chars() {
                let cell = match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    c => panic!("invalid input char {c}"),
                };
                row.push(cell);
            }
            if row.len() == 0 {
                break;
            }
            grid.push(row);
        }
        Self { inner: grid }
    }

    fn count_surrounding_rolls(&self, pos: &Position) -> u64 {
        let mut rolls = 0;
        for cardinal in CARDINALS {
            let Some(pos) = self.move_position(pos, cardinal) else {
                continue;
            };
            if let Cell::Roll = self.inner[pos.0][pos.1] {
                rolls += 1;
            }
        }
        rolls
    }

    fn move_position(&self, pos: &Position, cardinal: Cardinal) -> Option<Position> {
        let res = (pos.0 as i64 + cardinal.0, pos.1 as i64 + cardinal.1);
        let (nrows, ncols) = (self.inner.len() as i64, self.inner[0].len() as i64);
        if res.0 < 0 || res.0 >= nrows {
            None
        } else if res.1 < 0 || res.1 >= ncols {
            None
        } else {
            Some((res.0 as usize, res.1 as usize))
        }
    }
}

fn p1(input: Stdin) -> Result<()> {
    let mut total = 0;
    let grid = Grid::new(input);
    for r in 0..grid.inner.len() {
        for c in 0..grid.inner[0].len() {
            if let Cell::Empty = grid.inner[r][c] {
                continue;
            }
            total += if grid.count_surrounding_rolls(&(r, c)) < 4 {
                1
            } else {
                0
            };
        }
    }
    println!("p1: {total}");
    Ok(())
}
