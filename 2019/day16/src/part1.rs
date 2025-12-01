use std::io::BufRead;

const SIGNAL_MUL: usize = 1;

fn calc_next(signal: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![0; signal.len()];
    let mut acc = 0;
    let mut cumulative: Vec<i64> = signal
        .iter()
        .map(|v| {
            acc += *v as i64;
            acc
        })
        .collect();
    cumulative.insert(0, 0);

    for i in 0..(signal.len()) {
        let mut total = 0;
        for (n, j) in (i..(signal.len())).step_by((i + 1) * 2).enumerate() {
            let negative = if n % 2 == 0 { 1 } else { -1 };
            let repeats = usize::min(signal.len() - j, i + 1);
            let base = (j / (cumulative.len())) as i64 * (*cumulative.last().unwrap());
            let cum_l = base + cumulative[(j % cumulative.len()) as usize];
            let base = (j + repeats) / (cumulative.len());
            let cum_r = base as i64 + cumulative[(j + repeats) % cumulative.len()];
            let val = negative * (cum_r - cum_l);
            total += val;
        }
        res[i] = total.abs() % 10;
    }
    res
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut signal = String::new();
    buf.read_to_string(&mut signal);
    let mut signal: Vec<i64> = signal
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    let mut signal = signal.repeat(SIGNAL_MUL);
    let offset = 0;

    for i in 0..100 {
        signal = calc_next(&signal);
    }
    println!("{:?}", &signal[offset..(offset + 8)]);
}
