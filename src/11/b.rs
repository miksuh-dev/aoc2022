use std::fs;

const ROUNT_COUNT: i64 = 10000;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    test: i64,
    true_target: i64,
    false_target: i64,
    inspect_count: i64,
}

impl Monkey {
    fn new(text: &str) -> Monkey {
        let mut monkey = text.split("\n");

        // Skip name field
        monkey.next();

        Monkey {
            items: Monkey::get_starting_items(monkey.next().unwrap()),
            operation: Monkey::get_operation(monkey.next().unwrap()).to_string(),
            test: Monkey::get_test(monkey.next().unwrap()),
            true_target: Monkey::get_target(monkey.next().unwrap()),
            false_target: Monkey::get_target(monkey.next().unwrap()),
            inspect_count: 0,
        }
    }

    fn get_starting_items(text: &str) -> Vec<i64> {
        text.split_once("Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    fn get_operation(text: &str) -> &str {
        text.split_once("Operation: new = ").unwrap().1
    }

    fn get_test(text: &str) -> i64 {
        text.split_once("Test: divisible by ")
            .unwrap()
            .1
            .parse::<i64>()
            .unwrap()
    }

    fn get_target(text: &str) -> i64 {
        text.split_once(":")
            .unwrap()
            .1
            .split(" ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap()
    }

    fn get_worry_level(&self, operation: &str, &item: &i64) -> i64 {
        let content = operation.split(" ").collect::<Vec<&str>>();

        let left_str = content[0];
        let operation_str = content[1];
        let right_str = content[2];

        let left = if left_str == "old" {
            item
        } else {
            left_str.parse::<i64>().unwrap()
        };

        let right = if right_str == "old" {
            item
        } else {
            right_str.parse::<i64>().unwrap()
        };

        match operation_str {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => panic!("Unknown operation"),
        }
    }

    fn do_test(&self, &item: &i64) -> i64 {
        if item % self.test == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

struct Playground {
    monkeys: Vec<Monkey>,
}

impl Playground {
    fn new(monkeys: Vec<Monkey>) -> Playground {
        Playground { monkeys }
    }

    pub fn do_round(&mut self, divider: &i64) {
        let mut monkeys_clone = self.monkeys.clone();

        for monkey_index in 0..monkeys_clone.len() {
            let mut monkey = monkeys_clone[monkey_index].clone();

            while monkey.items.len() > 0 {
                let item = monkey.items.remove(0) % divider;

                let worry_level = monkey.get_worry_level(&monkey.operation, &item);

                let target_num = monkey.do_test(&worry_level);
                let mut target = monkeys_clone[target_num as usize].clone();

                target.items.push(worry_level);

                monkeys_clone[target_num as usize] = target;

                monkey.inspect_count += 1;
            }

            monkeys_clone[monkey_index] = monkey;
        }

        self.monkeys = monkeys_clone;
    }
}

pub fn main() {
    let input = fs::read_to_string("src/11/input.txt").expect("File not found");

    let monkeys = input
        .split("\n\n")
        .map(|text| Monkey::new(text))
        .collect::<Vec<Monkey>>();

    let mut playground = Playground::new(monkeys);

    let divider = playground.monkeys.iter().map(|x| x.test).product::<i64>();

    for _ in 0..ROUNT_COUNT {
        playground.do_round(&divider);
    }

    let mut monkeys_clone = playground.monkeys.clone();
    monkeys_clone.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    let result = monkeys_clone
        .iter()
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspect_count)
        .to_string();

    println!("Result b: {}", result);
}
