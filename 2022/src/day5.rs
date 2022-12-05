use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

struct CargoManager {
    stacks: Vec<Vec<char>>,
    rules: Vec<(u32, u32, u32)>
}

impl CargoManager {
    pub fn init(&mut self, nbr_stacks: usize) {
        self.stacks = iter::repeat_with(|| Vec::<char>::new())
            .take(nbr_stacks)
            .collect();
    }
}

fn parse(lines: Vec<String>, cargo_manager: &mut CargoManager) -> () {
    for i in 0..cargo_manager.stacks.len() {
        for l in &lines {
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

}


fn part1(cargo_manager: &mut CargoManager) -> () {
    // for s in &cargo_manager.stacks {
    //     println!("{:?}", s);
    // }
    //
    // for s in &cargo_manager.rules {
    //     println!("{:?}", s);
    // }

    for instr in &cargo_manager.rules {
        for _ in 0..instr.0 {
            let val = cargo_manager.stacks.get_mut(instr.1 as usize - 1).unwrap().pop().unwrap();
            cargo_manager.stacks.get_mut(instr.2 as usize - 1).unwrap().push(val);
        }
    }
    // for s in &cargo_manager.stacks {
    //     println!("ss{:?}", s);
    // }

    print!("Part 1: ");
    for s in &cargo_manager.stacks {
        print!("{}", s.last().unwrap());
    }
    println!()
}


pub fn run() {
    let file = File::open("inputs/day5").unwrap();
    let lines = BufReader::new(file).lines().map(|x| format!("{} ", x.unwrap())).collect::<Vec<String>>();
    let mut cargo_manager = CargoManager {stacks: vec![], rules: vec![]};
    cargo_manager.init(lines.iter().next().unwrap().len() / 4);

    parse(lines, &mut cargo_manager);

    // for s in stacks {
    //     println!("{:?}", s);
    // }
    //
    // for i in instructions {
    //     println!("{:?}", i);
    // }

    println!("Day 5: ");
    part1(&mut cargo_manager);
    println!("----------")
}