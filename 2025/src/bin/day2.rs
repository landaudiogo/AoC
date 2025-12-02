use std::fs::{self, File};

fn main() {
    p1();
}

fn p1() {
    let content = fs::read_to_string("./inputs/day2.txt").unwrap();
    let content = content.trim_end();
    let mut res = 0;
    let mut start = 0;

    loop {
        if start == content.len() {
            break;
        };
        let range = match &content[start..].chars().position(|c| c == ',') {
            Some(idx) => {
                let range = &content[start..(start + *idx)];
                start = start + idx + 1;
                range
            }
            None => {
                let range = &content[start..];
                start = content.len();
                range
            }
        };
        let mut chunks = range.split("-");
        let first: i64 = chunks.next().unwrap().parse().unwrap();
        let last: i64 = chunks.next().unwrap().parse().unwrap();

        for id in first..=last {
            let idstr = id.to_string();
            let idlen = idstr.len();

            if (idlen % 2) != 0 {
                continue;
            }

            let mid = idlen / 2;
            let left = &idstr[..mid];
            let right = &idstr[mid..];
            if left == right {
                res += id;
            }
        }
    }
    println!("p1: {res}")
}
