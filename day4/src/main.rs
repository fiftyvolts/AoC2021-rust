use std::fmt::Display;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Square {
    v: i32,
    k: bool
}

#[derive(Debug)]
struct Board {
    sq: [[Square;5];5]
}

impl Board {
    fn _check_with_diag(&self) -> bool {
        if self.sq.iter().any(|r| r.iter().all(|c| c.k)) ||
           (0..5).any(|j|(0..5).all(|i|self.sq[i][j].k)) || 
           (0..5).all(|x| self.sq[x][x].k) ||
           (0..5).all(|x| self.sq[x][4-x].k) {
            true
        } else {
            false 
        }
    }
    fn check(&self) -> bool {
        //(0..4).any(|j| self.sq.iter().all(|r| r[j].k)
        if self.sq.iter().any(|r| r.iter().all(|c| c.k)) ||
           (0..5).any(|j|(0..5).all(|i|self.sq[i][j].k)) {
            true
        } else {
            false 
        }
    }
    fn score(&self, last: i32) -> i32 {
        let x = self.sq.iter()
        .flatten()
        .filter(|s| !s.k)
        .map(|s| s.v)
        .reduce(|v,t| t+v)
        .unwrap();
        x * last
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.sq {
            for c in r {
                write!(f, "{}{:2} ", if c.k {"*"} else {" "}, c.v)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
fn part1(input : &String) {
    let mut lines = input.lines().map(|x| String::from(x)).peekable();
    let nums : Vec<i32> = lines
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|x| i32::from_str_radix(x, 10).unwrap())
                    .collect();
    
    let mut boards = Vec::new();
    while lines.peek().is_some() {
        lines.next();
        let mut b = Board{sq: [[Square {v: 0, k: false};5];5]};
        for i in 0..5 {
            let mut j = 0;
            for col in lines.next().unwrap().split_whitespace() {
                b.sq[i][j].v = i32::from_str_radix(col, 10).unwrap();
                j+=1;
            }
        }

        boards.push(b);
    }

    for n in nums {
        for b in &mut boards {
            for r in & mut b.sq {
                for c in r {
                    if c.v == n {
                        c.k = true;
                    }
                } 
            }
            if b.check() {
                println!("{}", b.score(n));
                return
            }
        }
    }
}

fn part2(_input: &String) {

}