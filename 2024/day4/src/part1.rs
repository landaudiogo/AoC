use crate::common::{self, Matrix};
use std::io::BufRead;

fn find_xmas(matrix: &mut Matrix, start: (usize, usize)) -> u64 {
    let mut sum = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            let direction = (i, j);
            if let Ok(start) = matrix.get_relative(start, direction) {
                sum += common::find_mas(matrix, start, direction) as u64;
            }
        }
    }

    sum
}

pub fn run<B: BufRead>(buf: B) -> u64 {
    let mut sum = 0;
    let mut matrix = Matrix::new(buf);

    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if !(matrix.inner[i][j] == 'X') {
                continue;
            }
            sum += find_xmas(&mut matrix, (i, j)) as u64;
        }
    }
    sum
}
