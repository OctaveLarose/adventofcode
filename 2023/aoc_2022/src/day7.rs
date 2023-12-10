use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fs;
use itertools::Itertools;
use crate::day7::CardType::*;
use crate::day7::HandType::*;

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub enum CardType { Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

impl CardType {
    pub fn from_char(c: char) -> CardType {
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            invalid_card => panic!("Invalid card: {:?}", invalid_card)
        }
    }

    pub fn from_char_with_joker_rules(c: char) -> CardType {
        match CardType::from_char(c) {
            Jack => Joker,
            any => any
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug)]
pub(crate) struct Hand {
    pub(crate) cards: Vec<CardType>, // template is because it can be either a part1::CardType or a part2::CardType
    pub(crate) hand_type: HandType
}

impl Hand {
    fn from_str(str: String) -> Hand {
        let cards = str.chars()
            .map(CardType::from_char)
            .collect::<Vec<CardType>>();

        Hand {
            hand_type: Hand::get_type(&cards),
            cards
        }
    }

    fn from_str_joker_rules(str: String) -> Hand {
        let cards = str.chars()
            .map(CardType::from_char_with_joker_rules)
            .collect::<Vec<CardType>>();

        Hand {
            hand_type: Hand::get_type_with_joker_rules(&cards),
            cards,
        }
    }

    pub fn get_type(cards: &Vec<CardType>) -> HandType {
        let unique_items: Vec<(usize, &CardType)> = cards.iter().filter_map(|c| {
            Some((cards.iter().filter(|c2: &&CardType| *c2 == c).count(), c)) })
            .unique()
            .collect();

        // this RETURNED WRONG RESULTS for some reason >:(
        // let unique_items = cards.into_iter().dedup_with_count().collect::<Vec<_>>();

        match unique_items.len() {
            1 => return FiveOfAKind,
            2 => {
                match unique_items.iter().next().unwrap() {
                    (4, _) | (1, _) => return FourOfAKind,
                    (3, _) | (2, _) => return FullHouse,
                    _ => panic!("Should be an impossible combination of cards.")
                }
            }
            3 => {
                if unique_items.iter().any(|(nbr, _)| *nbr == 3) {
                    return ThreeOfAKind
                } else if unique_items.iter().filter(|(nbr, _)| *nbr == 2).count() == 2 {
                    return TwoPair
                } else {
                    panic!("Should be an impossible combination of cards.")
                }
            },
            4 => OnePair,
            5 => HighCard,
            _ => panic!("...there are more than 5 cards. What?")
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
            5 => std::panic!("5 unique items mean no jokers, which we should have checked for already"),
            _ => std::panic!("...more than 5 unique items in a 5 card hand. Not normal")
        }
    }

    // main function for sort
    pub fn does_it_beat(&self, other_hand: &Hand) -> Ordering {
        if self.hand_type > other_hand.hand_type {
            return Greater;
        } else if self.hand_type < other_hand.hand_type {
            return Less;
        } else {
            for (ours, theirs) in self.cards.iter().zip(&other_hand.cards) {
                if ours > theirs {
                    return Greater;
                } else if ours < theirs {
                    return Less;
                }
            }
        }
        panic!("I thought there were no hands that were exactly the same?")
    }
}

#[derive(Debug)]
pub(crate) struct HandAndBid {
    pub(crate) hand: Hand,
    pub(crate) bid: usize
}

impl Eq for HandAndBid {}

impl PartialEq<Self> for HandAndBid {
    fn eq(&self, _: &Self) -> bool {
        panic!("Hands can't be equal. Should this ever get invoked?")
    }
}

impl PartialOrd<Self> for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.does_it_beat(&other.hand))
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.does_it_beat(&other.hand)
    }
}

impl HandAndBid {
    pub(crate) fn parse(input_str: &str) -> HandAndBid {
        HandAndBid {
            hand: Hand::from_str(String::from(input_str.split_whitespace().nth(0).unwrap())),
            bid: input_str.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap()
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


fn part1() {
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

fn part2() {
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

pub fn run() {
    part1();
    part2();
}