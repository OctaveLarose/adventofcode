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

pub fn run() {
    let file = File::open("inputs/testday9").unwrap();
    let lines = BufReader::new(file).lines()
        .map(|res_str| Motion::from_str(res_str.unwrap()))
        .collect::<Vec<Motion>>();

    for l in lines{
        dbg!(l);
    }

    println!("Day 9: ");
    // println!("Part 1: {}", tree_map.get_number_of_visible_trees());
    // println!("Part 2: {}", tree_map.get_highest_scenic_score());
    println!("----------");
}