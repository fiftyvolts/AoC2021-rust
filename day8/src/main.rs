#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
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
    part12(&input);
}

type Signal = u8;
const LINE_A: Signal = 1 << 1;
const LINE_B: Signal = 1 << 2;
const LINE_C: Signal = 1 << 3;
const LINE_D: Signal = 1 << 4;
const LINE_E: Signal = 1 << 5;
const LINE_F: Signal = 1 << 6;
const LINE_G: Signal = 1 << 7;
lazy_static! {
    static ref SIGNAL_MAP: HashMap<Signal, u32> = HashMap::from([
        (LINE_A | LINE_B | LINE_C | LINE_E | LINE_F | LINE_G, 0),
        (LINE_C | LINE_F, 1),
        (LINE_A | LINE_C | LINE_D | LINE_E | LINE_G, 2),
        (LINE_A | LINE_C | LINE_D | LINE_F | LINE_G, 3),
        (LINE_B | LINE_C | LINE_D | LINE_F, 4),
        (LINE_A | LINE_B | LINE_D | LINE_F | LINE_G, 5),
        (LINE_A | LINE_B | LINE_D | LINE_E | LINE_F | LINE_G, 6),
        (LINE_A | LINE_C | LINE_F, 7),
        (
            LINE_A | LINE_B | LINE_C | LINE_D | LINE_E | LINE_F | LINE_G,
            8
        ),
        (LINE_A | LINE_B | LINE_C | LINE_D | LINE_F | LINE_G, 9)
    ]);
    static ref DIGIT_MAP: HashMap<u32, Signal> = HashMap::from([
        (0, LINE_A | LINE_B | LINE_C | LINE_E | LINE_F | LINE_G),
        (1, LINE_C | LINE_F),
        (2, LINE_A | LINE_C | LINE_D | LINE_E | LINE_G),
        (3, LINE_A | LINE_C | LINE_D | LINE_F | LINE_G),
        (4, LINE_B | LINE_C | LINE_D | LINE_F),
        (5, LINE_A | LINE_B | LINE_D | LINE_F | LINE_G),
        (6, LINE_A | LINE_B | LINE_D | LINE_E | LINE_F | LINE_G),
        (7, LINE_A | LINE_C | LINE_F),
        (
            8,
            LINE_A | LINE_B | LINE_C | LINE_D | LINE_E | LINE_F | LINE_G
        ),
        (9, LINE_A | LINE_B | LINE_C | LINE_D | LINE_F | LINE_G)
    ]);
}

#[derive(Debug)]
enum SignalParseErr {
    Err,
}

trait AsU32 {
    fn as_u32(&self) -> Result<u32, SignalParseErr>;
}

impl AsU32 for Signal {
    fn as_u32(&self) -> Result<u32, SignalParseErr> {
        match SIGNAL_MAP.get(self) {
            Some(x) => Ok(*x),
            None => Err(SignalParseErr::Err),
        }
    }
}

trait AsSignal {
    fn as_signal(&self) -> Result<Signal, SignalParseErr>;
}

impl AsSignal for &str {
    fn as_signal(&self) -> Result<Signal, SignalParseErr> {
        let mut signal: Signal = 0;
        for c in self.as_bytes() {
            signal |= match c {
                b'a' => LINE_A,
                b'b' => LINE_B,
                b'c' => LINE_C,
                b'd' => LINE_D,
                b'e' => LINE_E,
                b'f' => LINE_F,
                b'g' => LINE_G,
                _ => return Err(SignalParseErr::Err),
            };
        }
        Ok(signal)
    }
}


impl AsSignal for String {
    fn as_signal(&self) -> Result<Signal, SignalParseErr> {
        self.as_str().as_signal()
    }
}

impl AsSignal for u32 {
    fn as_signal(&self) -> Result<Signal, SignalParseErr> {
        match DIGIT_MAP.get(self) {
            Some(x) => Ok(*x),
            None => Err(SignalParseErr::Err),
        }
    }
}


fn dump_cross_map(cross_map: &HashMap<Signal, Signal>) {
    let mut pairs = cross_map
        .iter()
        .map(|p| (*p.1, *p.0))
        .collect::<Vec<(Signal, Signal)>>();
    pairs.sort();
    println!(
        "{}",
        pairs
            .iter()
            .map(|p| format!("{}:{:08b}", p.0, p.1))
            .collect::<Vec<String>>()
            .join(", ")
    );
}

fn part12(input: &str) {
    let mut counter = 0;
    let mut sum = 0u32;
    for line in input.lines() {
        let (readout, display) = {
            let pair: Vec<&str> = line.split("|").collect();
            (pair[0], pair[1])
        };

        let mut cross_map: HashMap<Signal, Signal> = HashMap::new();
        let mut lookup: [Signal; 10] = [0; 10];
        let mut input_digits: Vec<Signal> = readout
            .split_whitespace()
            .map(|s| s.as_signal().unwrap())
            .collect();
        input_digits.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()));

        lookup[1] = input_digits[0];
        lookup[4] = input_digits[2];
        lookup[7] = input_digits[1];
        lookup[8] = input_digits[9];

        for i in 3..6 {
            if lookup[1] & input_digits[i] == lookup[1] {
                lookup[3] = input_digits[i];
                break;
            }
        }

        lookup[0] = !(lookup[4] & (!lookup[1]) & lookup[3]) & lookup[8];
        lookup[9] = ((lookup[8] ^ lookup[3]) & !lookup[4]) ^ lookup[8];

        for i in 6..9 {
            if input_digits[i] != lookup[0] && input_digits[i] != lookup[9] {
                lookup[6] = input_digits[i];
            }
        }

        lookup[5] = lookup[6] & lookup[9];
        for i in 3..6 {
            if input_digits[i] != lookup[5] && input_digits[i] != lookup[3] {
                lookup[2] = input_digits[i];
            }
        }

        for i in 0..10 {
            cross_map.insert(lookup[i], i as u8);
        }

        dump_cross_map(&cross_map);

        let mapped_display = display.split_whitespace().map(|s| *cross_map.get(&s.as_signal().unwrap()).unwrap()).collect::<Vec<Signal>>();

        for d in &mapped_display {
            if *d == 1 || *d == 4 || *d == 7 || *d == 8 {
                counter += 1;
            }
        }
        sum += mapped_display[0] as u32 * 1000 + mapped_display[1] as u32 * 100 + mapped_display[2] as u32 * 10 + mapped_display[3] as u32;

    }
    println!("{}, {}", counter, sum);
}
