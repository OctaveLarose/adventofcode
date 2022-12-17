use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::Lines;
use itertools::{Itertools};
use crate::day14::Tile::{Air, Rock};

// const MIN_X: usize = 450;
// const MAX_X: usize = 520;
// const MAX_Y: usize = 170;
const MIN_X: usize = 492;
const MAX_X: usize = 505;
const MAX_Y: usize = 10;
const LENGTH: usize = MAX_X - MIN_X;

type MapTiles = [[Tile; LENGTH]; MAX_Y];

#[derive(Copy, Clone)]
enum Tile {
    Air,
    Rock,
    Sand
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Air => write!(f, "."),
            Rock => write!(f, "#"),
            Sand => write!(f, "o"),
        }
    }
}

fn print_map(map: &MapTiles) {
    for l in map {
        for c in l {
            print!("{}", c)
        }
        println!()
    }
}

fn create_map_from_input(lines: Vec<&str>) -> MapTiles {
    let mut map: MapTiles = [[Air; LENGTH]; MAX_Y];

    for l in lines {
        for (start, end) in l.split(" -> ").tuple_windows::<(&str, &str)>() {
            let (start_x, start_y) = start.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>().unwrap();
            let (end_x, end_y) = end.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>().unwrap();

            if start_x == end_x {
                let min = min(start_y, end_y);
                let max = max(start_y, end_y) + 1;
                for i in min..max {
                    map[i][start_x - MIN_X] = Rock;
                }
            } else if start_y == end_y {
                let min = min(start_x, end_x) - MIN_X;
                let max = max(start_x, end_x) - MIN_X + 1;

                for i in min..max {
                    map[start_y][i] = Rock;
                }
            } else {
                panic!("Invalid coordinates, not a straight line");
            }
        }
    }
    map
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/testday14").unwrap();
    let file_lines = file_str.lines().collect::<Vec<&str>>();

    let map = create_map_from_input(file_lines);

    print_map(&map);

    println!("Day 14: ");
    // println!("Part 1: {}", part1(&pairs));
    // println!("Part 2: {}", part2(&mut all_packets));
    println!("----------");
}