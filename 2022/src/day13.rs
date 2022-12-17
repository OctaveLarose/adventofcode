use std::fs;
use itertools::{
    Itertools,
    EitherOrBoth,
};
use crate::day13::ComparisonResult::{EQUIV, RIGHT, WRONG};

#[derive(Debug, Eq, PartialEq)]
enum ComparisonResult {
    RIGHT,
    WRONG,
    EQUIV
}

#[derive(Debug)]
struct Packet {
    children: Option<Vec<Packet>>,
    val: Option<u8>
}

impl Packet {
    fn parse(str: &str) -> Packet {
        // a number
        if str.chars().next().unwrap() != '[' {
            return Packet {children: None, val: Some(str.parse::<u8>().unwrap())}
        }

        // we assume they're the first and last if there's only one of each.
        if str.matches('[').count() == 1 && str.matches(']').count() == 1 {
            let children_str = &str[1..str.len() - 1];
            if children_str.len() == 0 {
                return Packet {children: None, val: None};
            }
            return Packet {
                children: Some(children_str
                    .split(',')
                    .map(|nbr_str| Packet {children: None, val: nbr_str.parse::<u8>().ok()})
                    .collect::<Vec<Packet>>()),
                val: None };
        }

        Packet {
            children: {
                let mut nbr_open = 0;
                let mut nbr_closed = 0;

                Some(str[1..str.len() - 1]
                    .split(|c| {
                        match c {
                            '[' => nbr_open += 1,
                            ']' => nbr_closed += 1,
                            _ => {}
                        };
                        return c == ',' && nbr_open == nbr_closed // i.e. we are in the outermost list scope, the current packet
                    })
                    .map(|c_str| Packet::parse(c_str))
                    .collect::<Vec<Packet>>()
                )
            },
            val: None
        }
    }

    fn compare(packet1: &Packet, packet2: &Packet) -> ComparisonResult {
        if packet1.val.is_some() && packet2.val.is_some() {
            let nbr1 = packet1.val.unwrap();
            let nbr2 = packet2.val.unwrap();

            return if nbr1 == nbr2 {
                EQUIV
            } else if nbr1 < nbr2 {
                RIGHT
            } else {
                WRONG
            }
        }

        if packet1.children.is_none() && packet1.val.is_none() {
            return RIGHT;
        } else if packet2.children.is_none() && packet2.val.is_none() {
            return WRONG;
        }

        let packet_from_val_1 = vec![Packet {children: None, val: packet1.val}];
        let children1: &Vec<Packet> = match &packet1.children {
            Some(c) => c,
            _ => &packet_from_val_1
        };
        let packet_from_val_2 = vec![Packet {children: None, val: packet2.val}];
        let children2: &Vec<Packet> = match &packet2.children {
            Some(c) => c,
            _ => &packet_from_val_2
        };

        for pair in children1.iter().zip_longest(children2.iter()) {
            match pair {
                EitherOrBoth::Left(_) => return WRONG, // if the right side runs out of elements
                EitherOrBoth::Right(_) => return RIGHT, // same with left
                EitherOrBoth::Both(l, r) => {
                    let comparison_result = Packet::compare(l, r);
                    match comparison_result {
                        RIGHT => return RIGHT,
                        WRONG => return WRONG,
                        EQUIV => {}
                    }
                },
            }
        }

        EQUIV
    }
}

#[derive(Debug)]
struct Pair {
    packet1: Packet,
    packet2: Packet,
}

impl Pair {
    fn compare(&self) -> ComparisonResult {
        Packet::compare(&self.packet1, &self.packet2)
    }
}

fn part1(pairs: &Vec<Pair>) -> usize {
    for (idx, pair) in pairs.iter().enumerate() {
        println!("{}: {:?}", idx + 1, pair.compare());
    }

    pairs.iter().enumerate()
        .filter_map(|(idx, p)| if p.compare() == RIGHT { Some(idx + 1) } else { None })
        .sum()
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/day13").unwrap();
    let pairs = file_str.split("\n\n")
        .map(| p_str | {
           let mut packet_split = p_str.split('\n');
            Pair {
                packet1: Packet::parse(packet_split.nth(0).unwrap()),
                packet2: Packet::parse(packet_split.nth(0).unwrap())
            }
        })
        .collect::<Vec<Pair>>();

    println!("Day 13: ");
    println!("Part 1: {}", part1(&pairs));
    // println!("Part 2: {}", part2(&pairs));
    println!("----------");
}