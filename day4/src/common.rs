use std::{
    char,
    collections::{HashMap, HashSet},
    io::BufRead,
};

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

pub fn check_pos_bounds(matrix: &Matrix, pos: (i64, i64)) -> bool {
    pos.0 >= 0 && pos.0 < matrix.height as i64 && pos.1 >= 0 && pos.1 < matrix.width as i64
}

pub fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

pub fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}
