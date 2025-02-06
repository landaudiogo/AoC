use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Square {
    Clear,
    Blocked,
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        if value == '#' {
            Self::Blocked
        } else {
            Self::Clear
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Position {
    coordinates: (usize, usize),
    orientation: (i64, i64),
}

impl TryFrom<((usize, usize), char)> for Position {
    type Error = ();
    fn try_from(value: ((usize, usize), char)) -> Result<Self, Self::Error> {
        let orientation = match value.1 {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => return Err(()),
        };
        Ok(Self {
            coordinates: value.0,
            orientation,
        })
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub inner: Vec<Vec<Square>>,
    pub height: usize,
    pub width: usize,
    pub position: Position,
    pub visited: HashSet<(usize, usize)>,
    pub obstructions: HashSet<(usize, usize)>,
    pub jump_table: HashMap<Position, (usize, usize)>,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut inner: Vec<Vec<Square>> = Vec::new();
        let mut visited = HashSet::new();
        let mut position = None;

        let mut i = 0;
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            inner.push(
                line.trim_end()
                    .chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if let Ok(pos) = Position::try_from(((i, j), c)) {
                            position = Some(pos);
                            visited.insert((i, j));
                        }
                        c.into()
                    })
                    .collect(),
            );
            line.truncate(0);
            i += 1;
        }

        let height = inner.len();
        let width = inner[0].len();
        Matrix {
            inner,
            height,
            width,
            position: position.unwrap(),
            visited,
            obstructions: HashSet::new(),
            jump_table: HashMap::new(),
        }
    }

    pub fn step_try_loop(&mut self) -> Option<bool> {
        let mut is_loop = false;
        if let Ok(next) = self.get_relative(self.position.coordinates, self.position.orientation) {
            match self.inner[next.0][next.1] {
                Square::Clear => {
                    if self.obstructions.get(&next).is_none() {
                        is_loop = self.attempt_loop(next);
                        self.obstructions.insert(next);
                    }
                    self.position.coordinates = next;
                }
                Square::Blocked => {
                    // multiply orientation by 90 degree rotation matrix
                    let o = self.position.orientation;
                    self.position.orientation = (o.0 * 0 + o.1 * 1, o.0 * -1 + o.1 * 0);
                }
            }
        } else {
            return None;
        }

        Some(is_loop)
    }

    pub fn attempt_loop(&mut self, block_position: (usize, usize)) -> bool {
        self.inner[block_position.0][block_position.1] = Square::Blocked;
        let start_position = self.position.clone();
        let mut res = false;
        let mut seen_states = HashSet::new();

        while let Some(_) = self.step() {
            if seen_states.get(&self.position).is_some() {
                res = true;
                break;
            }

            seen_states.insert(self.position.clone());
        }
        self.position = start_position;
        self.inner[block_position.0][block_position.1] = Square::Clear;
        res
    }

    pub fn step(&mut self) -> Option<&Position> {
        let next = self.get_relative(self.position.coordinates, self.position.orientation);
        if let Ok(next) = next {
            match self.inner[next.0][next.1] {
                Square::Clear => {
                    self.visited.insert(next);
                    self.position.coordinates = next;
                }
                Square::Blocked => {
                    let o = self.position.orientation;
                    // multiply orientation by 90 degree rotation matrix
                    self.position.orientation = (o.0 * 0 + o.1 * 1, o.0 * -1 + o.1 * 0);
                }
            }
            return Some(&self.position);
        }
        None
    }

    pub fn get_relative(
        &self,
        current: (usize, usize),
        offset: (i64, i64),
    ) -> Result<(usize, usize), ()> {
        let current = convert_pos_usize_to_i64(current);
        let pos = (current.0 + offset.0, current.1 + offset.1);
        if !Self::valid_position(pos, self.height, self.width) {
            return Err(());
        }

        Ok(convert_pos_i64_to_usize(pos))
    }

    pub fn valid_position(pos: (i64, i64), height: usize, width: usize) -> bool {
        pos.0 >= 0 && pos.0 < height as i64 && pos.1 >= 0 && pos.1 < width as i64
    }
}

fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}
