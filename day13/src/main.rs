use std::io::{stdin, Read};

fn input_txt() -> String {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).ok();
    buf
}

fn main() {
    let input = input_txt();
    part12(&input);
}
fn part12(_input: &String) {
    
}