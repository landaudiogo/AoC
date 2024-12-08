use std::{char, collections::HashMap, io::BufRead};

#[derive(Debug)]
pub struct Matrix {
    // pub inner: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
    pub antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut width = 0;
        let mut i = 0;
        let mut antennas = HashMap::new();
        // let mut inner: Vec<Vec<char>> = Vec::new();

        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            width = 0;
            let chars = line.trim_end().chars();
            chars.enumerate().for_each(|(j, c)| {
                if c != '.' {
                    let mut positions = antennas.entry(c).or_insert_with(|| Vec::new());
                    positions.push((i, j));
                }
                width += 1;
            });

            line.truncate(0);
            i += 1;
        }

        let height = i;
        Self {
            height,
            width,
            antennas,
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

pub fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

pub fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}
