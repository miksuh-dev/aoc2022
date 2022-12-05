use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/05/input.txt").expect("File not found");

    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|&c| c == "")
        .map(|group| {
            println!("{:?}", group);

            group
        })
        .for_each(|group| {
            println!("{:?}", group);
        });
}
