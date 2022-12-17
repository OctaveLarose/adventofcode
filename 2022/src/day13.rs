use std::fs;

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

        let mut nbr_open = 0;
        let mut nbr_closed = 0;

        Packet {
            children: {
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

    // dbg!(&pairs[0]);
    // dbg!(&pairs[1]);
    dbg!(&pairs[7]);

    println!("Day 13: ");
    println!("Part 1: {}", part1(&pairs));
    // println!("Part 2: {}", part2(&pairs));
    println!("----------");
}