use std::fs;
use std::env;

fn input_txt() -> String {
    fs::read_to_string(env::args().take(3).last().unwrap_or(
        String::from("ex1.txt"))).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn part1(input : &String) {
}

fn part2(input: &String) {

}