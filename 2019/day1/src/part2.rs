use std::io::BufRead;

pub fn run<B: BufRead>(buf: B) {
    let mut sum = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut mass = line.parse::<i64>().unwrap();
        while mass > 0 {
            let fuel = (mass / 3) - 2;
            mass = fuel;
            if fuel > 0 {
                sum += fuel;
            }
        }
    }
    println!("total fuel {sum}")
}
