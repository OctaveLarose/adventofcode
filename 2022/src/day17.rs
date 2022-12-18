use std::{fs};
use crate::day17::Push::{LEFT, RIGHT};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {x, y}
    }
}

#[derive(Debug)]
struct Rock {
    pattern: Vec<Pos>
}

impl Rock {
    fn pattern1() -> Rock {
        Rock { pattern: vec![Pos::new(0, 0), Pos::new(1, 0), Pos::new(2, 0), Pos::new(3, 0)] }
    }

    fn pattern2() -> Rock {
        Rock { pattern: vec![Pos::new(1, 0), Pos::new(0, 1), Pos::new(1, 1), Pos::new(2, 1), Pos::new(1, 2)] }
    }

    fn pattern3() -> Rock {
        Rock { pattern: vec![Pos::new(2, 0), Pos::new(2, 1), Pos::new(0, 2), Pos::new(1, 2), Pos::new(2, 2)] }
    }

    fn pattern4() -> Rock {
        Rock { pattern: vec![Pos::new(0, 0), Pos::new(0, 1), Pos::new(0, 2), Pos::new(0, 3)] }
    }

    fn pattern5() -> Rock {
        Rock { pattern: vec![Pos::new(0, 0), Pos::new(0, 1), Pos::new(1, 0), Pos::new(1, 1)] }
    }

    fn next_rock(rock_idx: usize) -> Rock {
        match rock_idx % 5 {
            0 => Rock::pattern1(),
            1 => Rock::pattern2(),
            2 => Rock::pattern3(),
            3 => Rock::pattern4(),
            4 => Rock::pattern5(),
            _ => { panic!("Unreachable") }
        }
    }
}

#[derive(Debug)]
enum Push {
    LEFT,
    RIGHT
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/testday17").unwrap();
    let pushes = file_str.chars().map(|c|
        match c {
            '<' => LEFT,
            '>' => RIGHT,
            invalid => panic!("Unexpected input: {}", invalid)
        }).collect::<Vec<Push>>();


    dbg!(&pushes);

    println!("Day 17: ");
    // println!("Part 1: {}", part1(&pushes));
    // println!("Part 2: {}", Map::create_map_from_input(&file_lines).get_nbr_sand_resting_with_ground());
    println!("----------");
}