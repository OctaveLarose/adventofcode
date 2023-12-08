use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct RGB {
    r: Option<usize>,
    g: Option<usize>,
    b: Option<usize>
}

impl fmt::Debug for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R{:?} G{:?} B{:?}",
               if self.r.is_some() { self.r.unwrap() } else { 0 },
               if self.g.is_some() { self.g.unwrap() } else { 0 },
               if self.b.is_some() { self.b.unwrap() } else { 0 })
    }
}

struct Game {
    id: usize,
    rounds: Vec<RGB>
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}\n",
               &self.id,
               self.rounds.iter()
                   .map(|round| format!("{:?}", round))
                   .collect::<Vec<_>>()
                   .join("; ")
        )
    }
}

impl Game {
    pub fn parse(input_str: String) -> Game {
        let split_input_rounds = input_str.as_str()[input_str.find(": ").unwrap() + 2..]
            .split("; ")
            .collect::<Vec<&str>>();

        Game {
            id: input_str.as_str()[5..input_str.find(':').unwrap()].parse::<usize>().unwrap(),
            rounds: split_input_rounds.iter().map(|l| {
                let mut round: RGB = RGB {r: None, g: None, b: None};
                for value_pair in l.split(", ") {
                    let nbr = value_pair.split(" ").next().unwrap().parse::<usize>().unwrap();
                    let color = value_pair.split(" ").nth(1).unwrap();

                    match color {
                        "red" => round.r = Some(nbr),
                        "green" => round.g = Some(nbr),
                        "blue" => round.b = Some(nbr),
                        _ => panic!("Invalid color")
                    }
                }

                round
            }
            ).collect::<Vec<RGB>>()
        }
    }

    pub fn return_id_if_possible(&self, expectation: &RGB) -> Option<usize> {
        match self.rounds.iter()
            .any(|round| expectation.r < round.r || expectation.g < round.g || expectation.b < round.b)
        {
            true => None,
            false => Some(self.id)
        }
    }
}



fn main() {
    let file = File::open("../inputs/day2").unwrap();
    let games: Vec<Game> = BufReader::new(file)
        .lines()
        .map(|l|Game::parse(l.unwrap()))
        .collect::<Vec<Game>>();

    dbg!(&games);

    let part1_expectation: RGB = RGB {r: Some(12), g: Some(13), b: Some(14)};
    println!("Part 1: {}", games.iter()
        .filter_map(|g| g.return_id_if_possible(&part1_expectation))
        .sum::<usize>());
}
