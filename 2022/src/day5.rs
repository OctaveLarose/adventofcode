use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: Vec<&mut Vec<char>> = Vec::new();
    let nbr_stacks = lines.iter().next().unwrap().len() / 4;

    for _ in 0..nbr_stacks {
        stacks.push(&mut Vec::new());
    }

    for l in lines {
        let chars = l.chars().collect::<Vec<char>>();
        let chunks = chars.chunks(4);
        println!("---");
        for (idx, c) in chunks.enumerate() {
            println!("{:?}", c);
            if c[1] != ' ' {
                let mut stack: &mut &mut Vec<char> = stacks.get(idx).unwrap();
                stack.push(c[1]);
            }
        }
        if l.eq(" ") {
            break;
        }
        // println!("xx{}xx", l);
    }

    (stacks, vec![])
}


fn part1() -> () {

}


pub fn run() {
    let file = File::open("inputs/testday5").unwrap();
    // let lines = BufReader::new(file).lines();
    let lines = BufReader::new(file).lines().map(|x| format!("{} ", x.unwrap())).collect::<Vec<String>>();

    let (stacks, instructions) = parse(lines);
    // let pairs = lines.iter().map(pair_maker).collect::<Vec<((usize, usize), (usize, usize))>>();

    println!("Day 5: ");
    // println!("Part 1: {}", part1(&pairs));
    println!("----------")
}