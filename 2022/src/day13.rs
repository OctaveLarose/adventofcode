use std::fs;

#[derive(Debug)]
struct Packet {
    children: Vec<Packet>,
    val: Option<u8>
}

impl Packet {
    fn parse(_str: &str) -> Packet {
        // TODO write
        Packet { children: vec![], val: None }
    }

    fn compare(&self, packet2: &Packet) -> bool {
        if self.val.is_some() && packet2.val.is_some() {
            let nbr1 = self.val.unwrap();
            let nbr2 = packet2.val.unwrap();
            if nbr1 == nbr2 {
                return true; // we continue? may need an output right order/wrong order/continue.
            }
            return nbr1 < nbr2;
        }
        false // TODO write
    }
}

#[derive(Debug)]
struct Pair {
    packet1: Packet,
    packet2: Packet,
}

impl Pair {
    fn compare(&self) -> bool {
        self.packet1.compare(&self.packet2)
    }
}

fn part1(pairs: &Vec<Pair>) -> usize {
    // we'll use this for debugging.
    for (idx, pair) in pairs.iter().enumerate() {
        println!("{}: {}", idx, pair.compare());
    }

    pairs.iter().enumerate()
        .filter_map(|(idx, p)| if p.compare() { Some(idx) } else { None })
        .sum()
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/testday13").unwrap();
    let pairs = file_str.split("\n\n")
        .map(| p_str | {
           let mut packet_split = p_str.split('\n');
            Pair {
                packet1: Packet::parse(packet_split.nth(0).unwrap()),
                packet2: Packet::parse(packet_split.nth(0).unwrap())
            }
        })
        .collect::<Vec<Pair>>();

    dbg!(&pairs);

    println!("Day 13: ");
    println!("Part 1: {}", part1(&pairs));
    // println!("Part 2: {}", part2(&pairs));
    println!("----------");
}