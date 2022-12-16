use rayon::prelude::*;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Valve {
    name: String,
    rate: u32,
    leads_to: Vec<String>,
}

fn parse_valve_row(line: &str) -> Valve {
    let line_split = line.split(" ").collect::<Vec<&str>>();

    let name = line_split[1].to_string();
    let rate = line_split[4].split("=").collect::<Vec<&str>>()[1]
        .replace(";", "")
        .parse::<u32>()
        .unwrap();
    let leads_to = line_split[9..]
        .iter()
        .map(|x| x.to_string().replace(",", ""))
        .collect::<Vec<String>>();

    Valve {
        name,
        rate,
        leads_to,
    }
}

fn travel(
    valve: Valve,
    valves: Vec<Valve>,
    timeleft: i32,
    paths: &Vec<Valve>,
    score: u32,
    velves_total: &usize,
) -> u32 {
    let mut updated_timeleft = timeleft;

    if updated_timeleft <= 0 {
        // println!("{}", score);
        return score;
    }

    let path_sum = paths.iter().fold(0, |acc, x| acc + x.rate);

    let mut updated_score = score + path_sum;

    if (paths.len()) == *velves_total {
        println!("{} {}", updated_score, paths.len());
        while updated_timeleft > 0 {
            updated_score += path_sum;
            updated_timeleft -= 1;
        }
        return updated_score;
    }

    let mut updated_paths = paths.clone();

    if !updated_paths.contains(&valve) && valve.rate != 0 {
        updated_score += path_sum;

        updated_timeleft -= 1;
        updated_paths.push(valve.clone());
    }

    if updated_timeleft - 1 <= 0 {
        return updated_score;
    }

    updated_timeleft -= 1;

    let moves = valves
        .clone()
        .into_par_iter()
        .filter(|x| valve.leads_to.contains(&x.name))
        .map(|next_valve| {
            travel(
                next_valve,
                valves.clone(),
                updated_timeleft,
                &updated_paths,
                updated_score,
                velves_total,
            )
        })
        .max();

    return moves.unwrap();
}

pub fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    let input = fs::read_to_string("src/16/input.txt").expect("File not found");

    let valves = input
        .lines()
        .map(|line| parse_valve_row(line))
        .collect::<Vec<Valve>>();

    let velves_total = valves.iter().filter(|x| x.rate != 0).count();

    let timeleft = 30;
    let score = 0;

    let new_valve = valves[0].clone();

    let max = travel(new_valve, valves, timeleft, &vec![], score, &velves_total);

    println!("{:?}", max);
}
