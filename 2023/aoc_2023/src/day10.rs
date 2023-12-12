use std::fs;
use pathfinding::prelude::astar;
use crate::day10::Pipe::Start;
use crate::map::{Map2D, MapElement};
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
    start_pos: usize
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
    fn get_valid_destinations(&self, tile_idx: usize) -> Vec<(usize, &Pipe)> {
        let mut valid_destinations = vec![];

        match self.map.get(tile_idx) {
            Pipe::NS => {
                valid_destinations.push((self.map.get_pos_in_dir(tile_idx, Direction::N).unwrap(), self.map.get(tile_idx)));
                valid_destinations.push((self.map.get_pos_in_dir(tile_idx, Direction::S).unwrap(), self.map.get(tile_idx)));
            }
            Pipe::EW => {}
            Pipe::NE => {}
            Pipe::NW => {}
            Pipe::SW => {}
            Pipe::SE => {}
            Pipe::Ground => {}
            Start => {}
        }

        valid_destinations
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
