use std::fs::File;
use std::usize;
use std::io::{BufRead, BufReader};

fn number_strs_to_nbr(line: &String) -> usize {
    let digit_strs = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut first_digit: Option<usize> = None;
    let mut last_digit: Option<usize> = None;

    for (idx, c) in line.chars().enumerate() {
        if first_digit.is_none() {
            first_digit = match c {
                '0'..='9' => Some(c as usize - '0' as usize),
                'A'..='Z' | 'a'..='z' => {
                    let lol = digit_strs.iter()
                        .enumerate()
                        .filter_map(|(digit_idx, digit_str)|
                            if line[idx..line.len() - 1].starts_with(digit_str) { Some(digit_idx + 1) } else { None }
                        ).collect::<Vec<usize>>();
                    if lol.is_empty() { None } else { Some(*lol.first().unwrap())}
                },
                _ => panic!("Non alphanumeric character in string.")
            }
        }
    }

    for (idx, c) in line.chars().rev().enumerate() {
        if last_digit.is_none() {
            last_digit = match c {
                '0'..='9' => Some(c as usize - '0' as usize),
                'A'..='Z' | 'a'..='z' => {
                    let lol = digit_strs.iter()
                        .enumerate()
                        .filter_map(|(digit_idx, digit_str)|
                            if line[line.len() - idx - 1..line.len()].starts_with(digit_str) { Some(digit_idx + 1) } else { None }
                        ).collect::<Vec<usize>>();
                    if lol.is_empty() {
                        None
                    } else {
                        Some(*lol.first().unwrap())}
                },
                _ => panic!("Non alphanumeric character in string.")
            }
        }
    }

    first_digit.unwrap() * 10 + last_digit.unwrap()
}

fn digits_to_nbr(line: &String) -> usize {
    let char_to_digit = |opt_c: Option<char>|
        match opt_c {
            Some(c) => c as u32 - '0' as u32,
            None => panic!("The character found doesn't translate to a digit")
        };

    let digit1: u32 = char_to_digit(line.chars().find(|c| c.is_ascii_digit()));
    let digit2: u32 = char_to_digit(line.chars().rfind(|c| c.is_ascii_digit()));
    (digit1 * 10 + digit2) as usize
}


pub fn main() {
    let file = File::open("../inputs/day1").unwrap();
    let lines = BufReader::new(file).lines().map(|s| s.unwrap()).collect::<Vec<String>>();

    println!("Day 1: ");
    println!("Part 1: {}", lines.iter().map(|s| digits_to_nbr(s)).sum::<usize>());
    println!("Part 2: {}", lines.iter().map(|s| number_strs_to_nbr(s)).sum::<usize>());
    println!("----------")
}