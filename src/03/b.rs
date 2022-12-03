use std::fs;

const PRIORITY_ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(c: char) -> usize {
    let index = PRIORITY_ITEMS.find(c);
    match index {
        Some(i) => i,
        None => panic!("Invalid character"),
    };

    index.unwrap() + 1
}

pub fn main() {
    let input = fs::read_to_string("src/03/input.txt").expect("File not found");

    let result = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let first = chunk[0].chars().collect::<Vec<char>>();
            let second = chunk[1].chars().collect::<Vec<char>>();
            let third = chunk[2].chars().collect::<Vec<char>>();

            (first, second, third)
        })
        .map(|(first, second, third)| {
            first
                .iter()
                .filter(|c| second.contains(*c) && third.contains(*c))
                .map(|c| priority(*c))
                .collect::<Vec<usize>>()
        })
        .map(|mut matching| {
            matching.dedup();
            matching
        })
        .flatten()
        .sum::<usize>();

    println!("Result b: {}", result);
}
