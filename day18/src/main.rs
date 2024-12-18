use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Square {
    Safe,
    Unsafe,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Safe => '.',
            Self::Unsafe => '#',
        };
        write!(f, "{}", c)
    }
}

struct Matrix {
    inner: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new<B: BufRead>(mut buf: B) -> Self {
        let mut inner = Vec::with_capacity(71);
        for _ in 0..71 {
            let init = [Square::Safe; 71];
            inner.push(Vec::from_iter(init));
        }

        let mut line = String::new();
        let mut i = 0;
        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            let mut elements = line.trim().split(",");
            let x = elements.next().unwrap().parse::<usize>().unwrap();
            let y = elements.next().unwrap().parse::<usize>().unwrap();
            inner[y][x] = Square::Unsafe;

            line.truncate(0);
            i += 1;
            if i == 1024 {
                break;
            }
        }
        Self {
            width: inner.len(),
            height: inner[0].len(),
            inner,
        }
    }

    fn find_shortest_path_len(&self, start: (usize, usize), end: (usize, usize)) -> Option<u64> {
        let mut visit = Vec::new();
        visit.push((start, 0));
        let mut visited: HashMap<(usize, usize), u64> = HashMap::new();
        visited.insert(start, 1);
        while let Some((pos, cost)) = visit.pop() {
            for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if let Some(next) = self.relative_pos(pos, offset) {
                    if self.inner[next.1][next.0] == Square::Unsafe {
                        continue;
                    }
                    let v = visited.entry(next).or_insert_with(|| {
                        visit.push((next, cost + 1));
                        cost + 1
                    });
                    if cost + 1 < *v {
                        visited.insert(next, cost + 1);
                        visit.push((next, cost + 1));
                    }
                }
            }
        }

        visited.remove(&end)
    }

    fn relative_pos(&self, start: (usize, usize), offset: (i64, i64)) -> Option<(usize, usize)> {
        if (start.0 as i64 + offset.0 < self.width as i64)
            && ((start.1 as i64 + offset.1) < self.height as i64)
            && (start.0 as i64 + offset.0 >= 0)
            && (start.1 as i64 + offset.1 >= 0)
        {
            return Some((
                (start.0 as i64 + offset.0) as usize,
                (start.1 as i64 + offset.1) as usize,
            ));
        }
        None
    }

    fn find_first_byte(&mut self, bytes: Vec<(usize, usize)>) -> (usize, usize) {
        let mut curr = bytes.len();
        let mut true_map = BTreeSet::new();
        let mut false_map = BTreeSet::new();
        loop {
            let f = *false_map.first().unwrap_or(&bytes.len());
            let t = *true_map.last().unwrap_or(&0);
            if (f - t) == 1 {
                break;
            }
            curr = t + (f - t) / 2;
            for byte in &bytes[0..curr] {
                if self.inner[byte.1][byte.0] == Square::Unsafe {
                    panic!()
                }
                self.inner[byte.1][byte.0] = Square::Unsafe;
            }

            if let Some(_) = self.find_shortest_path_len((0, 0), (70, 70)) {
                true_map.insert(curr);
            } else {
                false_map.insert(curr);
            }

            for byte in &bytes[0..curr] {
                self.inner[byte.1][byte.0] = Square::Safe;
            }
        }

        bytes[curr - 1]
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner.iter() {
            for s in row {
                write!(f, "{:?}", s);
            }
            write!(f, "\n");
        }
        write!(f, "{},{}", self.width, self.height)
    }
}

fn main() {
    let mut buf = BufReader::new(File::open("input").unwrap());
    let mut matrix = Matrix::new(&mut buf);
    let len = matrix.find_shortest_path_len((0, 0), (70, 70));
    println!("p1 - {:?}", len);

    let mut bytes = Vec::new();
    let mut line = String::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut elements = line.trim().split(",");
        let x = elements.next().unwrap().parse::<usize>().unwrap();
        let y = elements.next().unwrap().parse::<usize>().unwrap();
        bytes.push((x, y));

        line.truncate(0);
    }

    println!("p2 - {:?}", matrix.find_first_byte(bytes));
}
