use std::fs::File;
use std::io::{BufRead, BufReader};

fn pair_maker(line: &String) -> ((usize, usize), (usize, usize)) {
    let (first, second) = line.split_once(",").unwrap();
    let (a, b) = first.split_once("-").unwrap();
    let (c, d) = second.split_once("-").unwrap();

    ((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()),
     (c.parse::<usize>().unwrap(), d.parse::<usize>().unwrap()))
}

fn part1(pairs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut sum = 0;

    for p in pairs {
        if (p.0.0 <= p.1.0 && p.0.1 >= p.1.1) || (p.1.0 <= p.0.0 && p.1.1 >= p.0.1)  {
            sum += 1;
        }
    }
    sum
}

fn part2(pairs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut sum = 0;

    for p in pairs {
        if p.0.0 >= p.1.0 && p.0.0 <= p.1.1 {
            sum += 1;
        } else if p.0.1 >= p.1.0 && p.0.1 <= p.1.1 {
            sum += 1;
        } else if p.1.0 >= p.0.0 && p.1.0 <= p.0.1 {
            sum += 1;
        } else if p.1.1 >= p.0.0 && p.1.1 <= p.0.1 {
            sum += 1;
        }
    }
    sum
}

pub fn run() {
    let file = File::open("inputs/day4").unwrap();
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let pairs = lines.iter().map(pair_maker).collect::<Vec<((usize, usize), (usize, usize))>>();

    println!("Day 3: ");
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
    println!("----------")
}