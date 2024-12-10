use std::io::BufRead;

use crate::common::Matrix;

pub fn run<B: BufRead>(buf: B) -> u64 {
    let matrix = Matrix::new(buf);
    let mut hikes = 0;
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix.inner[i][j] == 0 {
                hikes += matrix.find_hikes((i, j)).len() as u64;
            }
        }
    }
    hikes
}
