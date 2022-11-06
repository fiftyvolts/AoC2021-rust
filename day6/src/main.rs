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
