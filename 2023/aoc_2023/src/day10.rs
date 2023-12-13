use std::fs;
use pathfinding::prelude::astar;
use crate::day10::Pipe::Start;
use crate::map::{Map2D, MapElement, Pos};
use crate::map::Direction;

// PipeOrGround would be more accurate. But eh.
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

// type PipeMap = Map2D<Pipe>;
struct PipeMap {
    map: Map2D<Pipe>,
    start_pos: Pos
}

impl PipeMap {
    pub fn parse(string: String) -> PipeMap {
        let map = Map2D::<Pipe>::parse(string.as_str());
        let start_pos = map.tiles.iter().position(|p| *p == Start).unwrap();
        PipeMap {
            map,
            start_pos
        }
    }
}

impl PipeMap {
    fn get_valid_destinations(&self, tile_idx: usize) -> Vec<(Pos, usize)> {
        let mut valid_destinations = vec![];

        match self.map.get(tile_idx) {
            Pipe::NS => {
                valid_destinations.extend([
                    self.map.get_pos_in_dir(tile_idx, Direction::N),
                    self.map.get_pos_in_dir(tile_idx, Direction::S)]
                );
            }
            Pipe::EW => {
                valid_destinations.extend([
                    self.map.get_pos_in_dir(tile_idx, Direction::E),
                    self.map.get_pos_in_dir(tile_idx, Direction::W)]
                );
            }
            Pipe::NE => {
                valid_destinations.extend([
                    self.map.get_pos_in_dir(tile_idx, Direction::N),
                    self.map.get_pos_in_dir(tile_idx, Direction::E)]
                );
            }
            Pipe::NW => {
                valid_destinations.extend([
                    self.map.get_pos_in_dir(tile_idx, Direction::N),
                    self.map.get_pos_in_dir(tile_idx, Direction::W)]
                );
            }
            Pipe::SW => {
                valid_destinations.extend([
                    self.map.get_pos_in_dir(tile_idx, Direction::S),
                    self.map.get_pos_in_dir(tile_idx, Direction::W)]
                );
            }
            Pipe::SE => {
                valid_destinations.extend([
                    self.map.get_pos_in_dir(tile_idx, Direction::S),
                    self.map.get_pos_in_dir(tile_idx, Direction::E)]
                );
            }
            Start => {
                // the tile to the north is only valid if it has a connection to the south.
                if let Some(Pipe::NS) | Some(Pipe::SW) | Some(Pipe::SE) = self.map.get_in_dir(tile_idx, Direction::N)  {
                    valid_destinations.push(self.map.get_pos_in_dir(tile_idx, Direction::N));
                }

                // to the east, only valid if it connects to the west
                if let Some(Pipe::EW) | Some(Pipe::SW) | Some(Pipe::NW) = self.map.get_in_dir(tile_idx, Direction::E) {
                    valid_destinations.push(self.map.get_pos_in_dir(tile_idx, Direction::E));
                }

                if let Some(Pipe::EW) | Some(Pipe::SE) | Some(Pipe::NE) = self.map.get_in_dir(tile_idx, Direction::W) {
                    valid_destinations.push(self.map.get_pos_in_dir(tile_idx, Direction::W));
                }

                if let Some(Pipe::NS) | Some(Pipe::NW) | Some(Pipe::NE) = self.map.get_in_dir(tile_idx, Direction::S) {
                    valid_destinations.push(self.map.get_pos_in_dir(tile_idx, Direction::S))
                }
            },
            Pipe::Ground => {}
        }

        valid_destinations.iter()
            .filter_map(|&des_opt| des_opt.and_then(|dest| Some((dest, 1))))
            .collect::<Vec<(usize, Pos)>>()
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
        &pipe_map.start_pos,
        |&p| pipe_map.get_valid_destinations(p),
        |&p| pipe_map.get_heuristic(p),
        |&p| p == pipe_map.start_pos)
        .unwrap();

    dbg!(&a_star_result);
    42
}

pub fn run() {
    let pipe_map = PipeMap::parse(fs::read_to_string("../inputs/testday10").unwrap());

    // dbg!(&pipe_map);

    println!("Part 1: {}", part1(&pipe_map));
    // println!("Part 2: {}", &histories.iter().map(part2_rec).sum::<isize>());
}
