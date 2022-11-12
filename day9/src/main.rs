use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

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

fn adjacent(point: (i32, i32)) -> [(i32, i32); 4] {
    [
        (point.0 - 1, point.1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1),
        (point.0, point.1 + 1),
    ]
}

fn part12(input: &String) {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut jmax = 0;

    for line in input.lines() {
        for d in line.bytes() {
            map.insert((i, j), (d - b'0') as i32);
            j += 1;
        }
        i += 1;
        if j > jmax {
            jmax = j;
        }
        j = 0;
    }
    let imax = i;

    let mut risk = 0;
    let mut low_points: Vec<(i32, i32)> = Vec::new();

    for i in 0..imax {
        for j in 0..jmax {
            let mut adjacent_risk: Vec<i32> = adjacent((i, j))
                .iter()
                .map(|ap| *map.get(&(ap.0, ap.1)).unwrap_or(&9))
                .collect();
            adjacent_risk.sort();

            let r = *map.get(&(i, j)).unwrap();
            if r < *adjacent_risk.get(0).unwrap() {
                risk += r + 1;
                low_points.push((i, j));
            }
        }
    }

    let mut basins: Vec<HashSet<(i32, i32)>> = Vec::new();
    for low_point in low_points {
        let mut found: HashSet<(i32, i32)> = HashSet::new();
        let mut stack = vec![low_point];

        loop {
            let curr_point = match stack.pop() {
                Some(x) => x,
                None => break,
            };
            found.insert(curr_point);

            let curr_risk = *map.get(&curr_point).unwrap();

            for adjacent_point in adjacent(curr_point) {
                if found.contains(&adjacent_point) {
                    continue;
                }

                let adjacent_risk = match map.get(&adjacent_point) {
                    Some(x) => *x,
                    None => continue,
                };

                if adjacent_risk != 9 && adjacent_risk >= curr_risk {
                    stack.push(adjacent_point);
                }
            }
        }
        basins.push(found);
    }

    println!("{}", risk);

    basins.sort_by(|a,b| a.len().cmp(&b.len()));
    basins.reverse();

    println!("{}", basins.get(0).unwrap().len() * basins.get(1).unwrap().len() * basins.get(2).unwrap().len());
}
