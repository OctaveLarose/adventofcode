use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Chunks;

fn get_item_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 96,
        'A'..='Z' => item as i32 - 38,
        _ => { panic!("Unreachable") } // Is there a more idiomatic way to say "this string only contains alphabetical characters"? making my own type?
    }
}

fn part1(rucksacks: &Vec<String>) -> i32 {
    let mut sum = 0;

    for rucksack in rucksacks {
        let compartments = rucksack.split_at(rucksack.len() / 2);
        let mut filter = compartments.0.chars().filter(|c| compartments.1.contains(*c));
        sum += get_item_priority(filter.next().unwrap());
    }

    sum
}

fn part2(groups: Chunks<&String>) -> i32 {
    let mut sum = 0;

    for g in groups {
        let mut filter = g[0].chars().filter(|c| g.iter().all(|s| s.contains(*c)));
        let only_elem = filter.next().unwrap();
        sum += get_item_priority(only_elem);
    }

    sum
}

pub fn run() {
    let file = File::open("inputs/day3").unwrap();
    let lines = BufReader::new(file).lines();
    let rucksacks = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

    println!("Day 3: ");
    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(rucksacks.iter().collect::<Vec<&String>>().chunks(3)));
    println!("----------")
}