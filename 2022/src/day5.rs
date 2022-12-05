use std::fs::File;
use std::io::{BufRead, BufReader};

struct CargoManager {
    stacks: Vec<Vec<char>>,
    rules: Vec<(u32, u32, u32)>
}

fn parse(lines: Vec<String>, cargo_manager: &mut CargoManager) -> () {
    let nbr_stacks = lines.iter().next().unwrap().len() / 4;

    for i in 0..nbr_stacks {
        cargo_manager.stacks.push(Vec::new());
        for l in &lines {
            if l.eq(" ") {
                continue;
            }

            let char = l.chars().nth(i * 4 + 1).unwrap();

            if char.is_numeric() {
                break;
            }

            if char != ' ' {
                cargo_manager.stacks.last().unwrap().insert(0, char);
            }
        }
    }

    let mut lines_iter = lines.iter();
    loop {
        let lol = lines_iter.next().unwrap();
        if lol.len() == 1 {
            lines_iter.next();
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
    for instr in &cargo_manager.rules {
        for _ in 0..instr.1 {
            let val = cargo_manager.stacks.get(instr.1 as usize).unwrap().pop().unwrap();
            cargo_manager.stacks.get(instr.0 as usize).unwrap().push(val);
        }
        println!("{:?}", instr);
    }

    for s in &cargo_manager.stacks {
        print!("{}", s.last().unwrap());
    }
}


pub fn run() {
    let file = File::open("inputs/testday5").unwrap();
    // let lines = BufReader::new(file).lines();
    let lines = BufReader::new(file).lines().map(|x| format!("{} ", x.unwrap())).collect::<Vec<String>>();
    let mut cargo_manager = CargoManager {stacks: vec![], rules: vec![]};
    parse(lines, &mut cargo_manager);

    // for s in stacks {
    //     println!("{:?}", s);
    // }
    //
    // for i in instructions {
    //     println!("{:?}", i);
    // }

    println!("Day 5: ");
    // part1(&stacks, &instructions);
    println!("----------")
}