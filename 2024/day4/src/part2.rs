use crate::common::{self, Matrix};
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
        if common::find_mas(matrix, start, *direction) {
            for complement in complements.get(direction).unwrap() {
                if let Ok(complement_pos) = matrix.get_relative(start, complement.offset) {
                    sum += common::find_mas(matrix, complement_pos, complement.direction) as u64;
                }
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
            if !(matrix.inner[i][j] == 'M') {
                continue;
            }
            sum += find_xmas(&mut matrix, (i, j)) as u64;
        }
    }
    sum
}
