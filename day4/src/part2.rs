use crate::common::{self, convert_pos_i64_to_usize, Matrix};
use std::{collections::HashMap, io::BufRead};

struct Complement {
    offset: (i64, i64),
    direction: (i64, i64),
}

fn find_xmas(matrix: &mut Matrix, start: (usize, usize)) -> u64 {
    let mut sum = 0;
    let complements = HashMap::from([
        (
            (1, 1),
            [
                Complement {
                    offset: (0, 2),
                    direction: (1, -1),
                },
                Complement {
                    offset: (2, 0),
                    direction: (-1, 1),
                },
            ],
        ),
        (
            (1, -1),
            [
                Complement {
                    offset: (0, -2),
                    direction: (1, 1),
                },
                Complement {
                    offset: (2, 0),
                    direction: (-1, -1),
                },
            ],
        ),
        (
            (-1, 1),
            [
                Complement {
                    offset: (-2, 0),
                    direction: (1, 1),
                },
                Complement {
                    offset: (0, 2),
                    direction: (-1, -1),
                },
            ],
        ),
        (
            (-1, -1),
            [
                Complement {
                    offset: (-2, 0),
                    direction: (1, -1),
                },
                Complement {
                    offset: (0, -2),
                    direction: (-1, 1),
                },
            ],
        ),
    ]);

    for direction in complements.keys() {
        if let Some(_) = matrix.found.get(&(start, *direction)) {
            continue;
        }
        if _find_mas(matrix, start, *direction) {
            for complement in complements.get(direction).unwrap() {
                if let Ok(complement_pos) = matrix.get_relative(start, complement.offset) {
                    sum += _find_mas(matrix, complement_pos, complement.direction) as u64;
                }
            }
        }
    }

    sum
}

pub fn _find_mas(matrix: &mut Matrix, start: (usize, usize), direction: (i64, i64)) -> bool {
    let start = common::convert_pos_usize_to_i64(start);
    for i in 0..3 {
        let pos = (start.0 + i * direction.0, start.1 + i * direction.1);
        if !common::check_pos_bounds(&matrix, pos) {
            return false;
        }
        let pos = common::convert_pos_i64_to_usize(pos);

        match i {
            0 => {
                if !(matrix.inner[pos.0][pos.1] == 'M') {
                    return false;
                }
            }
            1 => {
                if !(matrix.inner[pos.0][pos.1] == 'A') {
                    return false;
                }
            }
            2 => {
                if !(matrix.inner[pos.0][pos.1] == 'S') {
                    return false;
                }
            }
            _ => {
                panic! {}
            }
        }
    }

    matrix
        .found
        .insert((convert_pos_i64_to_usize(start), direction));
    return true;
}

pub fn run<B: BufRead>(buf: B) -> u64 {
    let mut sum = 0;
    let mut matrix = Matrix::new(buf);

    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if !(matrix.inner[i][j] == 'M') {
                continue;
            }
            sum += find_xmas(&mut matrix, (i, j)) as u64;
        }
    }
    sum
}
