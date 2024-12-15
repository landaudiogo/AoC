use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Square {
    Empty,
    Box,
    Wall,
    Robot,
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
struct Matrix {
    inner: Vec<Vec<Square>>,
    height: usize,
    width: usize,
    robot: (usize, usize),
}

impl Matrix {
    fn new<B: BufRead>(mut buf: B) -> Self {
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
        let mut curr = self.robot;
        let mut next = self.next_position(self.robot, &offset);
        let mut objects = VecDeque::new();
        objects.push_back((Square::Empty, curr));
        while (self.inner[curr.0][curr.1] != Square::Empty)
            && (self.inner[curr.0][curr.1] != Square::Wall)
        {
            objects.push_back((self.inner[curr.0][curr.1].clone(), next));
            curr = next;
            next = self.next_position(next, &offset);
        }

        if self.inner[curr.0][curr.1] == Square::Empty {
            while let Some((object, new_position)) = objects.pop_back() {
                if object == Square::Robot {
                    self.robot = new_position;
                }
                self.inner[new_position.0][new_position.1] = object;
            }
        }
    }

    fn process_moves(&mut self, moves: Vec<Move>) {
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
}

#[derive(Debug)]
enum Move {
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

fn read_moves<B: BufRead>(mut buf: B) -> Vec<Move> {
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

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut matrix = Matrix::new(&mut buf);
    let moves = read_moves(&mut buf);
    matrix.process_moves(moves);

    let mut total = 0;
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix.inner[i][j] == Square::Box {
                total += 100 * i + j;
            }
        }
    }
    total as u64
}
