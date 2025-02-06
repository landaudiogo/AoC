use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};

use crate::common::{self, convert_pos_i64_to_usize, convert_pos_usize_to_i64, Matrix};

pub fn run<B: BufRead>(buf: B) -> usize {
    let matrix = Matrix::new(buf);
    let mut antinodes = HashSet::new();
    for (_, positions) in &matrix.antennas {
        for pair in positions.into_iter().combinations(2) {
            let (first, second) = (
                convert_pos_usize_to_i64(*pair[0]),
                convert_pos_usize_to_i64(*pair[1]),
            );
            antinodes.insert(convert_pos_i64_to_usize(first));
            antinodes.insert(convert_pos_i64_to_usize(second));

            let direction: (i64, i64) = (second.0 - first.0, second.1 - first.1);
            let mut curr = second;
            while let Ok(pos) = matrix.get_relative(convert_pos_i64_to_usize(curr), direction) {
                antinodes.insert(pos);
                curr = convert_pos_usize_to_i64(pos);
            }

            let direction: (i64, i64) = (first.0 - second.0, first.1 - second.1);
            let mut curr = first;
            while let Ok(pos) = matrix.get_relative(convert_pos_i64_to_usize(curr), direction) {
                antinodes.insert(pos);
                curr = convert_pos_usize_to_i64(pos);
            }
        }
    }

    antinodes.len()
}
