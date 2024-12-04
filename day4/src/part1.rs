use crate::common::{self, Matrix};
use std::io::BufRead;

fn find_xmas(matrix: &Matrix, start: (usize, usize)) -> u64 {
    let mut sum = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            sum += _find_xmas(matrix, start, (i, j)) as u64;
        }
    }

    sum
}

pub fn _find_xmas(matrix: &Matrix, start: (usize, usize), direction: (i64, i64)) -> bool {
    let start = common::convert_pos_usize_to_i64(start);
    for i in 1..4 {
        let pos = (start.0 + i * direction.0, start.1 + i * direction.1);
        if !common::check_pos_bounds(&matrix, pos) {
            return false;
        }
        let pos = common::convert_pos_i64_to_usize(pos);

        match i {
            1 => {
                if !(matrix.inner[pos.0][pos.1] == 'M') {
                    return false;
                }
            }
            2 => {
                if !(matrix.inner[pos.0][pos.1] == 'A') {
                    return false;
                }
            }
            3 => {
                if !(matrix.inner[pos.0][pos.1] == 'S') {
                    return false;
                }
            }
            _ => {
                panic! {}
            }
        }
    }
    return true;
}

pub fn run<B: BufRead>(buf: B) -> u64 {
    let mut sum = 0;
    let matrix = Matrix::new(buf);

    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if !(matrix.inner[i][j] == 'X') {
                continue;
            }
            sum += find_xmas(&matrix, (i, j)) as u64;
        }
    }
    sum
}
