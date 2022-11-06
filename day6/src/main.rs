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
    let mut fish: Vec<i32> = input
        .split(",")
        .map(|d| i32::from_str_radix(d, 10).unwrap())
        .collect();

    for _ in 0..80 {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish.push(8);
                fish[i] = 6;
            } else {
                fish[i]-=1;
            }
        }
    }

    println!("{}", fish.len())
}

fn part2(input: &str) {
    let mut days: [i64;9] =[0;9];
    for i in input.split(",").map(|d| d.parse::<usize>().unwrap()) {
        days[i]+=1;
    }
    for _ in 0..256 {
        let babies = days[0];
        for day in 1..9 {
            days[day-1] = days[day];
        }
        days[6]+=babies;
        days[8]=babies;
    }
    println!("{}", days.iter().sum::<i64>());
}