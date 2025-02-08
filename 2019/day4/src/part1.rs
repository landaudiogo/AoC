pub fn run() {
    let mut total = 0;
    for i in 172851..=675869 {
        let digits = i
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let mut has_double = false;
        let mut is_growing = true;
        let mut prev = digits[0];
        for d in digits[1..].iter() {
            if *d == prev {
                has_double = true
            } else if *d < prev {
                is_growing = false
            }
            prev = *d;
        }

        if has_double && is_growing {
            total += 1;
        }
    }

    println!("{total}");
}
