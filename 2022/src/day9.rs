use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day9::Direction::{U, R, D, L};

#[derive(Debug)]
enum Direction {
    U, R, D, L
}

#[derive(Debug)]
struct Motion {
    dir: Direction,
    steps_nbr: u8
}

impl Motion {
    pub fn from_str(str: String) -> Motion {
        let mut split = str.split_whitespace();

        Motion {
            dir: match split.nth(0).unwrap() {
                "U" => U,
                "R" => R,
                "D" => D,
                "L" => L,
                _ => { panic!("Invalid direction string"); }
            },
            steps_nbr: split.nth(0).unwrap().parse::<u8>().unwrap()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: isize,
    y: isize
}

#[derive(Debug)]
struct Rope {
    head: Pos,
    tail: Pos,
    length: usize // number of non head/tail elements
}

impl Rope {
    pub fn new(length: usize) -> Rope {
        let start_pos = Pos {x: 0, y: 0}; // arbitrary
        Rope { head: start_pos.clone(), tail: start_pos.clone(), length }
    }

    pub fn move_rope(&mut self, mot: &Motion) {
        match mot.dir {
            R => {
                self.head.x += 1;
                if self.tail.x < self.head.x - 1 {
                    self.tail.x = self.head.x - 1;
                    self.tail.y = self.head.y;
                }
            },
            L => {
                self.head.x -= 1;
                if self.tail.x > self.head.x + 1 {
                    self.tail.x = self.head.x + 1;
                    self.tail.y = self.head.y;
                }
            },
            D => {
                self.head.y -= 1;
                if self.tail.y > self.head.y + 1 {
                    self.tail.x = self.head.x;
                    self.tail.y = self.head.y + 1;
                }
            },
            U => {
                self.head.y += 1;
                if self.tail.y < self.head.y - 1 {
                    self.tail.x = self.head.x;
                    self.tail.y = self.head.y - 1;
                }
            },
        }
    }
}

fn part1(mut rope: Rope, motions: &Vec<Motion>) -> usize {
    let mut locations_tail_visited: HashSet<Pos> = HashSet::new();

    for mot in motions {
        // dbg!(&mot);
        for _ in 0..mot.steps_nbr {
            rope.move_rope(mot);
            // println!("head: {} {}", rope.head.x, rope.head.y);
            // println!("tail: {} {}", rope.tail.x, rope.tail.y);
            // println!("---");
            locations_tail_visited.insert(rope.tail.clone());
        }
    }

    locations_tail_visited.len()
}

pub fn run() {
    let file = File::open("inputs/day9").unwrap();
    let motions = BufReader::new(file).lines()
        .map(|res_str| Motion::from_str(res_str.unwrap()))
        .collect::<Vec<Motion>>();

    println!("Day 9: ");
    println!("Part 1: {}", part1(Rope::new(0), &motions));
    // println!("Part 2: {}", tree_map.get_highest_scenic_score());
    println!("----------");
}