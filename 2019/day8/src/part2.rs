use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    buf.read_to_string(&mut line).unwrap();
    let line = line.trim();
    let image = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut layers: Vec<Vec<u32>> = (0..HEIGHT).map(|_| vec![2; WIDTH]).collect();

    let mut seen = HashSet::new();

    for (height, pixels) in image.windows(WIDTH).step_by(WIDTH).enumerate() {
        let row = height % HEIGHT;
        for (col, v) in pixels.iter().enumerate() {
            if seen.get(&(row, col)).is_some() {
                continue;
            }

            if *v == 2 {
                continue;
            }

            layers[row][col] = *v;
            seen.insert((row, col));
        }
    }

    for row in layers {
        for v in row {
            let c = if v == 1 { '█' } else { ' ' };
            print!("{}", c);
        }
        println!()
    }
}
