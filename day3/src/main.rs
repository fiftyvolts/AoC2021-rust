
use std::collections::HashSet;
use std::env;
use std::fs;

fn input_txt() -> String {
    let path = env::args().take(3).last().unwrap_or(
        String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn part1(input : &String) {
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

fn part2(input : &String) {

    let mut off = 0;
    let mut search : HashSet<String> =
        input.split_whitespace().map(|x| String::from(x)).collect();

    while search.len() > 1 {

        let mut ones = HashSet::new();
        let mut zeroes = HashSet::new();
        let mut count = 0;

        for bit_str in &search {
            if bit_str.as_bytes()[off] == b'1' {
                ones.insert(bit_str.clone());
                count+=1;
            } else {
                zeroes.insert(bit_str.clone());
            }

        }

        
        if search.len() - count <= count {
            search = ones;
        } else {
            search = zeroes;
        }
        
        off += 1;
    }

    let  o2 = i32::from_str_radix(
        search.iter().next().unwrap(), 2).unwrap();
          
    
    search = input.split_whitespace().map(|x| String::from(x)).collect();

    off = 0;
    while search.len() > 1 {

        let mut ones = HashSet::new();
        let mut zeroes = HashSet::new();
        let mut count = 0;

        for bit_str in &search {
            if bit_str.as_bytes()[off] == b'1' {
                ones.insert(bit_str.clone());
                count+=1;
            } else {
                zeroes.insert(bit_str.clone());
            }
        }

        
        if search.len() - count > count {
            search = ones;
        } else {
            search = zeroes;
        }  
        off += 1;
    }

    let  co2 = i32::from_str_radix(
        search.iter().next().unwrap(), 2).unwrap();
       
    println!("o2 {} co2 {} product {}", o2, co2, o2 * co2);
}