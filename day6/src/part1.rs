use std::io::BufRead;

use crate::common::Matrix;

pub fn run<B: BufRead>(buf: B) -> u64 {
    let mut matrix = Matrix::new(buf);

    while let Some(_) = Matrix::step(
        &matrix.inner,
        &mut matrix.position,
        &mut matrix.visited,
        matrix.height,
        matrix.width,
    ) {}

    matrix.visited.len() as u64
}
