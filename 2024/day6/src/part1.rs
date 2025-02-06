use std::io::BufRead;

use crate::common::Matrix;

pub fn run<B: BufRead>(buf: B) -> u64 {
    let mut matrix = Matrix::new(buf);

    while let Some(_) = matrix.step() {}

    matrix.visited.len() as u64
}
