use std::fs;
use std::ops::Range;

fn values_to_range(value: &str) -> Range<usize> {
    let (start, end) = value.split_once('-').unwrap();

    Range {
        start: start.parse::<usize>().unwrap(),
        end: end.parse::<usize>().unwrap(),
    }
}

fn contains(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start <= b.start && a.end >= b.end || b.start <= a.start && b.end >= a.end
}

pub fn main() {
    let input = fs::read_to_string("src/04/input.txt").expect("File not found");

    let result = input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (values_to_range(a), values_to_range(b)))
                .unwrap()
        })
        .filter(|(a, b)| contains(a, b))
        .count();

    println!("Result a: {}", result);
}
