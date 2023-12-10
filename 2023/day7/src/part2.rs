use itertools::Itertools;
use std::{fs, panic};
use crate::common::{CardType, Hand, HandAndBid, HandType};
use crate::common::CardType::*;
use crate::common::HandType::*;

impl CardType {
    pub fn from_char_with_joker_rules(c: char) -> CardType {
        match CardType::from_char(c) {
            Jack => Joker,
            any => any
        }
    }
}

impl Hand {
    fn from_str_joker_rules(str: String) -> Hand {
        let cards = str.chars()
            .map(CardType::from_char_with_joker_rules)
            .collect::<Vec<CardType>>();

        Hand {
            hand_type: Hand::get_type_with_joker_rules(&cards),
            cards,
        }
    }

    fn get_type_with_joker_rules(cards: &Vec<CardType>) -> HandType {
        let unique_non_joker_items: Vec<(usize, &CardType)> = cards.iter().filter_map(|c| {
            match c {
                Joker => None,
                _ => Some((cards.iter().filter(|c2: &&CardType| *c2 == c).count(), c))
            }})
            .unique()
            .collect();
        let nbr_jokers: usize = cards.iter().filter(|c| **c == Joker).count();

        if nbr_jokers == 0 {
            return Hand::get_type(cards);
        }

        match unique_non_joker_items.len() {
            0 => FiveOfAKind, // Five jokers!
            1 => return FiveOfAKind, // if there's only one non-joker unique item, the rest are jokers.
            2 => {
                match unique_non_joker_items.iter().all(|(nbr, _)| *nbr == 2) {
                    false => FourOfAKind, // A B J J J ; A A B J J; A B B B J
                    true => FullHouse // A A B B J
                }
            },
            3 => return ThreeOfAKind, // A B C J J; A B C C J
            4 => return OnePair, // A B C D J
            5 => panic!("5 unique items mean no jokers, which we should have checked for already"),
            _ => panic!("...more than 5 unique items in a 5 card hand. Not normal")
        }
    }
}

impl HandAndBid {
    fn parse_with_joker_rules(input_str: &str) -> HandAndBid {
        HandAndBid {
            hand: Hand::from_str_joker_rules(String::from(input_str.split_whitespace().nth(0).unwrap())),
            bid: input_str.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap()
        }
    }
}

pub fn part2() {
    let input_file = fs::read_to_string("../inputs/day7").unwrap();
    let mut hand_bid_pairs: Vec<HandAndBid> = input_file.lines()
        .map(|line| HandAndBid::parse_with_joker_rules(line))
        .collect();

    hand_bid_pairs.sort();

    println!("Part 2: {}", hand_bid_pairs.iter()
        .enumerate()
        .map(|(rank, hand_and_bid)| (rank + 1) * hand_and_bid.bid)
        .sum::<usize>());
}