use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day10::InstrType::{ADDX, NOOP};

#[derive(Debug)]
enum InstrType {
    ADDX,
    NOOP
}

#[derive(Debug)]
struct Instr {
    instr_type: InstrType,
    value: Option<u32>
}

impl Instr {
    pub fn from_str(str: String) -> Instr {
        match &str[0..4] {
            "noop" => Instr {instr_type: NOOP, value: None},
            "addx" => Instr {instr_type: ADDX, value: str.split_whitespace().nth(1).unwrap().parse::<u32>().ok()},
            _ => panic!("Invalid instruction string")
        }
    }
}

pub fn run() {
    let file = File::open("inputs/testday10").unwrap();
    let instructions = BufReader::new(file).lines()
        .map(|res_str| Instr::from_str(res_str.unwrap()))
        .collect::<Vec<Instr>>();

    dbg!(&instructions);
    println!("Day 9: ");
    // println!("Part 1: {}", get_nbr_locations_tail_visited(Rope::new(2), &instructions));
    // println!("Part 2: {}", get_nbr_locations_tail_visited(Rope::new(10), &instructions));
    println!("----------");
}