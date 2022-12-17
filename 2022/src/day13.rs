use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;
use itertools::{
    Itertools,
    EitherOrBoth,
};

#[derive(Eq)]
struct Packet {
    children: Option<Vec<Packet>>,
    val: Option<u8>
}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        Packet::compare(self, other) == Equal
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Packet::compare(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        Packet::compare(self, other)
    }
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        let mut packet_str = String::new();

        if self.val.is_some() {
            packet_str.push_str(&*self.val.unwrap().to_string())
        } else {
            packet_str.push_str("[");
            if self.children.is_some() {
                for (child, is_last_element) in self.children.as_ref().unwrap().iter().enumerate()
                    .map(|(i, w)| (w, i == self.children.as_ref().unwrap().len() - 1)) {
                    packet_str.push_str(&*child.to_string());
                    if !is_last_element {
                        packet_str.push_str(",");
                    }
                }
            }
            packet_str.push_str("]");
        }

        packet_str
    }
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

    fn compare(packet1: &Packet, packet2: &Packet) -> Ordering {
        if packet1.val.is_some() && packet2.val.is_some() {
            let nbr1 = packet1.val.unwrap();
            let nbr2 = packet2.val.unwrap();

            return if nbr1 == nbr2 {
                Equal
            } else if nbr1 < nbr2 {
                Less
            } else {
                Greater
            }
        }

        if packet1.children.is_none() && packet1.val.is_none() && packet2.children.is_none() && packet2.val.is_none() {
            return Equal;
        } else if packet1.children.is_none() && packet1.val.is_none() {
            return Less;
        } else if packet2.children.is_none() && packet2.val.is_none() {
            return Greater;
        }

        // there's gotta be a better way to do this but i need a ref to a vec so i need it on the stack...
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
                EitherOrBoth::Left(_) => return Greater, // if the right side runs out of elements
                EitherOrBoth::Right(_) => return Less, // same with left
                EitherOrBoth::Both(l, r) => {
                    let comparison_result = Packet::compare(l, r);
                    match comparison_result {
                        Less => return Less,
                        Greater => return Greater,
                        Equal => {}
                    }
                },
            }
        }

        Equal
    }
}

fn part1(pairs: &Vec<(Packet, Packet)>) -> usize {
    pairs.iter().enumerate()
        .filter_map(|(idx, (p1, p2))| if Packet::compare(p1, p2) == Less { Some(idx + 1) } else { None })
        .sum()
}

fn part2(packets: &mut Vec<Packet>) -> usize {
    quicksort::quicksort(packets);

    let product_dividers = packets.iter().enumerate()
        .filter_map(|(idx, p)| {
            if p.to_string() == "[[2]]" || p.to_string() == "[[6]]" {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product::<usize>();

    product_dividers
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/day13").unwrap();
    let pairs = file_str.split("\n\n")
        .map(| p_str | {
           let mut packet_split = p_str.split('\n');
            (Packet::parse(packet_split.nth(0).unwrap()),
             Packet::parse(packet_split.nth(0).unwrap()))
        })
        .collect::<Vec<(Packet, Packet)>>();

    let mut all_packets = file_str.split("\n")
        .filter_map(|l| if l.len() != 0 { Some(l) } else { None })
        .map(|res_str| Packet::parse(res_str))
        .collect::<Vec<Packet>>();
    all_packets.push(Packet::parse("[[2]]"));
    all_packets.push(Packet::parse("[[6]]"));

    println!("Day 13: ");
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&mut all_packets));
    println!("----------");
}