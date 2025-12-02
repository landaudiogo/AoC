use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fmt::Display,
    fs,
};

fn main() {
    p1();
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Key {
    pos: (usize, usize),
    id: char,
}

type Robot = (usize, usize);
type RequiredKeys = HashSet<Key>;
type MissingKeys = HashSet<Key>;
type HeldKeys = HashSet<Key>;

enum Square {
    Empty,
    Wall,
    Key(Key),
    Door(char),
    Robot,
}

enum Move {
    Up,
    Right,
    Down,
    Left,
}

struct Grid {
    inner: Vec<Vec<Square>>,
    keys: HashMap<char, Key>,
    start: Robot,
    shortest_path: HashMap<(Robot, Key), (u64, RequiredKeys)>,
}

impl Grid {
    fn new(map: String) -> Self {
        let mut grid = Vec::new();
        let mut keys = HashMap::new();
        let mut robot = None;

        for (y, line) in map.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let square = match c {
                    '#' => Square::Wall,
                    '.' => Square::Empty,
                    '@' => {
                        robot = Some((y, x));
                        Square::Robot
                    }
                    c => {
                        if c.is_uppercase() {
                            Square::Door(c.to_ascii_lowercase())
                        } else {
                            let key = Key { pos: (y, x), id: c };
                            keys.insert(c, key.clone());
                            Square::Key(key)
                        }
                    }
                };
                row.push(square)
            }
            grid.push(row);
        }

        Self {
            inner: grid,
            keys: keys,
            start: robot.unwrap(),
            shortest_path: HashMap::new(),
        }
    }

    fn path(&mut self, robot: &Robot, key: &Key) -> (u64, RequiredKeys) {
        if let Some(shortest) = self.shortest_path.get(&(*robot, key.clone())) {
            return (shortest.0, shortest.1.clone());
        };

        let mut visited: HashSet<Robot> = HashSet::new();
        let mut visit: VecDeque<(u64, Robot, RequiredKeys)> = VecDeque::new();
        visit.push_back((0, *robot, HashSet::new()));
        while let Some((cost, curr, mut required_keys)) = visit.pop_front() {
            visited.insert(curr);
            match &self.inner[curr.0][curr.1] {
                Square::Key(k) => {
                    if k == key {
                        self.shortest_path
                            .insert((*robot, key.clone()), (cost, required_keys.clone()));
                        return (cost, required_keys);
                    }
                }
                Square::Door(k) => {
                    required_keys.insert(self.keys.get(k).unwrap().clone());
                }
                _ => {}
            }

            for dir in [Move::Up, Move::Right, Move::Down, Move::Left] {
                let Some(next) = self.move_robot(curr, dir) else {
                    continue;
                };
                if visited.get(&next).is_none() {
                    visit.push_back((cost + 1, next, required_keys.clone()));
                }
            }
        }
        panic!("we should be able to reach all keys");
    }

    fn move_robot(&self, curr: Robot, dir: Move) -> Option<Robot> {
        let pos = match dir {
            Move::Up => {
                if curr.0 == 0 {
                    None
                } else {
                    Some((curr.0 - 1, curr.1))
                }
            }
            Move::Down => {
                if curr.0 == self.inner.len() - 1 {
                    None
                } else {
                    Some((curr.0 + 1, curr.1))
                }
            }
            Move::Left => {
                if curr.1 == 0 {
                    None
                } else {
                    Some((curr.0, curr.1 - 1))
                }
            }
            Move::Right => {
                if curr.1 == self.inner[0].len() - 1 {
                    None
                } else {
                    Some((curr.0, curr.1 + 1))
                }
            }
        };
        match pos {
            Some(pos) => {
                if self.is_valid(pos) {
                    Some(pos)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn is_valid(&self, robot: Robot) -> bool {
        match self.inner[robot.0][robot.1] {
            Square::Wall => false,
            _ => true,
        }
    }
}
impl From<&Square> for char {
    fn from(value: &Square) -> Self {
        match value {
            Square::Wall => '#',
            Square::Empty => '.',
            Square::Key(k) => k.id,
            Square::Door(k) => k.to_ascii_uppercase(),
            Square::Robot => '@',
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner.iter() {
            for cell in row.iter() {
                write!(f, "{}", char::from(cell));
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn p1() {
    let mut grid = Grid::new(fs::read_to_string("../inputs/day18.txt").unwrap());
    let mut seen: HashMap<(Robot, String), u64> = HashMap::new();

    let mut visit: BTreeMap<(u64, Robot, String), (MissingKeys, HeldKeys)> = BTreeMap::new();
    visit.insert(
        (0, grid.start, String::new()),
        (
            HashSet::from_iter(grid.keys.values().map(|k| k.clone())),
            HashSet::new(),
        ),
    );
    while let Some(((cost, robot, seq), (missing_keys, held_keys))) = visit.pop_first() {
        if missing_keys.len() == 0 {
            println!("p1: arrived at solution {cost} {seq}");
            break;
        }
        for key in &missing_keys {
            let (added_cost, required_keys) = grid.path(&robot, &key);
            if required_keys
                .difference(&held_keys)
                .collect::<Vec<_>>()
                .len()
                == 0
            {
                let mut missing_keys = missing_keys.clone();
                let mut held_keys = held_keys.clone();
                let mut seq = seq.clone();

                missing_keys.remove(&key);
                held_keys.insert(key.clone());
                seq.push(key.id);
                let mut sorted_keys = seq.chars().collect::<Vec<char>>();
                sorted_keys.sort();
                let sorted_keys = String::from_iter(sorted_keys);
                let next_cost = cost + added_cost;

                match seen.get_mut(&(robot, sorted_keys.clone())) {
                    Some(val) => {
                        if next_cost < *val {
                            visit.insert((next_cost, key.pos, seq), (missing_keys, held_keys));
                            seen.insert((robot, sorted_keys), next_cost);
                        }
                    }
                    None => {
                        visit.insert((next_cost, key.pos, seq), (missing_keys, held_keys));
                        seen.insert((robot, sorted_keys), next_cost);
                    }
                }
            }
        }
    }
}
