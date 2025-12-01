use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
    sync::mpsc::{self, Receiver},
    thread,
};

use regex::Regex;

use crate::intcode::Program;

#[derive(Eq, PartialEq)]
enum Square {
    Scaffold,
    Empty,
}

impl From<Square> for char {
    fn from(value: Square) -> Self {
        match value {
            Square::Scaffold => '#',
            Square::Empty => '.',
        }
    }
}

impl From<&Square> for char {
    fn from(value: &Square) -> Self {
        match value {
            Square::Scaffold => '#',
            Square::Empty => '.',
        }
    }
}

impl TryFrom<char> for Square {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Scaffold),
            '^' => Ok(Self::Scaffold),
            '.' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum RobotMotion {
    Rotate90,
    Rotate240,
    Move,
}

impl From<RobotMotion> for &'static str {
    fn from(value: RobotMotion) -> Self {
        match value {
            RobotMotion::Rotate90 => "R1",
            RobotMotion::Rotate240 => "L1",
            RobotMotion::Move => "1",
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Robot {
    pos: (i64, i64),
    dir: (i64, i64),
}

impl Robot {
    fn next(&self, mv: RobotMotion) -> Robot {
        match mv {
            RobotMotion::Move => Robot {
                pos: (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1),
                dir: self.dir,
            },
            RobotMotion::Rotate90 => {
                let new_dir = (self.dir.1, -self.dir.0);
                Robot {
                    pos: (self.pos.0 + new_dir.0, self.pos.1 + new_dir.1),
                    dir: new_dir,
                }
            }
            RobotMotion::Rotate240 => {
                let new_dir = (-self.dir.1, self.dir.0);
                Robot {
                    pos: (self.pos.0 + new_dir.0, self.pos.1 + new_dir.1),
                    dir: new_dir,
                }
            }
        }
    }
}

impl From<&Robot> for char {
    fn from(value: &Robot) -> Self {
        match value.dir {
            (1, 0) => 'v',
            (-1, 0) => '^',
            (0, 1) => '>',
            (0, -1) => '<',
            _ => panic!(),
        }
    }
}

struct Matrix {
    grid: Vec<Vec<Square>>,
    grid_height: i64,
    grid_width: i64,
    robot_start: Robot,
    total_scaffolds: i64,
}

impl Matrix {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let mut total_scaffolds = 0;
        let mut robot = None;
        let grid: Vec<Vec<Square>> = grid
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(j, cell)| {
                        if cell == '^' {
                            robot = Some(Robot {
                                pos: (i as i64, j as i64),
                                dir: (-1, 0),
                            });
                        }
                        let square = Square::try_from(cell).unwrap();
                        if let Square::Scaffold = square {
                            total_scaffolds += 1;
                        }
                        square
                    })
                    .collect()
            })
            .collect();
        Self {
            grid_height: grid.len() as i64,
            grid_width: grid.last().unwrap().len() as i64,
            grid,
            robot_start: robot.unwrap(),
            total_scaffolds,
        }
    }

    fn from_channel(rx: &Receiver<i64>) -> Self {
        let mut map = Vec::new();
        map.push(Vec::new());
        while let Ok(v) = rx.recv() {
            let row = map.last_mut().unwrap();
            match v {
                35 => {
                    row.push('#');
                }
                46 => {
                    row.push('.');
                }
                10 => {
                    if row.len() == 0 {
                        map.pop();
                        break;
                    } else {
                        map.push(Vec::new());
                    }
                }
                v => {
                    row.push(unsafe { char::from_u32_unchecked(v as u32) });
                }
            }
        }
        Self::new(map)
    }

    fn display_grid(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if self.robot_start.pos == (i as i64, j as i64) {
                    print!("{}", char::from(&self.robot_start));
                } else {
                    print!("{}", char::from(cell));
                }
            }
            println!();
        }
    }

    fn on_scaffold(&self, robot: &Robot) -> bool {
        if !(robot.pos.0 >= 0
            && robot.pos.0 < self.grid_height
            && robot.pos.1 >= 0
            && robot.pos.1 < self.grid_width)
        {
            return false;
        }

        if self.grid[robot.pos.0 as usize][robot.pos.1 as usize] == Square::Empty {
            return false;
        }

        true
    }

    fn find_path(&self) -> Option<String> {
        let mut visit = VecDeque::new();
        visit.push_back((
            String::new(),
            self.robot_start.clone(),
            HashSet::from([self.robot_start.pos]),
            HashSet::new(),
        ));
        while let Some((seq, robot, visited_scaffolds, visited)) = visit.pop_back() {
            if visited_scaffolds.len() == self.total_scaffolds as usize {
                let path = shorten_path(&seq);
                if let Some(stream) = routines(path.clone(), path, 0, Vec::new()) {
                    return Some(stream);
                }
                continue;
            }

            for mv in [
                RobotMotion::Rotate240,
                RobotMotion::Rotate90,
                RobotMotion::Move,
            ] {
                let next = robot.next(mv);

                if self.on_scaffold(&next) {
                    if visited.get(&(next.pos, robot.pos)).is_none()
                        && visited.get(&(robot.pos, next.pos)).is_none()
                    {
                        let mut visited_scaffolds = visited_scaffolds.clone();
                        visited_scaffolds.insert(next.pos);

                        let mut visited = visited.clone();
                        visited.insert((robot.pos, next.pos));

                        visit.push_back((
                            seq.clone() + mv.into(),
                            next,
                            visited_scaffolds,
                            visited,
                        ));
                    }
                }
            }
        }

        None
    }
}

