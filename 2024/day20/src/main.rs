use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Cheat {
    Started(u64),
    Depleted,
}

enum Reach {
    Reachable(u64),
    Unreachable,
}

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

    fn get_cheats(
        &self,
        curr: (usize, usize),
        cheat: u64,
        cache: &mut HashMap<((usize, usize), u64), HashSet<(usize, usize)>>,
    ) -> HashSet<(usize, usize)> {
        if let Some(set) = cache.get(&(curr, cheat)) {
            return set.clone();
        }

        if cheat == 0 && self.inner[curr.0][curr.1] != Square::Wall {
            return HashSet::from_iter([curr]);
        }

        let mut exits = HashSet::new();
        for offset in [Offset(0, 1), Offset(0, -1), Offset(1, 0), Offset(-1, 0)] {
            if let Some(next) = self.next_position(curr, &offset) {
                if cheat >= 1 {
                    exits.extend(self.get_cheats(next, cheat - 1, cache));
                    if self.inner[next.0][next.1] != Square::Wall {
                        exits.insert(next);
                    }
                }
            }
        }
        cache.insert((curr, cheat), exits.clone());
        exits
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
    println!("{:?}", matrix);
    let start_dist = matrix.get_start_dist();
    let end_dist = matrix.get_end_dist();
    let shortest_time = *start_dist.get(&matrix.end).unwrap();

    let mut improvements = HashMap::new();
    let mut cache = HashMap::new();
    let cheat_time = 20;
    let minimum_improvement = 100;
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix.inner[i][j] != Square::Wall {
                let entry = (i, j);
                let exits = matrix.get_cheats((i, j), cheat_time, &mut cache);

                for exit in exits {
                    let entry_cost = start_dist.get(&(i, j)).unwrap();
                    let exit_cost = end_dist.get(&exit).unwrap();
                    let total_cost = entry_cost
                        + exit_cost
                        + (entry.0 as i64 - exit.0 as i64).abs() as u64
                        + (entry.1 as i64 - exit.1 as i64).abs() as u64;
                    let val = improvements.entry(total_cost).or_insert(0);
                    *val += 1;
                }
            }
        }
    }

    let mut improvements = improvements
        .into_iter()
        .filter(|(k, _)| *k <= shortest_time)
        .map(|(k, v)| (shortest_time - k, v))
        .filter(|(k, _)| *k >= minimum_improvement)
        .collect::<Vec<(u64, u64)>>();
    improvements.sort();
    println!("{:?}", improvements);
    println!(
        "{:?}",
        improvements
            .iter()
            .map(|(_, v)| v)
            .fold(0, |acc, v| acc + v)
    );
}
