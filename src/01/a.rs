use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/one/input.txt").expect("File not found");

    let result = input.lines().fold(vec![0], |mut acc, line| -> Vec<i32> {
        if line == "" {
            acc.push(0);
            return acc;
        }

        let last = acc.last_mut().unwrap();
        let current = line.parse().unwrap_or(0);

        *last += current;

        acc
    });

    if let Some(result) = result.iter().max() {
        println!("{}", result);
    }
}
