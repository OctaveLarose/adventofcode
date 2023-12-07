use std::fs::File;
use std::usize;
use std::io::{BufRead, BufReader};

const DIGIT_STRS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn is_digit_or_digit_string(c: char, slice_to_check: &str) -> Option<usize> {
    match c {
        '0'..='9' => Some(c as usize - '0' as usize),
        'A'..='Z' | 'a'..='z' => {
            let does_digit_str_match = DIGIT_STRS.iter()
                .position(|digit_str| slice_to_check.starts_with(digit_str));

            match does_digit_str_match {
                Some(digit) => Some(digit + 1),
                None => None,
            }
        },
        _ => panic!("Non alphanumeric character in string.")
    }
}
fn number_strs_to_nbr(line: &String) -> usize {
    let first_digit = line.chars()
        .enumerate()
        .find_map(|(idx, c)| {
            let start_slice = &line[idx..line.len() - 1];
            is_digit_or_digit_string(c, start_slice)
        })
        .unwrap_or_else(|| panic!("First digit couldn't be found"));

    // Using rev(): we're starting from the end.
    let last_digit = line.chars().rev()
        .enumerate()
        .find_map(|(idx, c)| {
            let end_slice = &line[line.len() - idx - 1..line.len()];
            is_digit_or_digit_string(c, end_slice)
        })
        .unwrap_or_else(|| panic!("Last digit couldn't be found"));

    first_digit * 10 + last_digit
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
    let lines = BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>();

    println!("Day 1: ");
    println!("Part 1: {}", lines.iter().map(|s| digits_to_nbr(s)).sum::<usize>());
    println!("Part 2: {}", lines.iter().map(|s| number_strs_to_nbr(s)).sum::<usize>());
    println!("----------")
}