use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{stdin, Read};

fn input_txt() -> String {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).ok();
    buf
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Children {
    left: Box<SnailNum>,
    right: Box<SnailNum>,
}

enum SnailNum {
    Regular(i32),
    Pair(Children),
}

#[derive(Debug)]
enum SnailNumError {
    PareError(String),
}

impl SnailNum {
    fn parse(s: &str) -> Result<(SnailNum, usize), SnailNumError> {
        match s.chars().nth(0).unwrap() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Ok((SnailNum::Regular((s.as_bytes()[0] - b'0') as i32), 1))
            }
            '[' => {
                let mut idx = 1;
                let (sn1, len1) = SnailNum::parse(&s[idx..]).expect("[");
                idx += len1 + 1; /* comma */
                let (sn2, len2) = SnailNum::parse(&s[idx..]).expect(format!(", {}", len1).as_str());
                idx += len2 + 1; /* close square */
                Ok((
                    SnailNum::Pair(Children {
                        left: Box::new(sn1),
                        right: Box::new(sn2),
                    }),
                    idx,
                ))
            }
            _ => Err(SnailNumError::PareError(String::from(s))),
        }
    }

    fn reduce(&mut self) {
        loop {
            let (did_explode, _, _) = self.explode(0);
            if did_explode {
                continue;
            }
            if !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<i32>, Option<i32>) {
        if let SnailNum::Regular(_) = self {
            return (false, None, None);
        } else if let SnailNum::Pair(ch) = self {
            let mut exploded = false;
            let mut left_val: Option<i32> = None;
            let mut right_val: Option<i32> = None;

            if depth == 3 {
                if let SnailNum::Pair(chch) = &*ch.left {
                    exploded = true;
                    left_val = chch.left.as_ref().into();
                    right_val = chch.right.as_ref().into();

                    ch.left = Box::new(SnailNum::Regular(0));
                    right_val = ch.right.add_right_neighbor(right_val);
                } else if let SnailNum::Pair(chch) = &*ch.right {
                    exploded = true;
                    left_val = chch.left.as_ref().into();
                    right_val = chch.right.as_ref().into();

                    ch.right = Box::new(SnailNum::Regular(0));
                    left_val = ch.left.add_left_neighbor(left_val);
                }
                return (exploded, left_val, right_val);
            }

            (exploded, left_val, right_val) = ch.left.explode(depth + 1);
            if exploded {
                right_val = ch.right.add_right_neighbor(right_val);
                return (exploded, left_val, right_val);
            }

            (exploded, left_val, right_val) = ch.right.explode(depth + 1);
            if exploded {
                left_val = ch.left.add_left_neighbor(left_val);
                return (exploded, left_val, right_val);
            }
        }

        return (false, None, None);
    }

    fn add_right_neighbor(&mut self, val: Option<i32>) -> Option<i32> {
        if val.is_none() {
            return None;
        }

        match self {
            SnailNum::Regular(x) => {
                *x += val.unwrap();
                return None;
            }
            SnailNum::Pair(ch) => {
                return ch.left.add_right_neighbor(val);
            }
        }
    }

    fn add_left_neighbor(&mut self, val: Option<i32>) -> Option<i32> {
        if val.is_none() {
            return None;
        }

        match self {
            SnailNum::Regular(x) => {
                *x += val.unwrap();
                return None;
            }
            SnailNum::Pair(ch) => {
                return ch.right.add_left_neighbor(val);
            }
        }
    }

    fn split(&mut self) -> bool {
        let ch = match self {
            SnailNum::Regular(_) => {
                return false;
            }
            SnailNum::Pair(ch) => ch,
        };

        let mut splitted = match ch.left.as_mut() {
            SnailNum::Regular(x) => {
                if *x >= 10 {
                    ch.left = Box::new(SnailNum::Pair(Children {
                        left: Box::new(SnailNum::Regular(*x / 2)),
                        right: Box::new(SnailNum::Regular(*x / 2 + (*x & 1))),
                    }));
                    true
                } else {
                    false
                }
            }
            SnailNum::Pair(_) => ch.left.split(),
        };

        if splitted {
            return true;
        }

        splitted = match ch.right.as_mut() {
            SnailNum::Regular(x) => {
                if *x >= 10 {
                    ch.right = Box::new(SnailNum::Pair(Children {
                        left: Box::new(SnailNum::Regular(*x / 2)),
                        right: Box::new(SnailNum::Regular(*x / 2 + (*x & 1))),
                    }));
                    true
                } else {
                    false
                }
            }
            SnailNum::Pair(_) => ch.right.split(),
        };

        return splitted;
    }

    fn magnitude(&self) -> i32 {
        match self {
            SnailNum::Regular(x) => *x,
            SnailNum::Pair(ch) => ch.left.magnitude() * 3 + ch.right.magnitude() * 2,
        }
    }
}

impl From<&SnailNum> for Option<i32> {
    fn from(sn: &SnailNum) -> Option<i32> {
        match sn {
            SnailNum::Regular(x) => Some(*x),
            SnailNum::Pair(_) => None,
        }
    }
}

impl From<&str> for SnailNum {
    fn from(s: &str) -> SnailNum {
        SnailNum::parse(s).expect("Parse Error").0
    }
}

impl Debug for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular(x) => f.write_fmt(format_args!("{}", *x)),
            Self::Pair(ch) => f.write_fmt(format_args!("[{:?},{:?}]", ch.left, ch.right)),
        }
    }
}

impl Clone for SnailNum {
    fn clone(&self) -> SnailNum {
        match self {
            SnailNum::Regular(x) => SnailNum::Regular(*x),
            SnailNum::Pair(ch) => SnailNum::Pair(Children {
                left: Box::new((*ch.left).clone()),
                right: Box::new((*ch.right).clone()),
            }),
        }
    }
}

fn part1(input: &String) {
    let mut nums: VecDeque<SnailNum> = VecDeque::new();
    for line in input.lines() {
        nums.push_back(SnailNum::from(line));
    }
    let mut current = nums.pop_front().unwrap();
    current.reduce();

    while let Some(mut next) = nums.pop_front() {
        next.reduce();
        current = SnailNum::Pair(Children {
            left: Box::new(current),
            right: Box::new(next),
        });

        current.reduce();
    }

    println!("{:?}", current);
    println!("{}", current.magnitude());
}

fn part2(input: &String) {
    let mut nums: VecDeque<SnailNum> = VecDeque::new();
    for line in input.lines() {
        nums.push_back(SnailNum::from(line));
    }

    let mut mag: Vec<i32> = vec![];
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            let mut sum = SnailNum::Pair(Children {
                left: Box::new(nums[i].clone()),
                right: Box::new(nums[j].clone()),
            });
            sum.reduce();
            mag.push(sum.magnitude());
        }
    }

    mag.sort();
    println!("{}", mag.last().unwrap());
}
