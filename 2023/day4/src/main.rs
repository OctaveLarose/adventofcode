use std::fs;

#[derive(Debug)]
struct Card {
    id: usize, // turned out to be useless! oops.
    winning_nbrs: Vec<usize>,
    nbrs: Vec<usize>
}

impl Card {
    pub fn parse(card_str: &str) -> Card {
        let three_parter: [&str; 3] = card_str.split([':', '|'])
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        Card {
            id: three_parter[0]
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>().unwrap(),
            winning_nbrs: three_parter[1]
                .split_whitespace()
                .map(|nbr_str| nbr_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            nbrs: three_parter[2]
                .split_whitespace()
                .map(|nbr_str| nbr_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        }
    }

    pub fn get_nbr_winning_nbrs(&self) -> usize {
        self.nbrs.iter().filter(|nbr| { self.winning_nbrs.contains(nbr)}).count()
    }

    pub fn get_value(&self) -> usize {
        match self.get_nbr_winning_nbrs()
        {
            0 => 0,
            win_nbr => usize::pow(2, (win_nbr - 1) as u32)
        }
    }
}

fn part2(cards: &Vec<Card>) -> usize {
    let mut wins_per_card: Vec<usize> = vec![0; cards.len()];

    for (card_idx, card) in cards.iter().enumerate() {
        let current_nbr_wins = wins_per_card[card_idx];

        wins_per_card[card_idx] += 1; // current card always wins

        match card.get_nbr_winning_nbrs() {
            0 => continue,
            nbr_wins_for_this_card => {
                for _ in 1..=current_nbr_wins + 1 { // one time for the current card + N times for the copies
                    for j in card_idx + 1..card_idx + nbr_wins_for_this_card + 1 {
                        wins_per_card[j] += 1;
                    }
                }
            }
        }
    }

    wins_per_card.iter().sum()
}

fn main() {
    let cards = fs::read_to_string("../inputs/day4").unwrap()
        .lines()
        .map(|s| Card::parse(s))
        .collect::<Vec<Card>>();

    println!("Part 1: {}", cards.iter().map(|c| c.get_value()).sum::<usize>());
    println!("Part 2: {}", part2(&cards));
}
