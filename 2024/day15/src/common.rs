use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    io::BufRead,
};

#[derive(PartialEq, Eq, Clone)]
pub enum Square {
    Empty,
    BoxLeft,
    BoxRight,
    Box,
    Wall,
    Robot,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
            Self::Box => 'O',
            Self::Wall => '#',
            Self::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Offset(i64, i64);

impl From<Move> for Offset {
    fn from(value: Move) -> Self {
        match value {
            Move::Up => Offset(-1, 0),
            Move::Down => Offset(1, 0),
            Move::Right => Offset(0, 1),
            Move::Left => Offset(0, -1),
        }
    }
}

impl TryFrom<char> for Square {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub inner: Vec<Vec<Square>>,
    robot: (usize, usize),
    pub height: usize,
    pub width: usize,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut warehouse: Vec<Vec<Square>> = Vec::new();
        let mut robot = None;
        let mut row = 0;
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 1 {
                break;
            }

            warehouse.push(
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(c, s)| {
                        let s = Square::try_from(s).unwrap();
                        if let Square::Robot = s {
                            robot = Some((row, c));
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
            robot: robot.unwrap(),
        }
    }

    fn process_move(&mut self, move_: Move) {
        let offset = Offset::from(move_);

        let mut objects = Vec::new();
        let mut positions = VecDeque::new();
        positions.push_back(self.robot);
        let mut seen = HashSet::new();

        while let Some(curr) = positions.pop_front() {
            if self.inner[curr.0][curr.1] == Square::Wall {
                return;
            }

            if self.inner[curr.0][curr.1] == Square::BoxLeft {
                let neighbour = self.next_position(curr, &Offset(0, 1));
                if seen.get(&neighbour).is_none() {
                    positions.push_back(neighbour);
                }
            } else if self.inner[curr.0][curr.1] == Square::BoxRight {
                let neighbour = self.next_position(curr, &Offset(0, -1));
                if seen.get(&neighbour).is_none() {
                    positions.push_back(neighbour);
                }
            }

            let next = self.next_position(curr, &offset);
            objects.push((self.inner[curr.0][curr.1].clone(), next));
            objects.push((Square::Empty, curr));
            seen.insert(curr);
            if self.inner[next.0][next.1] != Square::Empty {
                positions.push_back(next);
            }
        }

        while let Some((object, new_position)) = objects.pop() {
            if object == Square::Robot {
                self.robot = new_position;
            }
            self.inner[new_position.0][new_position.1] = object;
        }
    }

    pub fn process_moves(&mut self, moves: Vec<Move>) {
        for move_ in moves {
            self.process_move(move_);
        }
    }

    fn next_position(&self, current: (usize, usize), offset: &Offset) -> (usize, usize) {
        (
            (current.0 as i64 + offset.0) as usize,
            (current.1 as i64 + offset.1) as usize,
        )
    }

    pub fn recreate_map(&mut self) {
        for row_idx in 0..self.inner.len() {
            let mut new_row = Vec::new();
            let mut robot_flag = false;
            for s in &self.inner[row_idx] {
                let squares = match s {
                    Square::Wall => vec![Square::Wall, Square::Wall],
                    Square::Empty => vec![Square::Empty, Square::Empty],
                    Square::Box => vec![Square::BoxLeft, Square::BoxRight],
                    Square::Robot => {
                        robot_flag = true;
                        vec![Square::Robot, Square::Empty]
                    }
                    _ => panic!(),
                };
                new_row.extend(squares);
            }
            if robot_flag {
                new_row.iter().enumerate().for_each(|(c, s)| {
                    if s != &Square::Robot {
                        return;
                    }
                    self.robot = (row_idx, c);
                });
            }
            self.width = new_row.len();
            self.inner[row_idx] = new_row;
        }
    }
}

#[derive(Debug)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

pub fn read_moves<B: BufRead>(mut buf: B) -> Vec<Move> {
    let mut line = String::new();
    let mut moves = Vec::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let row: Vec<Move> = line
            .trim()
            .chars()
            .map(|m| Move::try_from(m).unwrap())
            .collect();
        moves.extend(row);

        line.truncate(0);
    }
    moves
}
