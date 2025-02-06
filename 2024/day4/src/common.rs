use std::{char, collections::HashSet, io::BufRead};

pub struct Matrix {
    pub inner: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
    pub found: HashSet<((usize, usize), (i64, i64))>,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut inner: Vec<Vec<char>> = Vec::new();

        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            inner.push(line.trim_end().chars().collect());
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

    pub fn valid_position(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.height as i64 && pos.1 >= 0 && pos.1 < self.width as i64
    }
}

fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}

pub fn find_mas(matrix: &mut Matrix, start: (usize, usize), direction: (i64, i64)) -> bool {
    for (i, c) in ['M', 'A', 'S'].into_iter().enumerate() {
        let offset = (i as i64 * direction.0, i as i64 * direction.1);
        let pos = match matrix.get_relative(start, offset) {
            Ok(pos) => pos,
            Err(_) => return false,
        };

        if matrix.inner[pos.0][pos.1] != c {
            return false;
        }
    }

    matrix.found.insert((start, direction));

    return true;
}
