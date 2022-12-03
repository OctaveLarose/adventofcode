use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_item_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 96,
        'A'..='Z' => item as i32 - 38,
        _ => { panic!("Unreachable") }
    }
}

fn intersect_str(a: &str, b: &str) -> char {
    for c in a.chars() {
        if b.contains(c) {
            return c;
        }
    }
    panic!("Unreachable")
}

fn part1(rucksacks: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;

    for rucksack_res in rucksacks {
        let rucksack = rucksack_res.unwrap();
        let compartments = rucksack.split_at(rucksack.len() / 2);
        let lol = intersect_str(compartments.0, compartments.1);
        sum += get_item_priority(lol);
    }

    sum
}

pub fn run() {
    let file = File::open("inputs/day3").unwrap();
    let rucksacks = BufReader::new(file).lines();

    println!("Day 3: ");
    println!("Part 1: {}", part1(rucksacks));
    // println!("Part 2: {}", pairs.iter().map(strat_2).sum::<usize>());
    println!("----------")
}