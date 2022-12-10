use std::fs;

const WIDTH: usize = 40;

#[derive(Debug)]
enum Step {
    AddX(i32),
    Noop,
}

pub fn main() {
    let input = fs::read_to_string("src/10/input.txt").expect("File not found");

    let steps = input
        .lines()
        .map(|line| match line.split_at(4) {
            ("addx", value) => Step::AddX(value.trim().parse().unwrap()),
            ("noop", _) => Step::Noop,
            _ => panic!("Unknown step"),
        })
        .collect::<Vec<Step>>();

    let mut cycle = 0;
    let mut cycles = vec![0; 240];

    cycles[0] = 0;

    for step in &steps {
        match step {
            Step::AddX(value) => {
                cycles[cycle + 2] += value;
                cycle += 2;
            }
            Step::Noop => {
                cycle += 1;
            }
        }
    }

    let mut register_x = cycles[0];

    println!("Result b:");
    cycles
        .iter()
        .map(|&cycle| {
            register_x += cycle;

            register_x
        })
        .enumerate()
        .for_each(|(index, value)| {
            if index != 0 && index % WIDTH == 0 {
                println!();
            }

            let column = (index % WIDTH) as i32;

            if column - 2 <= value && column >= value {
                print!("#");
            } else {
                print!(".");
            }
        });
}
