#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    io::Read,
    env, fs::File
};

struct PermutationIter {
    regs: Vec<usize>,
    done: bool,
}

impl PermutationIter {
    fn new(size: usize) -> PermutationIter {
        PermutationIter {
            regs: vec![0; size],
            done: false,
        }
    }
}

impl Iterator for PermutationIter {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut indexes: Vec<usize> = (0..self.regs.len()).collect();
        let ret = self.regs.iter().map(|i| indexes.remove(*i)).collect();

        let mut carry = 1;
        for i in 0..self.regs.len() {
            self.regs[i] = self.regs[i] + carry;
            if self.regs[i] >= (self.regs.len() - i) {
                self.regs[i] = 0;
                carry = 1;
            } else {
                carry = 0;
            }
        }

        if carry == 1 {
            self.done = true;
        }

        Some(ret)
    }
}

lazy_static! {
    static ref FACING: Vec<Vec<usize>> = PermutationIter::new(3).collect();
}

fn input_txt() -> std::io::Result<String> {
    let mut buf = String::new();
    let in_file = env::args().nth(1).unwrap();
    println!("{}", in_file);
    File::open(in_file)?.read_to_string(&mut buf).ok();
    Ok(buf)
}

fn main() -> std::io::Result<()> {
    let input = input_txt()?;
    part1(&input);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    p: [i64; 3],
}

