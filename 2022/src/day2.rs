use std::fs::File;
use std::io::{BufRead, BufReader};

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
    } else if (ours + 1) % 3 == opponent {
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

fn get_pair_from_line(line_res: Result<String, std::io::Error>) -> (char, char) {
    let line = line_res.unwrap();
    (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
}

pub fn run() {
    let file = File::open("inputs/day2").unwrap();
    let pairs: Vec<(char, char)> = BufReader::new(file).lines().map(get_pair_from_line).collect();

    println!("Day 2: ");
    println!("Part 1: {}", pairs.iter().map(strat_1).sum::<usize>());
    println!("Part 2: {}", pairs.iter().map(strat_2).sum::<usize>());
    println!("----------")
}