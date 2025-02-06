use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::BufRead,
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Reindeer {
    pos: (usize, usize),
    dir: (i64, i64),
}

#[derive(PartialEq, Eq)]
enum Square {
    Wall,
    Empty,
    Start,
    End,
    Seat,
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => {
                dbg!(value);
                panic!();
            }
        }
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Wall => '#',
            Self::Empty => '.',
            Self::Start => 'S',
            Self::End => 'E',
            Self::Seat => 'O',
        };
        write!(f, "{}", value)
    }
}

struct Matrix {
    inner: Vec<Vec<Square>>,
    start: Reindeer,
    end: (usize, usize),
}

impl Matrix {
    fn new<B: BufRead>(mut buf: B) -> Self {
        let mut inner = Vec::new();
        let mut line = String::new();
        let mut start = None;
        let mut end = None;
        let mut i = 0;
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            inner.push(
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let square = Square::from(c);
                        if square == Square::Start {
                            start = Some(Reindeer {
                                pos: (i, j),
                                dir: (0, 1),
                            });
                        } else if square == Square::End {
                            end = Some((i, j));
                        }
                        square
                    })
                    .collect(),
            );

            line.truncate(0);
            i += 1;
        }

        Self {
            inner,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn find_min_score(&mut self) -> (u64, u64) {
        let mut min_score = None;
        let mut seen = HashMap::new();
        seen.insert(self.start.clone(), 0);
        let mut visit = Vec::new();
        visit.push((self.start.clone(), 0));
        while let Some((reindeer, cost)) = visit.pop() {
            if reindeer.pos == self.end {
                if let Some(min) = min_score {
                    if cost < min {
                        min_score = Some(cost);
                        seen.insert(reindeer, cost);
                    }
                } else {
                    min_score = Some(cost);
                    seen.insert(reindeer, cost);
                }
                continue;
            }
            let moves = self.find_valid_next_states(&reindeer, cost);
            moves.into_iter().for_each(|(nstate, ncost)| {
                if seen.get(&nstate).is_none() {
                    visit.push((nstate, ncost));
                } else if let Some(pos_cost) = seen.get_mut(&nstate) {
                    if ncost < *pos_cost {
                        visit.push((nstate, ncost))
                    }
                }
            });
            seen.insert(reindeer, cost);
        }

        let mut seats = HashSet::new();
        let mut visited = HashSet::new();
        let mut visit = Vec::new();
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            visit.push((Reindeer { pos: self.end, dir }, min_score.unwrap() as i64));
        }
        while let Some((reindeer, cost)) = visit.pop() {
            let moves = self.backtrack_states(&reindeer, cost);
            moves.into_iter().for_each(|(nstate, ncost)| {
                if seen.get(&nstate).map(|n| *n as i64) == Some(ncost) && ncost >= 0 {
                    if let None = visited.get(&(nstate.clone(), ncost)) {
                        visit.push((nstate, ncost));
                    }
                }
            });
            if seen.get(&reindeer).map(|n| *n as i64) == Some(cost) {
                seats.insert(reindeer.pos);
            }
            visited.insert((reindeer, cost));
        }

        (min_score.unwrap(), seats.len() as u64)
    }

    fn backtrack_states(&self, reindeer: &Reindeer, cost: i64) -> Vec<(Reindeer, i64)> {
        let mut states = Vec::new();
        let dir = self.rotate90(reindeer.dir, true);
        let next = self.get_relative_pos(reindeer.pos, dir);
        if self.inner[next.0][next.1] != Square::Wall {
            states.push((
                Reindeer {
                    pos: reindeer.pos,
                    dir: self.rotate180(dir),
                },
                cost as i64 - 1000,
            ));
        }

        let dir = self.rotate90(reindeer.dir, false);
        let next = self.get_relative_pos(reindeer.pos, dir);
        if self.inner[next.0][next.1] != Square::Wall {
            states.push((
                Reindeer {
                    dir: self.rotate180(dir),
                    pos: reindeer.pos,
                },
                cost as i64 - 1000,
            ));
        }

        let next = self.get_relative_pos(reindeer.pos, self.rotate180(reindeer.dir));
        if self.inner[next.0][next.1] != Square::Wall {
            states.push((
                Reindeer {
                    pos: next,
                    dir: reindeer.dir,
                },
                cost as i64 - 1,
            ));
        }
        states
    }

    fn find_valid_next_states(&self, reindeer: &Reindeer, cost: u64) -> Vec<(Reindeer, u64)> {
        let mut valid = Vec::new();
        let dir = self.rotate90(reindeer.dir, true);
        let next = self.get_relative_pos(reindeer.pos, dir);
        if self.inner[next.0][next.1] != Square::Wall {
            valid.push((
                Reindeer {
                    pos: reindeer.pos,
                    dir,
                },
                cost + 1000,
            ));
        }

        let dir = self.rotate90(reindeer.dir, false);
        let next = self.get_relative_pos(reindeer.pos, dir);
        if self.inner[next.0][next.1] != Square::Wall {
            valid.push((
                Reindeer {
                    dir,
                    pos: reindeer.pos,
                },
                cost + 1000,
            ));
        }

        let next = self.get_relative_pos(reindeer.pos, reindeer.dir);
        if self.inner[next.0][next.1] != Square::Wall {
            valid.push((
                Reindeer {
                    pos: next,
                    dir: reindeer.dir,
                },
                cost + 1,
            ));
        }
        valid
    }

    fn rotate180(&self, offset: (i64, i64)) -> (i64, i64) {
        (-offset.0, -offset.1)
    }

    fn rotate90(&self, offset: (i64, i64), clockwise: bool) -> (i64, i64) {
        if clockwise {
            (offset.1 * 1, offset.0 * -1)
        } else {
            (offset.1 * -1, offset.0 * 1)
        }
    }

    fn get_relative_pos(&self, start: (usize, usize), offset: (i64, i64)) -> (usize, usize) {
        (
            (start.0 as i64 + offset.0) as usize,
            (start.1 as i64 + offset.1) as usize,
        )
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner.iter() {
            for square in row {
                write!(f, "{:?}", square)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "Start - {:?}, End - {:?}", self.start, self.end)
    }
}

pub fn run<B: BufRead>(buf: B) -> (u64, u64) {
    let mut matrix = Matrix::new(buf);
    println!("{:?}", matrix);
    matrix.find_min_score()
}
