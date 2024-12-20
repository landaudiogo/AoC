use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Clone)]
pub enum Square {
    Empty,
    Wall,
    Start,
    End,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Start => 'S',
            Self::End => 'E',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Offset(i64, i64);

impl TryFrom<char> for Square {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            _ => Err(()),
        }
    }
}

pub struct Matrix {
    pub inner: Vec<Vec<Square>>,
    start: (usize, usize),
    end: (usize, usize),
    pub height: usize,
    pub width: usize,
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "height: {:?}, width: {:?}", self.height, self.width)?;
        for row in self.inner.iter() {
            for c in row {
                write!(f, "{:?}", c)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)?;
        Ok(())
    }
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut warehouse: Vec<Vec<Square>> = Vec::new();
        let mut start = None;
        let mut end = None;
        let mut row = 0;
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            warehouse.push(
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(c, s)| {
                        let s = Square::try_from(s).unwrap();
                        if let Square::Start = s {
                            start = Some((row, c));
                        } else if let Square::End = s {
                            end = Some((row, c));
                        }
                        s
                    })
                    .collect(),
            );

            row += 1;
            line.truncate(0);
        }

        Self {
            height: warehouse.len(),
            width: warehouse[0].len(),
            inner: warehouse,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn next_position(&self, current: (usize, usize), offset: &Offset) -> Option<(usize, usize)> {
        if (current.0 as i64 + offset.0) > 0
            && (current.1 as i64 + offset.1) > 0
            && (current.0 as i64 + offset.0) < self.height as i64
            && (current.1 as i64 + offset.1) < self.width as i64
        {
            Some((
                (current.0 as i64 + offset.0) as usize,
                (current.1 as i64 + offset.1) as usize,
            ))
        } else {
            None
        }
    }

    fn get_start_dist(&self) -> HashMap<(usize, usize), u64> {
        let mut visit = Vec::new();
        visit.push(self.start);
        let mut cost = HashMap::new();
        cost.insert(self.start, 0);
        while let Some(curr) = visit.pop() {
            for offset in [Offset(0, 1), Offset(0, -1), Offset(1, 0), Offset(-1, 0)] {
                if let Some(next) = self.next_position(curr, &offset) {
                    if self.inner[next.0][next.1] == Square::Wall {
                        continue;
                    }

                    let curr_cost = *cost.get(&curr).unwrap();
                    if let Some(next_cost) = cost.get_mut(&next) {
                        if *next_cost < curr_cost + 1 {
                            continue;
                        }
                    }
                    visit.push(next);
                    cost.insert(next, curr_cost + 1);
                }
            }
        }
        cost
    }

    fn get_end_dist(&self) -> HashMap<(usize, usize), u64> {
        let mut visit = Vec::new();
        visit.push(self.end);
        let mut cost = HashMap::new();
        cost.insert(self.end, 0);
        while let Some(curr) = visit.pop() {
            for offset in [Offset(0, 1), Offset(0, -1), Offset(1, 0), Offset(-1, 0)] {
                if let Some(next) = self.next_position(curr, &offset) {
                    if self.inner[next.0][next.1] == Square::Wall {
                        continue;
                    }

                    let curr_cost = *cost.get(&curr).unwrap();
                    if let Some(next_cost) = cost.get_mut(&next) {
                        if *next_cost < curr_cost + 1 {
                            continue;
                        }
                    }
                    visit.push(next);
                    cost.insert(next, curr_cost + 1);
                }
            }
        }
        cost
    }
}

fn main() {
    let matrix = Matrix::new(BufReader::new(File::open("input").unwrap()));
    let start_dist = matrix.get_start_dist();
    let end_dist = matrix.get_end_dist();
    let shortest_time = *start_dist.get(&matrix.end).unwrap();
    let mut time_saving = HashMap::new();
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix.inner[i][j] == Square::Wall {
                let curr = (i, j);
                let offsets = [
                    (Offset(0, 1), Offset(0, -1)),
                    (Offset(0, -1), Offset(0, 1)),
                    (Offset(1, 0), Offset(-1, 0)),
                    (Offset(-1, 0), Offset(1, 0)),
                ];
                for off in offsets {
                    let prev = matrix.next_position(curr, &off.0);
                    let next = matrix.next_position(curr, &off.1);
                    if let (Some(prev), Some(next)) = (prev, next) {
                        let prev_cost = start_dist.get(&prev);
                        let next_cost = end_dist.get(&next);
                        if let (Some(prev), Some(next)) = (prev_cost, next_cost) {
                            if prev + next + 2 < shortest_time {
                                let mut save = time_saving
                                    .entry(shortest_time - (prev + next + 2))
                                    .or_insert(0);
                                *save += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut total = 0;
    for (k, v) in time_saving {
        if k >= 100 {
            total += v;
        }
    }
    dbg!(total);
}
