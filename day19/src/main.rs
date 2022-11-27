#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    io::{stdin, Read},
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

fn input_txt() -> String {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).ok();
    buf
}

fn main() {
    let input = input_txt();
    part1(&input);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    p: [i64; 3],
}

impl Point {
    fn origin_ds2(&self) -> i64 {
        self.p[0] * self.p[0] + self.p[1] * self.p[1] + self.p[2] * self.p[2]
    }
}

#[derive(Debug, Clone, Copy)]
struct Delta {
    ds: [i64; 3],
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    start: Point,
    end: Point,
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

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Point>,
    rays: HashMap<(usize, usize), Ray>,
}

#[derive(Debug)]
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
            end,
            delta: Delta {
                ds: [
                    end.p[0] - start.p[0],
                    end.p[1] - start.p[1],
                    end.p[2] - start.p[2],
                ],
            },
        }
    }
}

fn part1(input: &String) {
    let mut scanners: Vec<Scanner> = vec![];

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("---") {
            let mut beacons: Vec<Point> = vec![];
            let mut rays: HashMap<(usize, usize), Ray> = HashMap::new();

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let d: Vec<i64> = line
                    .split(",")
                    .map(|d| i64::from_str_radix(d, 10).unwrap())
                    .collect();
                beacons.push(Point {
                    p: [d[0], d[1], d[2]],
                });
            }

            for i in 0..beacons.len() {
                for j in 0..beacons.len() {
                    if i == j {
                        continue;
                    }
                    rays.insert((i, j), Ray::new(beacons[i], beacons[j]));
                }
            }
            scanners.push(Scanner { beacons, rays });
        }
    }
}
