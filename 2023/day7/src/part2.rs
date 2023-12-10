use itertools::Itertools;
use std::fs;
use crate::part1::HandType::*;
use crate::part1::{Hand, HandType};
use crate::part1::HandAndBid;

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
enum CardType { Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Queen, King, Ace }

impl CardType {
    pub fn from_char(c: char) -> CardType {
        match c {
            '2' => CardType::Two,
            '3' => CardType::Three,
            '4' => CardType::Four,
            '5' => CardType::Five,
            '6' => CardType::Six,
            '7' => CardType::Seven,
            '8' => CardType::Eight,
            '9' => CardType::Nine,
            'T' => CardType::Ten,
            'J' => CardType::Joker,
            'Q' => CardType::Queen,
            'K' => CardType::King,
            'A' => CardType::Ace,
            invalid_card => panic!("Invalid card: {:?}", invalid_card)
        }
    }
}

impl<T> Hand<T> {
    fn from_str_joker_rules(str: String) -> Hand<CardType> {
        let cards = str.chars()
            .map(CardType::from_char)
            .collect::<Vec<CardType>>();

        Hand {
            hand_type: Hand::<CardType>::get_type_with_joker_rules(&cards),
            cards,
        }
    }

    fn get_type_with_joker_rules(cards: &Vec<CardType>) -> HandType {
        let unique_items: Vec<(usize, &CardType)> = cards.iter().filter_map(|c| {
            match c {
                CardType::Joker => None,
                _ => Some((cards.iter().filter(|c2: &&CardType| *c2 == c).count(), c))
            }})
            .unique()
            .collect();
        let nbr_jokers: usize = cards.iter().filter(|c| **c == CardType::Joker).count();


        dbg!(&nbr_jokers);

        match unique_items.len() {
            1 => return FiveOfAKind, // if there's only one non-joker unique item, the rest are jokers anyway.
            2 => {
                match unique_items.iter().next().unwrap() {
                    (4, _) | (1, _) => return FourOfAKind,
                    (3, _) | (2, _) => return FullHouse,
                    _ => panic!("Should be an impossible combination of cards.")
                }
            }
            3 => {
                if unique_items.iter().any(|(nbr, _)| *nbr == 3) {
                    return ThreeOfAKind;
                } else if unique_items.iter().filter(|(nbr, _)| *nbr == 2).count() == 2 {
                    return TwoPair;
                } else {
                    panic!("Should be an impossible combination of cards.")
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => panic!("...there are more than 5 cards. What?")
        }
    }
}

impl<T> HandAndBid<T> {
    fn parse_with_joker_rules(input_str: &str) -> HandAndBid<CardType> {
        HandAndBid {
            hand: Hand::<CardType>::from_str_joker_rules(String::from(input_str.split_whitespace().nth(0).unwrap())),
            bid: input_str.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap()
        }
    }
}

pub fn part2() {
    let input_file = fs::read_to_string("../inputs/testday7").unwrap();
    let mut hand_bid_pairs: Vec<HandAndBid<CardType>> = input_file.lines()
        .map(|line| HandAndBid::<CardType>::parse_with_joker_rules(line))
        .collect();

    hand_bid_pairs.sort();

    println!("Part 2: {}", hand_bid_pairs.iter()
        .enumerate()
        .map(|(rank, hand_and_bid)| (rank + 1) * hand_and_bid.bid)
        .sum::<usize>());
}