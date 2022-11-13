use std::{collections::HashMap, env, fs};

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

fn adjacent(point: (usize, usize)) -> Vec<(usize, usize)> {
    let point_add = |d: (i32, i32)| -> Option<(usize, usize)> {
        Some((
            match (point.0 as i32 + d.0).try_into() {
                Ok(r) => r,
                Err(_) => return None {},
            },
            match (point.1 as i32 + d.1).try_into() {
                Ok(r) => r,
                Err(_) => return None {},
            },
        ))
    };
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(|d| point_add(*d))
    .filter(|p| p.0 < 10 && p.1 < 10)
    .collect()
}

fn main() {
    let input = input_txt();
    part12(&input);
}

fn part12(input: &str) {
    let mut oct: HashMap<(usize, usize), u8> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            oct.insert((x, y), c - b'0');
        }
    }

    let mut to_update: Vec<(usize, usize)> = Vec::new();

    let mut step_count = 0;
    let mut flash_count = 0;
    '_step: loop {
        step_count += 1;
        to_update.clear();
        to_update.append(&mut oct.keys().cloned().collect());

        while to_update.len() > 0 {
            let k = to_update.pop().unwrap();
            let v = *oct.get(&k).unwrap();
            if v == 9 {
                to_update.append(&mut adjacent(k));
                flash_count += 1;
            }

            if v < 10 {
                oct.insert(k, v + 1);
            }

        }

        let mut to_clear : Vec<(usize, usize)> = Vec::new();
        for (k, v) in &oct {
            if *v > 9 {
                to_clear.push(*k);
            }
        }

        if to_clear.len() == oct.len() {
            println!("All clear {}", step_count);
            break;
        }

        for k in to_clear {
            oct.insert(k, 0);
        }

        if step_count == 100 {
            println!("100 steps! {}", flash_count);
        }
    }
}
