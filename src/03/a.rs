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
        .map(|line| line.split_at(line.len() / 2))
        .flat_map(|(start, end)| start.chars().find(|c| end.contains(*c)).map(priority))
        .sum::<usize>();

    println!("Result a: {}", result);
}
