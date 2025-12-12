use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader, Read},
};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input);
    p1(&mut input);
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Some,
            '.' => Cell::None,
            c => panic!("present contained {c}"),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Some => '#',
            Cell::None => '.',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    None,
    Some,
}

type Arrangement = Vec<Vec<Cell>>;
struct Present {
    id: u64,
    arrangements: HashSet<Arrangement>,
}

impl Present {
    fn new(id: u64, mut init: Arrangement) -> Self {
        let mut arrangements = HashSet::new();
        for mut shape in [
            flip_ns(&init),
            flip_we(&init),
            flip_sw_ne(&init),
            flip_nw_se(&init),
            init,
        ] {
            for _ in 0..4 {
                shape = rotate90(&shape);
                arrangements.insert(shape.clone());
            }
        }

        Self { id, arrangements }
    }
}

fn print_arrangement(arr: &Arrangement) {
    for row in arr {
        for c in row {
            print!("{}", char::from(c))
        }
        print!("\n");
    }
}

fn flip_ns(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arri) in (0..=2).enumerate() {
        for (resj, arrj) in (0..=2).rev().enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn flip_we(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arri) in (0..=2).rev().enumerate() {
        for (resj, arrj) in (0..=2).enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn flip_sw_ne(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arrj) in (0..=2).rev().enumerate() {
        for (resj, arri) in (0..=2).rev().enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn flip_nw_se(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arrj) in (0..=2).enumerate() {
        for (resj, arri) in (0..=2).enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }
    res
}

fn rotate90(arr: &Arrangement) -> Arrangement {
    let mut res = Vec::with_capacity(3);
    for _ in 0..3 {
        res.push(vec![Cell::None; 3]);
    }

    for (resi, arrj) in (0..=2).rev().enumerate() {
        for (resj, arri) in (0..=2).enumerate() {
            res[resi][resj] = arr[arri][arrj];
        }
    }

    res
}

fn p1(input: &[u8]) {
    let mut buf_reader = BufReader::new(input);
    let mut line = String::new();
    while let Ok(len) = buf_reader.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let Some(colon_idx) = line.find(":") else {
            panic!("id should contain colon")
        };

        let Ok(id): Result<u64, _> = line[..colon_idx].parse() else {
            break;
        };

        line.clear();
        let mut init: Arrangement = Vec::new();
        while let Ok(len) = buf_reader.read_line(&mut line) {
            if len == 1 {
                break;
            }
            init.push(line.trim().chars().map(|c| Cell::from(c)).collect());
            line.clear();
        }

        let present = Present::new(id, init);

        line.clear();
    }
}
