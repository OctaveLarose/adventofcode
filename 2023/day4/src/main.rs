use std::fs;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_nbrs: Vec<usize>,
    nbrs: Vec<usize>
}

impl Card {
    pub fn parse(card_str: &str) -> Card {
        let three_parter = card_str.split([':', '|']).collect::<Vec<&str>>();

        Card {
            id: three_parter.get(0).unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>().unwrap(),
            winning_nbrs: three_parter.get(1).unwrap()
                .split_whitespace()
                .map(|nbr_str| nbr_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            nbrs: three_parter.get(2).unwrap()
                .split_whitespace()
                .map(|nbr_str| nbr_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        }
    }

    pub fn get_value(&self) -> usize {
        let nbr_wins = self.nbrs.iter()
            .filter(|nbr| { self.winning_nbrs.contains(nbr)})
            .count();

        match nbr_wins {
            0 => 0,
            _ => usize::pow(2, (nbr_wins - 1) as u32)
        }
    }
}

fn main() {
    let cards = fs::read_to_string("../inputs/day4").unwrap()
        .lines()
        .map(|s| Card::parse(s))
        .collect::<Vec<Card>>();

    // dbg!(&cards);

    println!("Part 1: {}", cards.iter().map(|c| c.get_value()).sum::<usize>());
}
