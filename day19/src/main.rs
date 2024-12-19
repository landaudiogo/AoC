use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Bound;

fn find_sequence(
    sequence: &[char],
    towels: &HashSet<Vec<char>>,
    cache: &mut HashMap<Vec<char>, u64>,
) -> u64 {
    if sequence.len() == 0 {
        return 1;
    }

    if let Some(val) = cache.get(sequence) {
        return *val;
    }

    let mut subseqs = Vec::new();
    for i in (1..sequence.len() + 1).rev() {
        let subseq = &sequence[0..i];
        if let Some(towel) = towels.get(subseq) {
            subseqs.push(towel);
        }
    }

    let mut total = 0;
    for towel in subseqs {
        total += find_sequence(&sequence[towel.len()..sequence.len()], towels, cache);
    }

    cache.insert(Vec::from_iter(sequence.iter().map(|c| *c)), total);
    return total;
}

fn main() {
    let mut buf = BufReader::new(File::open("input").unwrap());
    let mut line = String::new();
    let mut towels = HashSet::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 1 {
            break;
        }

        let mut elements = line.trim().split(", ").for_each(|towel| {
            towels.insert(Vec::from_iter(towel.chars()));
        });
        line.truncate(0);
    }

    let mut total = 0;
    let mut possibilities = 0;
    let mut cache = HashMap::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let sequence = Vec::from_iter(line.trim().chars());
        let combinations = find_sequence(&sequence, &towels, &mut cache) as u64;
        total += combinations;
        possibilities += (combinations > 0) as u64;

        line.truncate(0);
    }

    println!("p1 - {}", possibilities);
    println!("p2 - {}", total);
}
