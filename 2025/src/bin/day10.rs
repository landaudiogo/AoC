use std::{
    collections::BTreeSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    p1(&input);
}

type Button = u16;
type State = u16;

fn button_from_str(input: &str) -> Button {
    let mut button = 0;
    for light in input[1..(input.len() - 1)].chars().step_by(2) {
        let light = light.to_digit(10).unwrap();
        button |= 1 << light;
    }
    button
}

fn p1(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let mut splits = line.split_whitespace();
        let machine = splits.next().unwrap();
        let mut desired: u16 = 0;
        for (light, state) in machine[1..(machine.len() - 1)].chars().enumerate() {
            let state = if state == '#' { 1 } else { 0 };
            desired |= state << light;
        }

        let mut buttons: Vec<Button> = Vec::new();
        for split in splits {
            if split.starts_with("(") {
                buttons.push(button_from_str(split));
            }
        }

        let mut visit: BTreeSet<(u64, State)> = BTreeSet::new();
        visit.insert((0, 0));
        while let Some((cnt, state)) = visit.pop_first() {
            if state == desired {
                total += cnt;
                break;
            }

            for button in buttons.iter() {
                visit.insert((cnt + 1, state ^ button));
            }
        }
    }
    println!("p1: {total}");
}
