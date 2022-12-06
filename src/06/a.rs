use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/06/input.txt").expect("File not found");

    let result = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .position(|(i, _)| {
                    if i < 4 {
                        return false;
                    }

                    let mut code = line[i - 4..i].chars().collect::<Vec<_>>();

                    code.sort();
                    code.dedup();

                    code.len() == 4
                })
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    println!("Result a: {:?}", result);
}
