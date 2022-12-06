use std::fs;

#[derive(Debug)]
struct Move(i32, i32, i32);

fn parse_step(line: &str) -> Move {
    let trimmed = line.replace("move ", "");

    let (count, rest) = trimmed.split_once(" from ").unwrap();

    let (from, to) = rest.split_once(" to ").unwrap();

    Move(
        from.parse::<i32>().unwrap() - 1,
        to.parse::<i32>().unwrap() - 1,
        count.parse::<i32>().unwrap(),
    )
}

pub fn main() {
    let input = fs::read_to_string("src/05/input.txt").expect("File not found");

    let (top, bottom) = input.split_once("\n\n").unwrap();

    let steps = bottom.lines().map(parse_step).collect::<Vec<_>>();

    let mut output = Vec::<Vec<char>>::with_capacity(20);

    top.lines().rev().skip(1).for_each(|line| {
        for (i, x) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if i >= output.len() {
                let mut temp: Vec<char> = Vec::with_capacity(20);
                temp.push(*x.iter().find(|&x| x.is_alphabetic() || *x == ' ').unwrap());

                output.push(temp);
            } else {
                output[i].push(*x.iter().find(|&x| *x == ' ' || x.is_alphabetic()).unwrap());
            }
        }
    });

    steps.iter().for_each(|step| {
        for _ in 0..step.2 {
            let index = output[step.0 as usize]
                .iter()
                .rposition(|&x| x != ' ')
                .unwrap();

            let temp = output[step.0 as usize].swap_remove(index);

            output[step.1 as usize].push(temp);
        }
    });

    let result = output
        .iter()
        .map(|x| x.iter().last().unwrap())
        .collect::<Vec<_>>();

    println!("Result a: {:?}", result);
}
