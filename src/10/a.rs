use std::fs;

#[derive(Debug)]
enum Step {
    AddX(i32),
    Noop,
}

fn register_value(step: usize, cycles: &Vec<i32>) -> i32 {
    cycles[0..step].iter().sum::<i32>()
}

fn signal_step(step: usize, cycles: &Vec<i32>) -> i32 {
    let value = register_value(step, cycles);

    value * step as i32
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

    cycles[0] = 1;

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

    let result = [
        signal_step(20, &cycles),
        signal_step(60, &cycles),
        signal_step(100, &cycles),
        signal_step(140, &cycles),
        signal_step(180, &cycles),
        signal_step(220, &cycles),
    ]
    .iter()
    .sum::<i32>();

    println!("Result a: {}", result);
}
