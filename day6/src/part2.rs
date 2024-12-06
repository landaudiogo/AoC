use std::io::BufRead;

use crate::common::Matrix;

pub fn run<B: BufRead>(buf: B) -> u64 {
    let mut matrix = Matrix::new(buf);
    let mut sum = 0;

    while let Some(is_loop) = matrix.step_with_loop() {
        sum += is_loop as u64;
    }

    sum
}
