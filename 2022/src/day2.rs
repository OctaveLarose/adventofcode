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

fn get_match_score_part_1(pair: &(char, char)) -> usize {
    let (opponent, ours) = pair;
    let t_opponent = *opponent as u8;

    let t_ours = match ours {
        'Y' => b'B',
        'X' => b'A',
        'Z' => b'C',
        _ => { panic!("should be unreachable")}
    };

    let bonus = (*ours as u8 % 4 + 1) as usize;

    if t_opponent as u8 == t_ours {
        return 3 + bonus;
    }
    else if t_opponent == t_ours + 1 || (t_ours == b'C' && t_opponent == b'A') {
        return 0 + bonus
    }
    else {
        return 6 + bonus
    }
}

fn part1(pairs: &Vec<(char, char)>) -> usize {
    let mut score = 0;

    for pair in pairs {
        let t_score = get_match_score_part_1(pair);
        score += t_score;
    }
    score
}

pub fn run() {
    let file = File::open("inputs/day2").unwrap();
    let lines = BufReader::new(file).lines();
    let pairs  = get_pairs(lines);

    println!("Day 2: ");
    println!("Part 1: {}", part1(&pairs));
    // println!("Part 2: {}", part2(elves));
    println!("----------")
}