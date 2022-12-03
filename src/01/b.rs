use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/one/input.txt").expect("File not found");

    let mut result = input.lines().fold(vec![0], |mut acc, line| -> Vec<i32> {
        if line == "" {
            acc.push(0);

            return acc;
        }

        let last = acc.last_mut().unwrap();
        let current = line.parse().unwrap_or(0);

        *last += current;

        acc
    });

    result.sort_by(|a, b| b.cmp(a));

    println!("{}", result.iter().take(3).sum::<i32>());
}
