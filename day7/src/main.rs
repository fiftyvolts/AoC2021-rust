use std::env;
use std::fs;

fn input_txt() -> String {
    fs::read_to_string(
        env::args()
            .take(3)
            .last()
            .unwrap_or(String::from("ex1.txt")),
    )
    .ok()
    .unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let crabs : Vec<i64> = input.split(",").map(|x|x.parse().unwrap()).collect();
    let max = crabs.iter().max().unwrap();
    let mut least_fuel = i64::MAX;
    let mut least_pos: i64 = 0;
    'check_position: for pos in 0..(max+1) {
        let mut fuel = 0i64;
        for crab in &crabs {
            fuel += (pos - crab).abs();
            if fuel > least_fuel {
                continue 'check_position;
            }
        }

        if fuel < least_fuel {
            least_fuel = fuel;
            least_pos = pos;
        }
    }

    println!("{} {}", least_pos, least_fuel);
}

fn part2(input: &str) {
    let crabs : Vec<i64> = input.split(",").map(|x|x.parse().unwrap()).collect();
    let max = crabs.iter().max().unwrap();
    let mut least_fuel = i64::MAX;
    let mut least_pos: i64 = 0;
    for pos in 0..(max+1) {
        let mut fuel = 0i64;
        for crab in &crabs {
            let dist = (pos - crab).abs();
            if dist&1 == 1 {
                fuel += (dist / 2) * dist + dist;
            } else {
                fuel += (dist / 2) * (dist+1);
            }
        }
        
        if fuel < least_fuel {
            least_fuel = fuel;
            least_pos = pos;
        }
    }

    println!("{} {}", least_pos, least_fuel);
}