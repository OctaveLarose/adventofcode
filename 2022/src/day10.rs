use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day10::InstrType::{ADDX, NOOP};

#[derive(Debug, Copy, Clone)]
enum InstrType {
    ADDX,
    NOOP
}

#[derive(Debug, Copy, Clone)]
struct Instr {
    instr_type: InstrType,
    value: i32, // Option would be cleaner (NOOP has no need for a value) but would use an object for nothing. Not sure the Rust compiler would optimize it away
    cycles_left: u32
}

impl Instr {
    pub fn from_str(str: String) -> Instr {
        match &str[0..4] {
            "noop" => Instr {instr_type: NOOP, value: 0, cycles_left: 1},
            "addx" => Instr {
                instr_type: ADDX,
                value: str.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap(),
                cycles_left: 2
            },
            _ => panic!("Invalid instruction string")
        }
    }
}

struct SignalHandler {
    register: isize,
    all_instrs: Vec<Instr>,
    pending_instrs: Vec<Instr>,
    current_cycle: usize,
    interesting_signal_strength_total: isize
}

impl SignalHandler {
    pub fn new(instructions: Vec<Instr>) -> SignalHandler {
        SignalHandler {
            register: 1,
            all_instrs: instructions,
            pending_instrs: vec![],
            current_cycle: 0,
            interesting_signal_strength_total: 0
        }
    }

    fn is_interesting_signal(&self) -> bool {
        (self.current_cycle + 20) % 40 == 0
    }

    pub fn execute(&mut self) -> isize {
        for current_instr in &self.all_instrs {
            if self.is_interesting_signal() {
                dbg!(self.current_cycle, self.register);
                self.interesting_signal_strength_total += (self.current_cycle as isize) * self.register;
            }

            match current_instr.instr_type {
                NOOP => {},
                ADDX => { self.pending_instrs.push(current_instr.clone()) }
            }

            for mut instr in self.pending_instrs.iter_mut() {
                instr.cycles_left -= 1;
                if instr.cycles_left == 0 {
                    self.register += instr.value as isize;
                }
            }
            self.pending_instrs.retain(|&instr| instr.cycles_left > 0);
            self.current_cycle += 1;
        }

        while !self.pending_instrs.is_empty() {
            if self.is_interesting_signal() {
                dbg!(self.current_cycle, self.register);
                self.interesting_signal_strength_total += (self.current_cycle as isize) * self.register;
            }

            for mut instr in self.pending_instrs.iter_mut() {
                instr.cycles_left -= 1;
                if instr.cycles_left == 0 {
                    self.register += instr.value as isize;
                }
            }
            self.pending_instrs.retain(|&instr| instr.cycles_left > 0);
            self.current_cycle += 1;
        }
        self.interesting_signal_strength_total
    }
}

pub fn run() {
    let file = File::open("inputs/testday10_2").unwrap();
    let instructions = BufReader::new(file).lines()
        .map(|res_str| Instr::from_str(res_str.unwrap()))
        .collect::<Vec<Instr>>();

    println!("Day 9: ");
    println!("Part 1: {}", SignalHandler::new(instructions).execute());
    println!("----------");
}