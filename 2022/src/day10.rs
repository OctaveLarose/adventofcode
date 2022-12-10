use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use crate::day10::InstrType::{ADDX, NOOP};

#[derive(Debug, Copy, Clone)]
enum InstrType {
    ADDX,
    NOOP
}

#[derive(Debug, Copy, Clone)]
struct Instr {
    instr_type: InstrType,
    value: isize, // Option would be cleaner (NOOP has no need for a value) but would use an object for nothing. Not sure the Rust compiler would optimize it away
    cycles_left: u32
}

impl Instr {
    pub fn from_str(str: String) -> Instr {
        match &str[0..4] {
            "noop" => Instr {instr_type: NOOP, value: 0, cycles_left: 1},
            "addx" => Instr {
                instr_type: ADDX,
                value: str.split_whitespace().nth(1).unwrap().parse::<isize>().unwrap(),
                cycles_left: 2
            },
            _ => panic!("Invalid instruction string")
        }
    }
}

struct CRT {
    register: isize,
    all_instrs: Vec<Instr>,
    current_cycle: usize,
    interesting_signal_strength_total: isize,
    should_draw: bool
}

impl CRT {
    pub fn new(instructions: Vec<Instr>) -> CRT {
        CRT {
            register: 1,
            all_instrs: instructions,
            current_cycle: 0,
            interesting_signal_strength_total: 0,
            should_draw: false
        }
    }

    pub fn should_draw(&mut self) -> &mut CRT {
        self.should_draw = true;
        self
    }

    fn is_interesting_signal(current_cycle: usize) -> bool {
        (current_cycle + 20) % 40 == 0
    }

    fn draw(current_cycle: usize, register: isize) {
        let pixel_idx = (current_cycle % 40) as isize - 1;

        if (pixel_idx - 1) <= register && register <= (pixel_idx + 1) {
            print!("#")
        } else {
            print!(".");
        }

        if current_cycle % 40 == 0 {
            print!("\n");
        }
    }

    pub fn execute(&mut self) -> isize {
        let mut instrs_iter = self.all_instrs.iter_mut().peekable();

        while instrs_iter.peek().is_some() {
            self.current_cycle += 1;

            if self.should_draw {
                CRT::draw(self.current_cycle, self.register);
            }

            if CRT::is_interesting_signal(self.current_cycle) {
                self.interesting_signal_strength_total += (self.current_cycle as isize) * self.register;
            }

            let mut current_instr = instrs_iter.peek_mut().unwrap();
            match current_instr.instr_type {
                NOOP => { instrs_iter.next(); },
                ADDX => {
                    current_instr.cycles_left -= 1;
                    if current_instr.cycles_left == 0 {
                        self.register += current_instr.value;
                        instrs_iter.next();
                    }
                }
            }
        }

        self.interesting_signal_strength_total
    }
}

pub fn run() {
    let file = File::open("inputs/day10").unwrap();
    let instructions = BufReader::new(file).lines()
        .map(|res_str| Instr::from_str(res_str.unwrap()))
        .collect::<Vec<Instr>>();

    println!("Day 10: ");
    println!("Part 1: {}", CRT::new(instructions.clone()).execute());
    println!("Part 2: ");
    CRT::new(instructions.clone()).should_draw().execute();
    println!("----------");
}