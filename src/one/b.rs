use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() {
    let file = "src/one/input.txt";

    let mut persons = match read_lines(file) {
        Ok(lines) => {
            let mut persons = vec![];
            let mut current = vec![];

            for line in lines {
                if let Ok(ip) = line {
                    if ip == "" {
                        let sum: i32 = current.iter().sum();
                        persons.push(sum);
                        current = vec![];
                    } else {
                        current.push(ip.parse().unwrap());
                    }
                }
            }

            persons
        }
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    persons.sort_by(|a, b| b.cmp(a));

    let output = persons.iter().take(3).sum::<i32>();

    println!("Top three summed: {}", output);
}
