use std::io::BufRead;

use crate::common::{self, Matrix, Square};

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut matrix = Matrix::new(&mut buf);
    let moves = common::read_moves(&mut buf);
    matrix.process_moves(moves);

    let mut total = 0;
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix.inner[i][j] == Square::Box {
                total += 100 * i + j;
            }
        }
    }
    total as u64
}
