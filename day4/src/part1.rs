use std::{char, io::BufRead};

struct Matrix {
    inner: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut inner: Vec<Vec<char>> = Vec::new();

        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            inner.push(line.trim_end().chars().collect());
            line.truncate(0);
        }

        let height = inner.len();
        let width = inner[0].len();
        Matrix {
            inner,
            height,
            width,
        }
    }
}

fn check_pos_bounds(matrix: &Matrix, pos: (i64, i64)) -> bool {
    pos.0 >= 0 && pos.0 < matrix.height as i64 && pos.1 >= 0 && pos.1 < matrix.width as i64
}

fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}

fn search_negative_slope(matrix: &Matrix, start: (usize, usize), direction: bool) -> bool {
    let direction = if direction { 1 } else { -1 };
    let start = convert_pos_usize_to_i64(start);
    for i in 1..4 {
        let pos = (start.0 + i * direction, start.1 + i * direction);
        if !check_pos_bounds(&matrix, pos) {
            return false;
        }
        let pos = convert_pos_i64_to_usize(pos);

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

fn search_positive_slope(matrix: &Matrix, start: (usize, usize), direction: bool) -> bool {
    let direction = if direction { 1 } else { -1 };
    let start = convert_pos_usize_to_i64(start);
    for i in 1..4 {
        let pos = (start.0 - i * direction, start.1 + i * direction);
        if !check_pos_bounds(&matrix, pos) {
            return false;
        }
        let pos = convert_pos_i64_to_usize(pos);

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

fn search_horizontal(matrix: &Matrix, start: (usize, usize), direction: bool) -> bool {
    let direction = if direction { 1 } else { -1 };
    for i in 1..4 {
        if (start.1 as i64 + (i * direction) >= matrix.width as i64)
            || (start.1 as i64 + (i * direction) < 0)
        {
            return false;
        }
        let pos = (start.0, (start.1 as i64 + (i * direction)) as usize);

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

fn search_vertical(matrix: &Matrix, start: (usize, usize), direction: bool) -> bool {
    let direction = if direction { 1 } else { -1 };
    let start = convert_pos_usize_to_i64(start);
    for i in 1..4 {
        let pos = (start.0 + i * direction, start.1);
        if !check_pos_bounds(&matrix, pos) {
            return false;
        }
        let pos = convert_pos_i64_to_usize(pos);

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

fn find_xmas(matrix: &Matrix, start: (usize, usize)) -> u64 {
    let mut sum = 0;

    sum += search_horizontal(&matrix, start, true) as u64;
    sum += search_horizontal(&matrix, start, false) as u64;
    sum += search_vertical(&matrix, start, true) as u64;
    sum += search_vertical(&matrix, start, false) as u64;
    sum += search_positive_slope(&matrix, start, true) as u64;
    sum += search_positive_slope(&matrix, start, false) as u64;
    sum += search_negative_slope(&matrix, start, true) as u64;
    sum += search_negative_slope(&matrix, start, false) as u64;

    sum
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
