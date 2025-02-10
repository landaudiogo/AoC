use std::{collections::HashMap, io::BufRead};

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

    let mut layers = Vec::new();

    for (height, row) in image.windows(WIDTH).step_by(WIDTH).enumerate() {
        if height % HEIGHT == 0 {
            layers.push(HashMap::new());
        }

        let layer = layers.last_mut().unwrap();
        for v in row {
            *layer.entry(v).or_insert(0) += 1;
        }
    }

    let most0 = layers
        .iter()
        .fold(None, |acc, layer| {
            let acc = if let Some(acc) = acc {
                acc
            } else {
                return Some(layer);
            };

            match (acc.get(&0), layer.get(&0)) {
                (Some(acc0), Some(layer0)) => {
                    if layer0 < acc0 {
                        Some(layer)
                    } else {
                        Some(acc)
                    }
                }
                (Some(_), None) => Some(acc),
                (None, Some(_)) => Some(layer),
                (None, None) => Some(acc),
            }
        })
        .unwrap();
    println!("{:?}", most0[&1] * most0[&2]);
}
