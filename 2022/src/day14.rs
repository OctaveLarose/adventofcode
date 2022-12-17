use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::fs;
use itertools::{Itertools};
use crate::day14::SandStatus::{AtRest, InAbyss};
use crate::day14::Tile::{Air, Rock, Sand};

const SAND_SPAWN_X: usize = 500;
const SAND_SPAWN_Y: usize = 0;

const MIN_X: usize = 450;
const MAX_X: usize = 520;
const MAX_Y: usize = 170;
// const MIN_X: usize = 492;
// const MAX_X: usize = 505;
// const MAX_Y: usize = 10;
const LENGTH: usize = MAX_X - MIN_X;

type MapTiles = [[Tile; LENGTH]; MAX_Y];

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand
}

enum SandStatus {
    Moved,
    AtRest,
    InAbyss
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

struct Map {
    map_tiles: MapTiles
}

impl Map {
    fn create_map_from_input(lines: Vec<&str>) -> Map {
        let mut map_tiles: MapTiles = [[Air; LENGTH]; MAX_Y];

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
                        map_tiles[i][start_x - MIN_X] = Rock;
                    }
                } else if start_y == end_y {
                    let min = min(start_x, end_x) - MIN_X;
                    let max = max(start_x, end_x) - MIN_X + 1;

                    for i in min..max {
                        map_tiles[start_y][i] = Rock;
                    }
                } else {
                    panic!("Invalid coordinates, not a straight line");
                }
            }
        }
        Map { map_tiles }
    }

    fn can_move_sand(&mut self, pos: (usize, usize)) -> bool {
        self.map_tiles[pos.1][pos.0 - MIN_X] == Air
    }

    fn make_sand_fall(&mut self) -> SandStatus {
        let mut sand_pos = (SAND_SPAWN_X, SAND_SPAWN_Y);

        loop {
            let pos_down = (sand_pos.0, sand_pos.1 + 1);
            let pos_down_l = (sand_pos.0 - 1, sand_pos.1 + 1);
            let pos_down_r = (sand_pos.0 + 1, sand_pos.1 + 1);

            if sand_pos.1 >= MAX_Y - 1 {
                return InAbyss;
            }

            if self.can_move_sand(pos_down) {
                sand_pos = pos_down;
                continue;
            } else if self.can_move_sand(pos_down_l) {
                sand_pos = pos_down_l;
                continue;
            } else if self.can_move_sand(pos_down_r) {
                sand_pos = pos_down_r;
                continue;
            } else {
                self.map_tiles[sand_pos.1][sand_pos.0 - MIN_X] = Sand;
                return AtRest
            }
        }
    }

    fn get_nbr_sand_resting(&mut self) -> usize {
        let mut nbr_sand = 0;

        loop {
            match self.make_sand_fall() {
                AtRest => {}
                InAbyss => {break;}
                SandStatus::Moved => {panic!("Sand is supposed to be done falling")}
            }
            nbr_sand += 1;
        }
        // print_map(&map);

        nbr_sand
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in self.map_tiles {
            for c in l {
                write!(f, "{}", c).expect("COULDN'T WRITE");
            }
            write!(f, "\n").expect("COULDN'T WRITE");
        }
        write!(f, "\n")
    }
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/day14").unwrap();
    let file_lines = file_str.lines().collect::<Vec<&str>>();

    let mut map = Map::create_map_from_input(file_lines);

    println!("Day 14: ");
    println!("Part 1: {}", map.get_nbr_sand_resting());
    // println!("Part 2: {}", part2(&mut all_packets));
    println!("----------");
}