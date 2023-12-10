use std::fs;
use crate::common::{HandAndBid};

pub fn part1() {
    let input_file = fs::read_to_string("../inputs/day7").unwrap();
    let mut hand_bid_pairs: Vec<HandAndBid> = input_file.lines()
        .map(|line| HandAndBid::parse(line))
        .collect();

    hand_bid_pairs.sort();

    println!("Part 1: {}", hand_bid_pairs.iter()
        .enumerate()
        .map(|(rank, hand_and_bid)| (rank + 1) * hand_and_bid.bid )
        .sum::<usize>());
}