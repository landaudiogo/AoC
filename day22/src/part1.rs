use std::io::BufRead;

pub fn run<B: BufRead>(mut buf: B) {
    let mut line = String::new();
    let mut total = 0;
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut secret = line.trim().parse::<u64>().unwrap();
        for _ in 0..2000 {
            secret = (secret ^ (secret << 6)) % 16777216;
            secret = (secret ^ (secret >> 5)) % 16777216;
            secret = (secret ^ (secret << 11)) % 16777216;
        }
        total += secret;

        line.truncate(0);
    }
    dbg!(total);
}
