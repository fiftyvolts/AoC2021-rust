#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
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
    part1(&input, false);
    part1(&input, true);
}
#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Vent {
    fn from(s: &str) -> Vent {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }
        let points = RE
            .captures(s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|p| i32::from_str_radix(p.unwrap().as_str(), 10).unwrap())
            .collect::<Vec<i32>>();
        Vent {
            x1: points[0],
            y1: points[1],
            x2: points[2],
            y2: points[3],
        }
    }

    fn straight(&self) -> bool {
        (self.x1 == self.x2) || (self.y1 == self.y2)
    }
}

fn part1(input: &str, include_diag: bool) {
    let vents: Vec<Vent> = input
        .lines()
        .map(|l| Vent::from(l))
        .filter(|v| include_diag || v.straight())
        .collect();

    let xmax = vents.iter().map(|v| [v.x1, v.x2]).flatten().max().unwrap();
    let ymax = vents.iter().map(|v| [v.y1, v.y2]).flatten().max().unwrap();

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for v in vents {
        let mut xcur = v.x1;
        let mut ycur = v.y1;

        loop {
            if !map.contains_key(&(xcur, ycur)) {
                map.insert((xcur, ycur), 0);
            }

            *map.get_mut(&(xcur, ycur)).unwrap() += 1;

            if xcur == v.x2 && ycur == v.y2 {
                break;
            }

            if xcur < v.x2 {
                xcur += 1;
            } else if xcur > v.x2 {
                xcur -= 1;
            }

            if ycur < v.y2 {
                ycur += 1;
            } else if ycur > v.y2 {
                ycur -= 1;
            }
        }
    }
    let mut count = 0;
    for v in map.values() {
        if *v >= 2 {
            count += 1;
        }
    }
    println!("{} {} {}", xmax, ymax, count);
}
