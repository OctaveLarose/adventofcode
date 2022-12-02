use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

enum MatchResult {
    WIN,
    LOSS,
    DRAW
}

fn get_pairs(lines: Lines<BufReader<File>>) -> Vec<(char, char)> {
    let mut pairs: Vec<(char, char)> = Vec::new();

    for line_res in lines {
        let line = line_res.unwrap();
        let split = line.split_whitespace().collect::<Vec<&str>>();
        // Horrendous parsing.
        pairs.push((split.get(0).unwrap().chars().next().unwrap(), split.get(1).unwrap().chars().next().unwrap()));
    }

    pairs
}

fn get_round_result(opponent: u8, ours: u8) -> MatchResult {
    if opponent as u8 == ours {
        MatchResult::DRAW
    } else if opponent == ours + 1 || (ours == 2 && opponent == 0) {
        MatchResult::LOSS
    } else {
        MatchResult::WIN
    }
}

fn strat_1(pair: &(char, char)) -> usize {
    let (char_opponent, char_ours) = pair;
    let opponent = *char_opponent as u8 - ('A' as u8);
    let ours = *char_ours as u8 % 4;
    let bonus = (*char_ours as u8 % 4 + 1) as usize;

    match get_round_result(opponent, ours) {
        MatchResult::WIN=> 6 + bonus,
        MatchResult::LOSS => 0 + bonus,
        MatchResult::DRAW => 3 + bonus
    }
}

pub fn run() {
    let file = File::open("inputs/day2").unwrap();
    let pairs  = get_pairs(BufReader::new(file).lines());

    println!("Day 2: ");
    println!("Part 1: {}", pairs.iter().map(strat_1).sum::<usize>());
    // println!("Part 2: {}", pairs.iter().map(strat_2).sum::<usize>());
    println!("----------")
}