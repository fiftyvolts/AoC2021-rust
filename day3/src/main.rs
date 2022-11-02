use std::env;
use std::fs;

fn input_txt() -> String {
    let mut arg_iter = env::args();
    arg_iter.next(); //bin w/ path
    arg_iter.next(); //bin
    let path = arg_iter.next().unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(input);
}

fn part1(input : String) {
    let tokens = input.split_whitespace();

    let mut one_counts = [0i32;32];
    let mut count = 0;
    let mut width = 0;

    for bits_str in tokens {
        let bits = bits_str.bytes();
        width = bits.len();


        for (i, b) in bits.rev().enumerate() {
            match b {
                b'1' => one_counts[i]+=1,
                _ => ()
            }
        }
        count+=1;
    }

    println!("{}", count);
    let mut gamma = 0u32;
    for i in 0..one_counts.len()-1 {
        gamma |= if one_counts[i] > count/2 {1<<i} else {0};
    }
    let epsilon = (!gamma) & !(u32::MAX<<width);
    println!("gamma {} epsilon {} product {} ",
                gamma, epsilon, gamma * epsilon);
}
