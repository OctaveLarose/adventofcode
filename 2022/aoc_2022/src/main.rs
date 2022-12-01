use std::fs::File;
use std::{io, usize};
use std::io::BufRead;
use std::str::FromStr;

fn part1() -> Result<usize, io::Error> {
    let file = File::open("inputs/day1")?;
    let lines = io::BufReader::new(file).lines();
    let mut elves: Vec<usize> = Vec::new();
    let mut current_elf_sum = 0;

    for line_res in lines {
        if let Ok(line) = line_res {
            if line.is_empty() {
                elves.push(current_elf_sum);
                current_elf_sum = 0;
            }
            if !line.is_empty() {
                current_elf_sum += usize::from_str(&*line.trim()).unwrap();
            }
        }
    }
    Ok(*elves.iter().max().unwrap())
}

fn main() {
    println!("Part 1: {}", part1().unwrap());
}
