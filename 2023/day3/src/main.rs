use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = usize; // maybe a (usize, usize) is more convenient?
type Symbol = char; // should that be an enum of the possible ones?

#[derive(Debug)]
struct Schematic {
    width: usize,
    numbers: Vec<(usize, Pos)>,
    symbols: Vec<(Symbol, Pos)>
}

impl Schematic {
    pub fn parse(input_file: String) -> Schematic {
        let width = input_file.find('\n').unwrap();
        let mut numbers: Vec<(usize, Pos)> = vec![];
        let mut symbols: Vec<(Symbol, Pos)> = vec![];

        let mut it = input_file.chars().enumerate();
        while let Some(tile) = it.next() {
            match tile.1 {
                '.' | '\n' => {},
                '0'..='9' => {
                    let nbr_len = input_file[tile.0..].find(|c: char| !c.is_numeric()).expect("Uh what");
                    numbers.push((input_file[tile.0..tile.0 + nbr_len].parse::<usize>().unwrap(), tile.0));
                    it.nth(nbr_len);
                },
                c => match c {
                    '+' | '-' | '*' | '/' | '@' | '=' | '$' | '#' | '%' | '&' => symbols.push((c, tile.0)),
                    idk => panic!("{}", idk)
                },
            }
        }

        Schematic { width, numbers, symbols }
    }
}

fn main() {
    let schematic = Schematic::parse(fs::read_to_string("../inputs/testday3").unwrap());

    dbg!(&schematic);
}
