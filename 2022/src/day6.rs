use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn part1(buffer: &String) -> usize {
    for i in 4..buffer.len() + 1 {
        let slice = &buffer[i - 4..i];
        if HashSet::<char>::from_iter(slice.chars()).len() == 4 {
            return i;
        }
    }
    panic!("Nothing found");
}

pub fn run() {
    let file = File::open("inputs/day6").unwrap();
    let buffer = BufReader::new(file).lines().next().unwrap().unwrap();

    println!("Day 5: ");
    println!("Part 1: {}", part1(&buffer));
    // println!("Part 2: {}", part2(&buffer));
    println!("----------")
}