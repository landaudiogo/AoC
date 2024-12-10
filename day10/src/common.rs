use std::{char, collections::HashSet, io::BufRead};

#[derive(Debug)]
pub struct Matrix {
    pub inner: Vec<Vec<u8>>,
    pub height: usize,
    pub width: usize,
    pub found: HashSet<((usize, usize), (i64, i64))>,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut inner: Vec<Vec<u8>> = Vec::new();

        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            inner.push(line.trim_end().chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
            line.truncate(0);
        }

        let height = inner.len();
        let width = inner[0].len();
        Matrix {
            inner,
            height,
            width,
            found: HashSet::new(),
        }
    }

    pub fn get_relative(
        &self,
        current: (usize, usize),
        offset: (i64, i64),
    ) -> Result<(usize, usize), ()> {
        let current = convert_pos_usize_to_i64(current);
        let pos = (current.0 + offset.0, current.1 + offset.1);
        if !self.valid_position(pos) {
            return Err(());
        }

        Ok(convert_pos_i64_to_usize(pos))
    }

    pub fn find_hikes(&self, pos: (usize, usize)) -> HashSet<(usize, usize)> {
        if self.inner[pos.0][pos.1] == 9 {
            return HashSet::from([pos]);
        }

        let mut hikes = HashSet::new();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for direction in directions {
            let next = self.get_relative(pos, direction);
            if let Ok(next) = next {
                if self.valid_hike_step(pos, next) {
                    hikes.extend(self.find_hikes(next));
                }
            }
        }

        hikes
    }

    pub fn valid_position(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.height as i64 && pos.1 >= 0 && pos.1 < self.width as i64
    }

    fn valid_hike_step(&self, pos: (usize, usize), next: (usize, usize)) -> bool {
        self.inner[next.0][next.1] as i8 - self.inner[pos.0][pos.1] as i8 == 1
    }
}

fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}
