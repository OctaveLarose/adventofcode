use std::fs;
use itertools::Itertools;

type History = Vec<isize>;

fn part1_rec(sequence: &History) -> isize {
    let sub_seq = &sequence.windows(2).map(|a| a[1] - a[0]).collect::<History>();

    match sub_seq.iter().all_equal() {
        true => sequence.last().unwrap() + *sub_seq.iter().next().unwrap(),
        false => sequence.last().unwrap() + part1_rec(sub_seq)
    }
}

pub fn run() {
    let histories: Vec<History> = fs::read_to_string("../inputs/day9").unwrap()
        .lines()
        .map(|l| l.split_whitespace().map(|l2| l2.parse::<isize>().unwrap()).collect())
        .collect();

    // dbg!(&histories);
    println!("Part 1: {}", &histories.iter().map(part1_rec).sum::<isize>());
    // println!("Part 2: {}", part2(&nodes, &directions));
}
