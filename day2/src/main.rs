use std::env;
use std::fs;


#[derive(Debug)]
enum Command {
    Forward(i32),
    Backward(i32),
    Up(i32),
    Down(i32)
}

fn read_input(path: &str) -> Option<Vec<Command>>{
    let txt = fs::read_to_string(path).expect(
        "couln't read file");
    let mut commands: Vec<Command> = Vec::new();
    let mut tokens = txt.as_str().split_whitespace();
    loop {
        let pair = (tokens.next(), tokens.next());
        let (c, v) = match pair.0.and(pair.1) {
            None => break,
            _ => (pair.0?, pair.1?)
        };

        commands.push(match c {
            "forward" => Command::Forward(v.parse().ok()?),
            "backward" => Command::Backward(v.parse().ok()?),
            "up" => Command::Up(v.parse().ok()?),
            "down" => Command::Down(v.parse().ok()?),
            _ => return None
        });
    }
    
    Some(commands)
}

fn main() {
    let mut arg_iter = env::args();
    arg_iter.next(); //bin w/ path
    arg_iter.next(); //bin
    let path = arg_iter.next().unwrap_or(String::from("ex1.txt"));
    let input = read_input(&path).expect("Malformed input");
    part1(&input);
    part2(&input);
}

fn part1 (input: &Vec<Command>) {
    let mut depth = 0;
    let mut dist = 0;
    for c in input {
        match c {
            Command::Forward(x) => dist += x,
            Command::Backward(x) => dist -= x,
            Command::Up(y) => depth -= y,
            Command::Down(y) => depth += y
        }
    }

    println!("part1 = {}", depth * dist);
}

fn part2(_input: &Vec<Command>) {
    println!("part2 = {}", 0);
}
