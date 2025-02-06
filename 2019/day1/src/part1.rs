use std::io::BufRead;

pub fn run<B: BufRead>(buf: B) {
    let mut sum = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mass = line.parse::<i64>().unwrap();
        let fuel = (mass / 3) - 2;
        sum += fuel;
    }
    println!("total fuel {sum}")
}
