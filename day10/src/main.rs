use std::{env, fs};

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
#[derive(Debug)]
enum TokenErr {
    Err,
}

#[derive(PartialEq, Eq)]
enum Token {
    OP,
    CP,
    OS,
    CS,
    OC,
    CC,
    OA,
    CA,
}

trait ToToken {
    fn to_token(&self) -> Result<Token, TokenErr>;
}

impl ToToken for char {
    fn to_token(&self) -> Result<Token, TokenErr> {
        match *self {
            '(' => Ok(Token::OP {}),
            ')' => Ok(Token::CP {}),
            '[' => Ok(Token::OS {}),
            ']' => Ok(Token::CS {}),
            '{' => Ok(Token::OC {}),
            '}' => Ok(Token::CC {}),
            '<' => Ok(Token::OA {}),
            '>' => Ok(Token::CA {}),
            _ => Err(TokenErr::Err),
        }
    }
}

impl Token {
    fn fail_score(&self) -> i32 {
        match *self {
            Token::CP => 3,
            Token::CS => 57,
            Token::CC => 1197,
            Token::CA => 25137,
            _ => 0,
        }
    }

    fn complete_score(&self) -> i32 {
        match *self {
            Token::CP => 1,
            Token::CS => 2,
            Token::CC => 3,
            Token::CA => 4,
            _ => 0,
        }
    }

    fn opposite(&self) -> Token {
        match *self {
            Token::OP => Token::CP {},
            Token::CP => Token::OP {},
            Token::OS => Token::CS {},
            Token::CS => Token::OS {},
            Token::OC => Token::CC {},
            Token::CC => Token::OC {},
            Token::OA => Token::CA {},
            Token::CA => Token::OA {},
        }
    }

    fn is_open(&self) -> bool {
        match *self {
            Token::OP | Token::OS | Token::OC | Token::OA => true,
            _ => false,
        }
    }
}

impl std::string::ToString for Token {
    fn to_string(&self) -> String {
        match *self {
            Token::OP => String::from("("),
            Token::CP => String::from(")"),
            Token::OS => String::from("["),
            Token::CS => String::from("]"),
            Token::OC => String::from("{"),
            Token::CC => String::from("}"),
            Token::OA => String::from("<"),
            Token::CA => String::from(">"),
        }
    }
}
fn main() {
    let input = input_txt();
    part12(&input);
}

fn part12(input: &str) {
    let mut fail_score = 0;
    let mut finish_scores: Vec<u64> = Vec::new();

    'line: for line in input.lines() {
        let mut stack: Vec<Token> = Vec::new();
        let mut finish_score = 0;
        for c in line.chars() {
            let tok = c.to_token().unwrap();
            if tok.is_open() {
                stack.push(tok);
            } else if *stack.last().unwrap() == tok.opposite() {
                stack.pop();
            } else {
                fail_score += tok.fail_score();
                continue 'line;
            }
        }

        stack.reverse();
        for t in &stack {
            finish_score = finish_score * 5 + t.opposite().complete_score() as u64;
        }

        finish_scores.push(finish_score);
    }

    println!("{}", fail_score);

    finish_scores.sort();
    println!("{}", finish_scores.get(finish_scores.len() / 2).unwrap());
}
