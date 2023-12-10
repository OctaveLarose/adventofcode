use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use itertools::Itertools;
use std::fs;
use crate::part1::HandType::*;

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
enum CardType { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

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
            'J' => CardType::Jack,
            'Q' => CardType::Queen,
            'K' => CardType::King,
            'A' => CardType::Ace,
            invalid_card => panic!("Invalid card: {:?}", invalid_card)
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
pub(crate) struct Hand<T> {
    pub(crate) cards: Vec<T>, // template is because it can be either a part1::CardType or a part2::CardType
    pub(crate) hand_type: HandType
}

impl<T: PartialOrd> Hand<T> {
    fn from_str(str: String) -> Hand<CardType> {
        let cards = str.chars()
            .map(CardType::from_char)
            .collect::<Vec<CardType>>();

        Hand {
            hand_type: Hand::<T>::get_type(&cards),
            cards
        }
    }

    fn get_type(cards: &Vec<CardType>) -> HandType {
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

    // main function for sort
    pub fn does_it_beat(&self, other_hand: &Hand<T>) -> Ordering {
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
pub(crate) struct HandAndBid<T> {
    pub(crate) hand: Hand<T>,
    pub(crate) bid: usize
}

impl<T: Eq> Eq for HandAndBid<T> {}

impl<T: PartialEq> PartialEq<Self> for HandAndBid<T> {
    fn eq(&self, _: &Self) -> bool {
        panic!("Hands can't be equal. Should this ever get invoked?")
    }
}

impl<T: PartialOrd> PartialOrd<Self> for HandAndBid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.does_it_beat(&other.hand))
    }
}

impl<T: Ord> Ord for HandAndBid<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.does_it_beat(&other.hand)
    }
}

impl<T> HandAndBid<T> {
    fn parse(input_str: &str) -> HandAndBid<CardType> {
        HandAndBid {
            hand: Hand::<CardType>::from_str(String::from(input_str.split_whitespace().nth(0).unwrap())),
            bid: input_str.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap()
        }
    }
}

pub fn part1() {
    let input_file = fs::read_to_string("../inputs/day7").unwrap();
    let mut hand_bid_pairs: Vec<HandAndBid<CardType>> = input_file.lines()
        .map(|line| HandAndBid::<CardType>::parse(line))
        .collect();

    hand_bid_pairs.sort();

    println!("Part 1: {}", hand_bid_pairs.iter()
        .enumerate()
        .map(|(rank, hand_and_bid)| (rank + 1) * hand_and_bid.bid )
        .sum::<usize>());
}