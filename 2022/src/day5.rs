use std::fs::File;
use std::io::{BufRead, BufReader};

struct CargoManager {
    stacks: Vec<Vec<char>>,
    rules: Vec<(u32, u32, u32)>
}

impl CargoManager {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut cargo_manager = CargoManager {stacks: vec![], rules: vec![]};
        let nbr_stacks = lines.iter().next().unwrap().len() / 4;

        for i in 0..nbr_stacks {
            cargo_manager.stacks.push(Vec::<char>::new());
            for l in lines {
                if l.eq(" ") {
                    continue;
                }

                let char = l.chars().nth(i * 4 + 1).unwrap();

                if char.is_numeric() {
                    break;
                }

                if char != ' ' {
                    cargo_manager.stacks.get_mut(i).unwrap().insert(0, char);
                }
            }
        }

        let mut lines_iter = lines.iter();
        loop {
            let lol = lines_iter.next().unwrap();
            if lol.len() == 1 {
                break;
            }
        }

        for instr_line in lines_iter {
            let mut split = instr_line.split_whitespace();
            let c1 = split.nth(1).unwrap().parse::<u32>().unwrap();
            let c2 = split.nth(1).unwrap().parse::<u32>().unwrap();
            let c3 = split.nth(1).unwrap().parse::<u32>().unwrap();
            cargo_manager.rules.push((c1, c2, c3));
        }

        cargo_manager
    }
}


fn part1(cargo_manager: &mut CargoManager) -> () {
    for instr in &cargo_manager.rules {
        for _ in 0..instr.0 {
            let val = cargo_manager.stacks.get_mut(instr.1 as usize - 1).unwrap().pop().unwrap();
            cargo_manager.stacks.get_mut(instr.2 as usize - 1).unwrap().push(val);
        }
    }

    println!("Part 1: {}", &cargo_manager.stacks.iter().map(|s| s.last().unwrap()).collect::<String>());
}

fn part2(cargo_manager: &mut CargoManager) -> () {
    for instr in &cargo_manager.rules {
        let mut elems = Vec::new();
        for _ in 0..instr.0 {
            let val = cargo_manager.stacks.get_mut(instr.1 as usize - 1).unwrap().pop().unwrap();
            elems.push(val);
        }
        elems.reverse();
        cargo_manager.stacks.get_mut(instr.2 as usize - 1).unwrap().append(&mut elems);
    }

    println!("Part 2: {}", &cargo_manager.stacks.iter().map(|s| s.last().unwrap()).collect::<String>());
}

pub fn run() {
    let file = File::open("inputs/day5").unwrap();
    let lines = BufReader::new(file).lines().map(|x| format!("{} ", x.unwrap())).collect::<Vec<String>>();

    println!("Day 5: ");
    part1(&mut CargoManager::new(&lines));
    part2(&mut CargoManager::new(&lines));
    println!("----------")
}