use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Key {
    Value(char),
    Invalid,
}

fn seq_to_string(seq: &[char]) -> String {
    seq.into_iter().collect::<String>()
}

fn arrow_from_dir(dir: (i64, i64)) -> char {
    match dir {
        (0, 1) => '>',
        (0, -1) => '<',
        (1, 0) => 'v',
        (-1, 0) => '^',
        _ => panic!(),
    }
}

#[derive(Debug)]
struct Keypad {
    inner: Vec<Vec<Key>>,
    char_map: HashMap<char, (i64, i64)>,
    width: i64,
    height: i64,
}

impl Keypad {
    fn new(keypad: Vec<Vec<Key>>) -> Self {
        let mut char_map = HashMap::new();
        for i in 0..keypad.len() {
            for j in 0..keypad[0].len() {
                if let Key::Value(val) = keypad[i][j] {
                    char_map.insert(val, (i as i64, j as i64));
                }
            }
        }
        Self {
            width: keypad[0].len() as i64,
            height: keypad.len() as i64,
            inner: keypad,
            char_map,
        }
    }

    fn move_to_key(&self, from: char, key: char) -> HashSet<Vec<char>> {
        let mut sequences = HashSet::new();
        let mut visit = VecDeque::new();
        visit.push_back((*self.char_map.get(&from).unwrap(), Vec::new()));
        let mut min_length = None;
        let mut visited = HashSet::new();
        while let Some((pos, mut sequence)) = visit.pop_front() {
            visited.insert(pos);
            match self.inner[pos.0 as usize][pos.1 as usize] {
                Key::Value(val) => {
                    if let Some(min_len) = min_length {
                        if sequence.len() + 1 > min_len {
                            continue;
                        }
                    }
                    if val == key {
                        sequence.push('A');
                        min_length = Some(sequence.len());
                        sequences.insert(sequence);
                    } else {
                        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let next = self.next_pos(pos, dir);
                            let next = if let Some(next) = next {
                                next
                            } else {
                                continue;
                            };

                            if visited.get(&next).is_none() {
                                let mut sequence = sequence.clone();
                                sequence.push(arrow_from_dir(dir));
                                visit.push_back((next, sequence));
                            }
                        }
                    }
                }
                Key::Invalid => {}
            }
        }
        sequences
    }

    fn next_pos(&self, pos: (i64, i64), dir: (i64, i64)) -> Option<(i64, i64)> {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if self.is_valid_pos(next) {
            Some(next)
        } else {
            None
        }
    }

    fn is_valid_pos(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.height && pos.1 >= 0 && pos.1 < self.width
    }
}

struct Chain {
    numpad: Keypad,
    arrowpad: Keypad,
    cache: HashMap<(String, i64), i64>,
    len: i64,
}

impl Chain {
    pub fn new(len: i64) -> Self {
        let numlayout = vec![
            vec![Key::Value('7'), Key::Value('8'), Key::Value('9')],
            vec![Key::Value('4'), Key::Value('5'), Key::Value('6')],
            vec![Key::Value('1'), Key::Value('2'), Key::Value('3')],
            vec![Key::Invalid, Key::Value('0'), Key::Value('A')],
        ];
        let arrowlayout = vec![
            vec![Key::Invalid, Key::Value('^'), Key::Value('A')],
            vec![Key::Value('<'), Key::Value('v'), Key::Value('>')],
        ];
        Self {
            numpad: Keypad::new(numlayout.clone()),
            arrowpad: Keypad::new(arrowlayout.clone()),
            cache: HashMap::new(),
            len,
        }
    }

    fn robot_sequence(&mut self, sequence: &[char], depth: i64) -> i64 {
        let mut res = Vec::new();
        for pair in sequence.into_iter().tuple_windows::<(&char, &char)>() {
            if let Some(min_len) = self.cache.get(&(seq_to_string(&[*pair.0, *pair.1]), depth)) {
                res.push(*min_len);
                continue;
            }

            let keypad = if depth == self.len {
                &self.numpad
            } else {
                &self.arrowpad
            };

            if depth == 1 {
                let seqs_for_pair = keypad.move_to_key(*pair.0, *pair.1);
                let shortest = seqs_for_pair.iter().next().unwrap().len() as i64;
                res.push(shortest);
                self.cache
                    .insert((seq_to_string(&[*pair.0, *pair.1]), depth), shortest);
                continue;
            }

            let mut lengths = BTreeSet::new();
            for mut seq in keypad.move_to_key(*pair.0, *pair.1) {
                seq.insert(0, 'A');
                lengths.insert(self.robot_sequence(seq.as_slice(), depth - 1));
            }

            let shortest = lengths.first().unwrap();
            self.cache
                .insert((seq_to_string(&[*pair.0, *pair.1]), depth), *shortest);
            res.push(*shortest);
        }
        res.iter().fold(0, |acc, x| acc + x)
    }
}

pub fn main() {
    // let depth = 3; // for part1
    let depth = 26;

    let mut chain = Chain::new(depth);
    let mut sum = 0;
    let input = File::open("input").unwrap();
    for line in io::BufReader::new(input).lines() {
        let line = line.unwrap();
        let num_sequence = line.trim();
        let mut num_sequence = num_sequence.chars().collect::<Vec<char>>();
        num_sequence.insert(0, 'A');

        let shortest = chain.robot_sequence(num_sequence.as_slice(), depth);
        let num_part = num_sequence[1..num_sequence.len() - 1]
            .into_iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        sum += shortest * num_part as i64;

        println!("{} * {}", shortest, num_part);
    }
    println!("{sum}");
}
