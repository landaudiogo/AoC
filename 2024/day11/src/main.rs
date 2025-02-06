use std::{collections::HashMap, fs::File, io::Read};

fn count_stones(
    stone: u64,
    blinks: usize,
    final_blink: usize,
    cache: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if blinks == final_blink {
        return 1;
    }

    if let Some(stones) = cache.get(&(stone, final_blink - blinks)) {
        return *stones;
    }

    let mut sum = 0;
    if stone == 0 {
        sum += count_stones(1, blinks + 1, final_blink, cache);
    } else if stone.to_string().len() % 2 == 0 {
        let stone = stone.to_string();
        let len = stone.len();
        let digits = stone.chars();
        let left: String = digits.take(len / 2).collect();
        let right: String = stone.chars().skip(len / 2).collect();

        sum += count_stones(left.parse::<u64>().unwrap(), blinks + 1, final_blink, cache);
        sum += count_stones(
            right.parse::<u64>().unwrap(),
            blinks + 1,
            final_blink,
            cache,
        );
    } else {
        sum += count_stones(stone * 2024, blinks + 1, final_blink, cache);
    }
    cache.insert((stone, final_blink - blinks), sum);

    sum
}

fn main() {
    let mut line = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut line)
        .unwrap();
    let line = line.trim();
    let stones: Vec<_> = line
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    let mut sum = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        sum += count_stones(stone, 0, 75, &mut cache);
    }

    dbg!(sum);
}
