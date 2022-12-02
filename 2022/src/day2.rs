use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

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

fn get_bonus_from_choice(choice: u8) -> usize {
    (choice % 4 + 1) as usize
}

fn strat_1(pair: &(char, char)) -> usize {
    let (char_opponent, char_ours) = pair;
    let opponent = *char_opponent as u8 - ('A' as u8);
    let ours = *char_ours as u8 % 4;
    let bonus = get_bonus_from_choice(ours);

    if opponent as u8 == ours {
        3 + bonus
    } else if opponent == ours + 1 || (ours == 2 && opponent == 0) {
        0 + bonus
    } else {
        6 + bonus
    }
}

fn strat_2(pair: &(char, char)) -> usize {
    let (char_opponent, char_ours) = pair;
    let opponent = *char_opponent as u8 - ('A' as u8);

    match char_ours {
        'Y' => 3 + get_bonus_from_choice(opponent),
        'Z'=> 6 + get_bonus_from_choice((opponent + 1) % 3),
        'X' => 0 + get_bonus_from_choice((opponent + 2) % 3),
        _ => { panic!("Unreachable")}
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