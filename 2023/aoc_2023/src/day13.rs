use std::fmt::{Debug, Formatter, Write};
use std::fs;
use crate::day13::Tile::*;
use crate::map::{Map2D, MapElement};

enum Tile {
    ASH,
    ROCK
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASH => f.write_char('.'),
            ROCK => f.write_char('#')
        }
    }
}

impl MapElement for Tile {
    fn parse_from_char(c: char) -> Self {
        match c {
            '.' => ASH,
            '#' => ROCK,
            _ => panic!("Invalid pattern")
        }
    }

    fn to_char(&self) -> char {
        match self {
            ASH => '.',
            ROCK => '#'
        }
    }
}

pub fn run() {
    let file_str = fs::read_to_string("../inputs/testday13").unwrap();
    let all_maps: Vec<Map2D<Tile>> = file_str.split("\n\n").map(Map2D::parse).collect();
    // let all_pairs: Vec<SpringsAndCondition> = file_str.lines()
    //     .map(SpringsAndCondition::from_str)
    //     .collect();

    dbg!(&all_maps);
    // println!("Part 1: {}", all_pairs.iter().map(SpringsAndCondition::get_nbr_arrangements).sum::<usize>());
    // println!("Part 2: {}", all_pairs.iter().map(SpringsAndCondition::unfold).map(|v| v.get_nbr_arrangements()).sum::<usize>());
}
