pub fn run() {
    let mut total = 0;
    for i in 172851..=675869 {
        let digits = i
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let mut has_double = false;
        let mut sequence = 1;
        let mut is_growing = true;
        let mut prev = digits[0];
        for d in digits[1..].iter() {
            if *d == prev {
                sequence += 1;
            } else {
                if sequence == 2 {
                    has_double = true;
                }
                sequence = 1;
            }

            if *d < prev {
                is_growing = false
            }

            prev = *d;
        }

        if (has_double || sequence == 2) && is_growing {
            total += 1;
        }
    }

    println!("{total}");
}
