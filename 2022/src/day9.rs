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
    body: Vec<Pos>,
    length: usize
}

impl Rope {
    pub fn new(length: usize) -> Rope {
        let start_pos = Pos {x: 0, y: 0}; // arbitrary
        Rope {
            head: start_pos.clone(),
            body: vec![start_pos.clone(); length - 1],
            length
        }
    }

    pub fn update_knot(&mut self, knot_idx: usize) {
        let mut body_iter = self.body.iter_mut();
        let prev_knot = match knot_idx {
            0 => {&self.head},
            _ => {body_iter.nth(knot_idx - 1).unwrap()}
        };
        let knot = body_iter.next().unwrap();


        if knot.x == prev_knot.x && knot.y == prev_knot.y - 2 {
            knot.y = prev_knot.y - 1
        }
        if knot.x == prev_knot.x && knot.y == prev_knot.y + 2 {
            knot.y = prev_knot.y + 1
        }
        if knot.y == prev_knot.y && knot.x == prev_knot.x - 2 {
            knot.x = prev_knot.x - 1
        }
        if knot.y == prev_knot.y && knot.x == prev_knot.x + 2 {
            knot.x = prev_knot.x + 1
        }

        // top right
        if knot.x + 1 == prev_knot.x && knot.y + 2 == prev_knot.y {
            knot.x += 1;
            knot.y += 1;
        }

        if knot.x + 2 == prev_knot.x && knot.y + 1 == prev_knot.y {
            knot.x += 1;
            knot.y += 1;
        }

        if knot.x + 2 == prev_knot.x && knot.y + 2 == prev_knot.y {
            knot.x += 1;
            knot.y += 1;
        }

        // bottom left
        if knot.x - 1 == prev_knot.x && knot.y - 2 == prev_knot.y {
            knot.x -= 1;
            knot.y -= 1;
        }

        if knot.x - 2 == prev_knot.x && knot.y - 1 == prev_knot.y {
            knot.x -= 1;
            knot.y -= 1;
        }

        if knot.x - 2 == prev_knot.x && knot.y - 2 == prev_knot.y {
            knot.x -= 1;
            knot.y -= 1;
        }

        // top left
        if knot.x - 2 == prev_knot.x && knot.y + 1 == prev_knot.y {
            knot.x -= 1;
            knot.y += 1;
        }

        if knot.x - 1 == prev_knot.x && knot.y + 2 == prev_knot.y {
            knot.x -= 1;
            knot.y += 1;
        }

        if knot.x - 2 == prev_knot.x && knot.y + 2 == prev_knot.y {
            knot.x -= 1;
            knot.y += 1;
        }

        // bottom right
        if knot.x + 2 == prev_knot.x && knot.y - 1 == prev_knot.y {
            knot.x += 1;
            knot.y -= 1;
        }

        if knot.x + 1 == prev_knot.x && knot.y - 2 == prev_knot.y {
            knot.x += 1;
            knot.y -= 1;
        }

        if knot.x + 2 == prev_knot.x && knot.y - 2 == prev_knot.y {
            knot.x += 1;
            knot.y -= 1;
        }
    }

    pub fn move_rope(&mut self, mot: &Motion) {
        match &mot.dir {
            R => { self.head.x += 1 },
            L => { self.head.x -= 1 },
            D => { self.head.y -= 1 },
            U => { self.head.y += 1 }
        }

        for i in 0..(self.length - 1) {
            self.update_knot(i);
        }
    }
}

fn get_nbr_locations_tail_visited(mut rope: Rope, motions: &Vec<Motion>) -> usize {
    let mut locations_tail_visited: HashSet<Pos> = HashSet::new();

    for mot in motions {
        for _ in 0..mot.steps_nbr {
            rope.move_rope(mot);
            locations_tail_visited.insert(rope.body.last().unwrap().clone());
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
    println!("Part 1: {}", get_nbr_locations_tail_visited(Rope::new(2), &motions));
    println!("Part 2: {}", get_nbr_locations_tail_visited(Rope::new(10), &motions));
    println!("----------");
}