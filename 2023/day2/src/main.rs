use std::fs::File;
use std::io::{BufRead, BufReader};

enum Color {
    RED = 0,
    GREEN = 1,
    BLUE = 2
}

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<(Option<usize>, Option<usize>, Option<usize>)>
}

impl Game {
    pub fn parse(input_str: String) -> Game {
        let mut rounds: Vec<(Option<usize>, Option<usize>, Option<usize>)> = vec![];
        let split_input_rounds = input_str.as_str()[input_str.find(": ").unwrap() + 2..]
            .split("; ")
            .collect::<Vec<&str>>();

        for l in split_input_rounds {
            let mut game_draws: (Option<usize>, Option<usize>, Option<usize>) = (None, None, None);
            for value_pair in l.split(", ") {
                let nbr = value_pair.split(" ").next().unwrap().parse::<usize>().unwrap();
                let color = value_pair.split(" ").nth(1).unwrap();

                match color {
                    "red" => game_draws.0 = Some(nbr),
                    "green" => game_draws.1 = Some(nbr),
                    "blue" => game_draws.2 = Some(nbr),
                    _ => panic!("Invalid color")
                }

                dbg!(nbr, color);
            }

            rounds.push(game_draws);
        }

        Game {
            id: input_str.as_str()[5..input_str.find(':').unwrap()].parse::<usize>().unwrap(),
            subsets: rounds
        }
    }
}



fn main() {
    let file = File::open("../inputs/testday2").unwrap();
    let games: Vec<Game> = BufReader::new(file)
        .lines()
        .map(|l|Game::parse(l.unwrap()))
        .collect::<Vec<Game>>();

    dbg!(&games);
}
