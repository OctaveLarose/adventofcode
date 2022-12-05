use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let nbr_stacks = lines.iter().next().unwrap().len() / 4;
    let mut instructions: Vec<(u32, u32, u32)> = Vec::new();

    for i in 0..nbr_stacks {
        let mut cur_stack: Vec<char> = Vec::new();
        for l in &lines {
            if l.eq(" ") {
                continue;
            }

            let char = l.chars().nth(i * 4 + 1).unwrap();

            if char.is_numeric() {
                break;
            }

            if char != ' ' {
                cur_stack.insert(0, char);
            }
        }
        stacks.push(cur_stack);
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
        instructions.push((c1, c2, c3));
    }

    (stacks, instructions)
}


fn part1(stacks: &Vec<Vec<char>>, instructions: &Vec<(u32, u32, u32)>) -> () {
    for instr in instructions {
        for _ in 0..instr.1 {
            let val = stacks.get(instr.1 as usize).unwrap().pop().unwrap();
            stacks.get(instr.0 as usize).unwrap().push(val);
        }
        println!("{:?}", instr);
    }

    for s in stacks {
        print!("{}", s.last().unwrap());
    }
}


pub fn run() {
    let file = File::open("inputs/testday5").unwrap();
    // let lines = BufReader::new(file).lines();
    let lines = BufReader::new(file).lines().map(|x| format!("{} ", x.unwrap())).collect::<Vec<String>>();
    let (stacks, instructions) = parse(lines);

    // for s in stacks {
    //     println!("{:?}", s);
    // }
    //
    // for i in instructions {
    //     println!("{:?}", i);
    // }

    println!("Day 5: ");
    part1(&stacks, &instructions);
    println!("----------")
}