use std::fs;
use pathfinding::prelude::astar;
use crate::map::{Map2D, MapElement, Pos};
use crate::map::Direction;

// PipeOrGround would be more accurate. But eh
#[derive(Debug, PartialEq)]
#[repr(u8)] // I don't use the chars defined in the enum, though. There's probably a way.
enum Pipe {
    NS = b'|',
    EW = b'-',
    NE = b'L',
    NW = b'J',
    SW = b'7',
    SE = b'F',
    Ground = b'.',
    Start = b'S'
}

impl MapElement for Pipe {
    fn parse_from_char(c: char) -> Pipe {
        match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid character for pipe: {}", c)
        }
    }
}

struct PipeMap {
    map: Map2D<Pipe>,
    start_pos: Pos
}

impl PipeMap {
    pub fn parse(string: String) -> PipeMap {
        let map = Map2D::<Pipe>::parse(string.as_str());
        let start_pos = map.tiles.iter().position(|p| *p == Pipe::Start).unwrap();
        PipeMap {
            map,
            start_pos
        }
    }
}

impl PipeMap {
    fn get_valid_destinations(&self, cur_pos: Pos, prev_pos: Option<Pos>) -> Vec<((Pos, Option<Pos>), usize)> {
        let mut valid_destinations = vec![];

        match self.map.get(cur_pos) {
            Pipe::NS => {[Direction::N, Direction::S].map(|d| valid_destinations.push(self.map.get_pos_in_dir(cur_pos, d)));},
            Pipe::EW => {[Direction::E, Direction::W].map(|d| valid_destinations.push(self.map.get_pos_in_dir(cur_pos, d)));},
            Pipe::NE => {[Direction::N, Direction::E].map(|d| valid_destinations.push(self.map.get_pos_in_dir(cur_pos, d)));},
            Pipe::NW => {[Direction::N, Direction::W].map(|d| valid_destinations.push(self.map.get_pos_in_dir(cur_pos, d)));},
            Pipe::SW => {[Direction::S, Direction::W].map(|d| valid_destinations.push(self.map.get_pos_in_dir(cur_pos, d)));},
            Pipe::SE => {[Direction::S, Direction::E].map(|d| valid_destinations.push(self.map.get_pos_in_dir(cur_pos, d)));},
            Pipe::Start => {
                // the tile to the north is only valid if it has a connection to the south.
                if let Some(Pipe::NS) | Some(Pipe::SW) | Some(Pipe::SE) = self.map.get_in_dir(cur_pos, Direction::N)  {
                    valid_destinations.push(self.map.get_pos_in_dir(cur_pos, Direction::N));
                }

                // to the east, only valid if it connects to the west
                if let Some(Pipe::EW) | Some(Pipe::SW) | Some(Pipe::NW) = self.map.get_in_dir(cur_pos, Direction::E) {
                    valid_destinations.push(self.map.get_pos_in_dir(cur_pos, Direction::E));
                }

                if let Some(Pipe::EW) | Some(Pipe::SE) | Some(Pipe::NE) = self.map.get_in_dir(cur_pos, Direction::W) {
                    valid_destinations.push(self.map.get_pos_in_dir(cur_pos, Direction::W));
                }

                if let Some(Pipe::NS) | Some(Pipe::NW) | Some(Pipe::NE) = self.map.get_in_dir(cur_pos, Direction::S) {
                    valid_destinations.push(self.map.get_pos_in_dir(cur_pos, Direction::S))
                }
            },
            Pipe::Ground => {}
        }

        valid_destinations.iter()
            .filter_map(|&des_opt| {
                match des_opt != prev_pos { // no backtracking!
                    true => des_opt.and_then(|dest| Some(((dest, Some(cur_pos)), 1))), // weeding out "None" destinations + for AStar, each tile is worth 1.
                    false => None
                }
            })
            .collect()
    }

    fn get_heuristic(&self, pos: usize) -> usize {
        let pos_x = pos % self.map.width;
        let pos_y = pos / self.map.width;
        let goal_x = self.start_pos % self.map.width;
        let goal_y = self.start_pos / self.map.width;

        usize::abs_diff(goal_x, pos_x) + usize::abs_diff(goal_y, pos_y)
    }
}

fn part1(pipe_map: &PipeMap) -> usize {
    let a_star_result = astar(
        &(pipe_map.start_pos, None),
        |(pos, prev_pos)| pipe_map.get_valid_destinations(*pos, *prev_pos),
        |(pos, prev_pos)| pipe_map.get_heuristic(*pos),
        |(pos, prev_pos)| prev_pos.is_some() && *pos == pipe_map.start_pos)
        .unwrap();

    a_star_result.1 / 2
}

pub fn run() {
    let pipe_map = PipeMap::parse(fs::read_to_string("../inputs/day10").unwrap());

    // dbg!(&pipe_map);

    println!("Part 1: {}", part1(&pipe_map));
    // println!("Part 2: {}", &histories.iter().map(part2_rec).sum::<isize>());
}