impl Point {
    fn origin_ds2(&self) -> i64 {
        self.p[0] * self.p[0] + self.p[1] * self.p[1] + self.p[2] * self.p[2]
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Delta {
    ds: [i64; 3],
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    start: Point,
    _end: Point,
    delta: Delta,
}

impl Delta {
    fn reckon(&self, other: Self) -> Option<([usize; 3], u8)> {
        for face in &*FACING {
            for flip in 0..7 {
                let x = if flip & 1 == 0 {
                    other.ds[face[0]]
                } else {
                    -other.ds[face[0]]
                };
                let y = if flip & 2 == 0 {
                    other.ds[face[1]]
                } else {
                    -other.ds[face[1]]
                };
                let z = if flip & 4 == 0 {
                    other.ds[face[2]]
                } else {
                    -other.ds[face[2]]
                };
                let translated = [x, y, z];
                if self.ds == translated {
                    return Some(([face[0], face[1], face[2]], flip));
                }
            }
        }
        None
    }
}

impl PartialEq for Delta {
    fn eq(&self, other: &Self) -> bool {
        self.reckon(*other).is_some()
    }
}

impl Hash for Delta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.ds[0] * self.ds[0] + self.ds[1] * self.ds[1] + self.ds[2] * self.ds[2]).hash(state);
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    beacon_set: HashSet<Point>,
    rays: HashMap<Delta, Vec<Ray>>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            beacons: vec![],
            beacon_set: HashSet::new(),
            rays: HashMap::new(),
        }
    }

    fn add_beacon(&mut self, added: Point) {
        if self.beacon_set.contains(&added) {
            return;
        }

        for beacon in &self.beacons {
            let r = Ray::new(*beacon, added);
            let for_d = self.rays.entry(r.delta).or_insert(vec![]);
            for_d.push(r);
        }
        self.beacons.push(added);
        self.beacon_set.insert(added);
    }

    fn match_points(&self, from: &Scanner, limit: usize) -> (usize, Option<Translation>) {
        let consensus_tx_opt = self.txfn(&from);
        if consensus_tx_opt.is_none() {
            return (0, None);
        }

        let tx = consensus_tx_opt.unwrap();
        let mut common_count = 0usize;
        for b in &from.beacons {
            if self.beacon_set.contains(&tx.translate(*b)) {
                common_count += 1;
                if common_count >= limit {
                    break;
                }
            }
        }
        return (common_count, consensus_tx_opt);
    }

    fn extend(&mut self, src: &Scanner) {
        let tx = self.txfn(&src).unwrap();
        for b in &src.beacons {
            self.add_beacon(tx.translate(*b));
        }
    }

    fn txfn(&self, from: &Scanner) -> Option<Translation> {
        let mut txs: HashMap<Translation, usize> = HashMap::new();

        for (delta, rays1) in &self.rays {
            if !from.rays.contains_key(delta) {
                continue; // not present in other scanner
            }

            let rays2 = from.rays.get(delta).unwrap();

            //at this point, there could be multiple rays with the same delta but on in
            //one scanner range and one in the other, so we'll generate the tx and
            //count how many times the same tx shows up to have a consensus
            for r1 in rays1 {
                for r2 in rays2 {
                    let tx = Translation::new(*r1, *r2);
                    txs.entry(tx)
                    .and_modify(|c| {
                        *c += 1;
                    })
                    .or_insert(1);
                }
            }
        }
        let mut sorted_tx: Vec<(Translation, usize)> =
            txs.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        sorted_tx.sort_by(|a, b| b.1.cmp(&a.1));
        if sorted_tx.len() == 0 || (sorted_tx.len() > 1 && sorted_tx[0].1 == sorted_tx[1].1) {
            return None;
        }

        Some(sorted_tx[0].0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Translation {
    offset: Point,
    ori: [usize; 3],
    flip: u8,
}

impl Translation {
    fn new(to: Ray, from: Ray) -> Self {
        let (ori, flip) = to.delta.reckon(from.delta).unwrap();
        let flipx = if flip & 1 == 0 { 1 } else { -1 };
        let flipy = if flip & 2 == 0 { 1 } else { -1 };
        let flipz = if flip & 4 == 0 { 1 } else { -1 };
        let x = to.start.p[0] - flipx * from.start.p[ori[0]];
        let y = to.start.p[1] - flipy * from.start.p[ori[1]];
        let z = to.start.p[2] - flipz * from.start.p[ori[2]];

        Translation {
            offset: Point { p: [x, y, z] },
            ori,
            flip,
        }
    }

    fn identity() -> Self {
        Translation { offset: Point{p: [0, 0, 0]}, ori: [0,1,2], flip: 0 }
    }

    fn translate(&self, p: Point) -> Point {
        let flipx = if self.flip & 1 == 0 { 1 } else { -1 };
        let flipy = if self.flip & 2 == 0 { 1 } else { -1 };
        let flipz = if self.flip & 4 == 0 { 1 } else { -1 };
        let x = flipx * p.p[self.ori[0]] + self.offset.p[0];
        let y = flipy * p.p[self.ori[1]] + self.offset.p[1];
        let z = flipz * p.p[self.ori[2]] + self.offset.p[2];
        Point { p: [x, y, z] }
    }
}

impl Ray {
    fn new(a: Point, b: Point) -> Ray {
        let ads2 = a.origin_ds2();
        let bds2 = b.origin_ds2();
        let (start, end);

        if a == b {
            start = a;
            end = b;
        } else if ads2 == bds2 {
            let apos = a.p.iter().filter(|x| **x >= 0).count();
            let bpos = b.p.iter().filter(|x| **x >= 0).count();

            if apos == bpos {
                let aneg =
                    a.p.iter()
                        .enumerate()
                        .find(|(_, x)| **x < 0)
                        .map(|x| x.0)
                        .unwrap_or(3);

                let bneg =
                    b.p.iter()
                        .enumerate()
                        .find(|(_, x)| **x < 0)
                        .map(|x| x.0)
                        .unwrap_or(3);

                if aneg < bneg {
                    start = a;
                    end = b;
                } else {
                    start = b;
                    end = a;
                }
            } else if apos < bpos {
                start = a;
                end = b;
            } else {
                start = b;
                end = a
            }
        } else if ads2 < bds2 {
            start = a;
            end = b;
        } else {
            start = b;
            end = a;
        }

        Ray {
            start,
            _end: end,
            delta: Delta {
                ds: [
                    end.p[0] - start.p[0],
                    end.p[1] - start.p[1],
                    end.p[2] - start.p[2],
                ],
            },
        }
    }

    fn is_cardinal(&self) -> bool {
        self.delta.ds.iter().filter(|x| **x == 0).count() >= 2
    }
}

fn part1(input: &String) {
    let mut scanners: Vec<Scanner> = vec![];

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("---") {
            let mut scanner = Scanner::new();
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let d: Vec<i64> = line
                    .split(",")
                    .map(|d| i64::from_str_radix(d, 10).unwrap())
                    .collect();
                scanner.add_beacon(Point {
                    p: [d[0], d[1], d[2]],
                });
            }
            scanners.push(scanner);
        }
    }

 
    let mut txs = HashMap::from([(0,  vec![(0, Translation::identity())])]);
    let mut found = scanners[0].beacon_set.clone();
    let mut tracker = scanners[0].beacons.clone();
    let mut included = HashSet::from([0]);
    let mut to_pair = VecDeque::from([0]);

    while !to_pair.is_empty() {
        let i = to_pair.pop_front().unwrap();
        let s1 = &scanners[i];
        
        for j in 0..scanners.len() {
            if i == j {
                continue;
            }

            if included.contains(&j) {
                continue;
            }

            let s2 = &scanners[j];

            // let (common_count, _) = s1.match_points(s2, usize::MAX);
            // if common_count < 12 {
            //     continue; // not enough poins
            // }
            
            let tx = match s1.txfn(s2) {
                Some(x) => x,
                None => continue
            };
            let mut pipeline = txs.get(&i).unwrap().clone();
            pipeline.push((j, tx));
            for mut b in s2.beacons.iter().cloned() {
                for (_, step) in pipeline.iter().rev() {
                    b = step.translate(b);
                }
                found.insert(b);
                tracker.push(b);
            }
            
            txs.insert(j, pipeline);
            included.insert(j);
            to_pair.push_back(j);
        }
    }

    let mut pipes : Vec<usize> = txs.keys().cloned().collect();
    pipes.sort();
    for pipe in pipes {
        println!("{}",txs.get(&pipe).unwrap().iter().rev().map({|(seg,_)| seg.to_string()}).collect::<Vec<String>>().join(", "));
    }
    let final_unique: HashSet<Point> = HashSet::from_iter(tracker.iter().cloned());
 
    println!("{}", found.len());
    println!("{}", final_unique.len());
}
