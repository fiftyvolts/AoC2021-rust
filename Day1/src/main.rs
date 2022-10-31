use std::env;
use std::fs;

fn read_input(path: &str) -> Vec<i32> {
    let txt = fs::read_to_string(path).expect(
        "couln't read file");
    let mut v: Vec<i32> = Vec::new();
    for line in txt.lines() {
        v.push(line.parse().unwrap());
    }
    v
}

fn main() {
    let mut arg_iter = env::args();
    arg_iter.next(); //bin w/ path
    arg_iter.next(); //bin
    let path = arg_iter.next().unwrap_or(String::from("ex1.txt"));

    let input = read_input(&path);

    part1(&input.as_slice());
    
}

fn part1(mut input :&[i32]) {
    let mut last_depth = input[0];
    input = &input[1..];

    let mut count = 0i32;

    for depth in input {
        if *depth > last_depth {
            count+=1;
        }

        last_depth = *depth;
    }

    println!("part1 count = {}", count);
}