pub fn run<B: BufRead>(mut buf: B) -> Result<(), Box<dyn std::error::Error>> {
    let mut program = String::new();
    buf.read_to_string(&mut program)?;
    let (mut otx, orx) = mpsc::channel();
    let (itx, mut irx) = mpsc::channel();
    thread::spawn(move || {
        let mut program: Vec<i64> = program
            .trim()
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect();
        program[0] = 2;
        let mut program = Program::new(program.into_iter(), &mut irx, &mut otx);
        program.execute();
    });

    let matrix = Matrix::from_channel(&orx);
    let stream = matrix.find_path().unwrap();

    let stream = stream.clone() + "\nn\n";
    for c in stream.chars() {
        itx.send(c as u8 as i64)?;
    }

    while let Ok(v) = orx.recv() {
        if let Some(c) = char::from_u32(v as u32) {
            if c.is_ascii() {
                print!("{c}")
            } else {
                println!("p2: {}", v);
            }
        } else {
            println!("p2: {}", v);
        }
    }

    Ok(())
}

fn routines(orig: String, s: String, depth: usize, current: Vec<String>) -> Option<String> {
    let leading_comma = Regex::new(r"^,").unwrap();
    if depth == 3 {
        let mut visit = Vec::new();
        visit.push((String::new(), orig));
        while let Some((calls, remainder)) = visit.pop() {
            if remainder.len() == 0 {
                let calls: String = leading_comma.replace_all(&calls, "").into();
                let calls = calls.replace("0", "A");
                let calls = calls.replace("1", "B");
                let calls = calls.replace("2", "C");
                let mut current = current.clone();
                current.insert(0, calls);
                let stream = current.join("\n");
                return Some(stream);
            }
            for (call, routine) in current.iter().enumerate() {
                if remainder.len() < routine.len() && routine.starts_with(&remainder) {
                } else if !remainder.starts_with(routine) {
                    continue;
                }

                let start = usize::min(remainder.len(), routine.len() + 1);
                let remainder = String::from(&remainder[start..]);
                let calls = calls.clone() + "," + &call.to_string();
                let calls: String = leading_comma.replace_all(&calls, "").into();
                if calls.len() >= 20 {
                    continue;
                }
                visit.push((calls, remainder));
            }
        }
        return None;
    }

    for (idx, _) in s.match_indices(",") {
        if idx > 20 {
            break;
        }

        let substring = String::from(&s[..idx]);
        let mut s = s.clone();
        while s.starts_with(&substring) {
            s = String::from(&s[substring.len() + 1..]);
        }

        let mut current = current.clone();
        current.push(substring);
        if let Some(stream) = routines(orig.clone(), s, depth + 1, current.clone()) {
            return Some(stream);
        }
    }
    return None;
}

fn shorten_path(path: &str) -> String {
    let mut shortened = String::new();
    let mut move_count = 0;
    for mv in path.chars() {
        match mv {
            'L' | 'R' => {
                if move_count > 0 {
                    shortened = shortened + &move_count.to_string() + ",";
                }
                shortened = shortened + &mv.to_string() + ",";
                move_count = 0;
            }
            '1' => {
                move_count += 1;
            }
            c => {
                println!("wrong char {c}");
                panic!()
            }
        }
    }

    if move_count > 0 {
        shortened = shortened + &move_count.to_string();
    }

    shortened
}

#[cfg(test)]
mod test {
    use super::routines;

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        let test: String = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2".into();
        routines(test.clone(), test, 0, Vec::new()).ok_or("failed")?;
        Ok(())
    }
}
