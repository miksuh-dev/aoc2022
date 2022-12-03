use std::fs;

enum Outcome {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

enum Hand {
    ROCK,
    PAPER,
    SCISSORS,
}

fn own_pick(own: &str) -> Hand {
    match own {
        "X" => Hand::ROCK,
        "Y" => Hand::PAPER,
        "Z" => Hand::SCISSORS,
        _ => panic!("Invalid own hand"),
    }
}

fn enemy_pick(enemy: &str) -> Hand {
    match enemy {
        "A" => Hand::ROCK,
        "B" => Hand::PAPER,
        "C" => Hand::SCISSORS,
        _ => panic!("Invalid opponent hand"),
    }
}

fn outcome(own: &Hand, enemy: &Hand) -> Outcome {
    match own {
        Hand::ROCK => match enemy {
            Hand::ROCK => Outcome::DRAW,
            Hand::PAPER => Outcome::LOSE,
            Hand::SCISSORS => Outcome::WIN,
        },
        Hand::PAPER => match enemy {
            Hand::ROCK => Outcome::WIN,
            Hand::PAPER => Outcome::DRAW,
            Hand::SCISSORS => Outcome::LOSE,
        },
        Hand::SCISSORS => match enemy {
            Hand::ROCK => Outcome::LOSE,
            Hand::PAPER => Outcome::WIN,
            Hand::SCISSORS => Outcome::DRAW,
        },
    }
}

fn score(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::WIN => 6,
        Outcome::DRAW => 3,
        Outcome::LOSE => 0,
    }
}

fn hand_value(hand: Hand) -> i32 {
    match hand {
        Hand::ROCK => 1,
        Hand::PAPER => 2,
        Hand::SCISSORS => 3,
    }
}

pub fn main() {
    let input = fs::read_to_string("src/two/input.txt").expect("File not found");

    let result = input.lines().fold(0, |acc, line| -> i32 {
        let row = line.split(" ").collect::<Vec<&str>>();

        let enemy = enemy_pick(row[0]);
        let own = own_pick(row[1]);

        let outcome = outcome(&own, &enemy);

        acc + score(outcome) + hand_value(own)
    });

    println!("Result a: {}", result);
}
