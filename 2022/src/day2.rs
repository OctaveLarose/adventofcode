use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::day2::MatchResult::{DRAW, LOSS, WIN};

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
        DRAW
    } else if opponent == ours + 1 || (ours == 2 && opponent == 0) {
        LOSS
    } else {
        WIN
    }
}

fn get_bonus_from_choice(choice: u8) -> usize {
    (choice % 4 + 1) as usize
}

fn strat_1(pair: &(char, char)) -> usize {
    let (char_opponent, char_ours) = pair;
    let opponent = *char_opponent as u8 - ('A' as u8);
    let ours = *char_ours as u8 % 4;
    let bonus = get_bonus_from_choice(ours);

    match get_round_result(opponent, ours) {
        WIN=> 6 + bonus,
        LOSS => 0 + bonus,
        DRAW => 3 + bonus
    }
}

fn get_winning_choice(opponent: u8) -> u8 {
    match opponent {
        0 => 1,
        1 => 2,
        2 => 0,
        _ => { panic!("Unreachable") }
    }
}

fn get_losing_choice(opponent: u8) -> u8 {
    match opponent {
        0 => 2,
        1 => 0,
        2 => 1,
        _ => { panic!("Unreachable") }
    }
}

fn strat_2(pair: &(char, char)) -> usize {
    let (char_opponent, char_ours) = pair;
    let opponent = *char_opponent as u8 - ('A' as u8);
    let exp_match_result = match char_ours {
        'X' => LOSS,
        'Y' => DRAW,
        'Z' => WIN,
        _ => panic!("Unreachable")
    };

    match exp_match_result {
        WIN=> 6 + get_bonus_from_choice(get_winning_choice(opponent)),
        LOSS => 0 + get_bonus_from_choice(get_losing_choice(opponent)),
        DRAW => 3 + get_bonus_from_choice(opponent)
    }
}

pub fn run() {
    let file = File::open("inputs/day2").unwrap();
    let pairs  = get_pairs(BufReader::new(file).lines());

    println!("Day 2: ");
    println!("Part 1: {}", pairs.iter().map(strat_1).sum::<usize>());
    println!("Part 2: {}", pairs.iter().map(strat_2).sum::<usize>());
    println!("----------")
}