use std::fs;

// PipeOrGround would be more accurate. But eh.
#[derive(Debug)]
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

impl Pipe {
    pub fn from_char(c: char) -> Pipe {
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

#[derive(Debug)]
struct PipeMap {
    map: Vec<Pipe>,
    width: usize,
    height: usize
}

impl PipeMap {
    pub fn parse(input_str: String) -> PipeMap {
        PipeMap {
            map: input_str.chars().filter_map(|c|
                match c {
                    '\n' => None,
                    any => Some(Pipe::from_char(any))
                }).collect(),
            width: input_str.lines().nth(0).unwrap().len(),
            height: input_str.lines().count()
        }
    }
}


pub fn run() {
    let pipe_map = PipeMap::parse(fs::read_to_string("../inputs/testday10").unwrap());

    dbg!(&pipe_map);
    // println!("Part 1: {}", &histories.iter().map(part1_rec).sum::<isize>());
    // println!("Part 2: {}", &histories.iter().map(part2_rec).sum::<isize>());
}
