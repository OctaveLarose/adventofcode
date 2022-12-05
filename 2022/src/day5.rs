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
        let c1 = instr_line.chars().nth(5).unwrap() as u32 - '0' as u32;
        let c2 = instr_line.chars().nth(12).unwrap() as u32 - '0' as u32;
        let c3 = instr_line.chars().nth(17).unwrap() as u32 - '0' as u32;
        instructions.push((c1, c2, c3));
    }

    (stacks, instructions)
}


fn part1() -> () {

}


pub fn run() {
    let file = File::open("inputs/testday5").unwrap();
    // let lines = BufReader::new(file).lines();
    let lines = BufReader::new(file).lines().map(|x| format!("{} ", x.unwrap())).collect::<Vec<String>>();
    let (stacks, instructions) = parse(lines);

    for s in stacks {
        println!("{:?}", s);
    }

    for i in instructions {
        println!("{:?}", i);
    }
    println!("Day 5: ");
    // println!("Part 1: {}", part1(&pairs));
    println!("----------")
}