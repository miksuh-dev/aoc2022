use serde_json::Value;
use std::cmp::Ordering;
use std::fs;

fn compare(left: Value, right: Value) -> Ordering {
    if Value::is_array(&left) && Value::is_array(&right) {
        let mut left_iter = Value::as_array(&left).unwrap().iter();
        let mut right_iter = Value::as_array(&right).unwrap().iter();

        loop {
            let l = left_iter.next();
            let r = right_iter.next();

            if l.is_none() && r.is_none() {
                return Ordering::Equal;
            }

            if l.is_some() && r.is_none() {
                return Ordering::Greater;
            }

            if l.is_none() && r.is_some() {
                return Ordering::Less;
            }

            if l.is_some() && r.is_some() {
                let left = l.unwrap();
                let right = r.unwrap();

                if Value::is_number(left) && Value::is_number(right) {
                    let left_f64 = left.as_f64().unwrap();
                    let right_f64 = right.as_f64().unwrap();

                    if left_f64 < right_f64 {
                        return Ordering::Less;
                    }

                    if left_f64 > right_f64 {
                        return Ordering::Greater;
                    }
                }

                if Value::is_array(&left) && Value::is_array(&right) {
                    if compare(left.clone(), right.clone()) != Ordering::Equal {
                        return compare(left.clone(), right.clone());
                    }
                }

                if Value::is_array(&left) && Value::is_number(&right) {
                    return compare(left.clone(), Value::Array(vec![right.clone()]));
                }

                if Value::is_number(&left) && Value::is_array(&right) {
                    return compare(Value::Array(vec![left.clone()]), right.clone());
                }
            }
        }
    }

    if Value::is_array(&left) && Value::is_number(&right) {
        return compare(left, Value::Array(vec![right]));
    }

    if Value::is_number(&left) && Value::is_array(&right) {
        return compare(Value::Array(vec![left]), right);
    }

    return Ordering::Equal;
}

pub fn main() {
    let mut input = fs::read_to_string("src/13/input.txt").expect("File not found");

    input += "\n[[2]]";
    input += "\n[[6]]";

    let mut signals = input
        .split("\n\n")
        .flat_map(|group| {
            let (left, right) = group.split_once("\n").unwrap();

            let left_json: Value = serde_json::from_str(left).expect("Invalid JSON");
            let right_json: Value = serde_json::from_str(right).expect("Invalid JSON");

            vec![left_json, right_json]
        })
        .collect::<Vec<Value>>();

    signals.sort_by(|a, b| compare(a.clone(), b.clone()));

    let result = signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| {
            let value = signal.to_string();

            value == "[[2]]" || value == "[[6]]"
        })
        .map(|(index, _)| index + 1)
        .product::<usize>();

    println!("Result b: {}", result);
}
