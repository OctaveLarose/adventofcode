use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_message(buffer: &String, len: usize) -> usize {
    for i in len..buffer.len() + 1 {
        let slice = &buffer[i - len..i];
        if HashSet::<char>::from_iter(slice.chars()).len() == len {
            return i;
        }
    }
    panic!("Nothing found");
}

pub fn run() {
    let file = File::open("inputs/day6").unwrap();
    let buffer = BufReader::new(file).lines().next().unwrap().unwrap();

    println!("Day 5: ");
    println!("Part 1: {}", find_message(&buffer, 4));
    println!("Part 2: {}", find_message(&buffer, 14));
    println!("----------");
}