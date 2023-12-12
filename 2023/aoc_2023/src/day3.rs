use std::fs;
use crate::map::{Map2D, CharMapElement};

type Symbol = char; // should that be an enum of the possible ones?

#[derive(Debug)]
struct Number {
    val: usize,
    pos: usize,
    nbr_len: usize // number of digits. to not have to recalculate it constantly
}


#[derive(Debug)]
struct Schematic {
    map: Map2D<CharMapElement>,
    numbers: Vec<Number>,
    symbols: Vec<(Symbol, usize)>
}

impl Schematic {
    pub fn parse(input_file: String) -> Schematic {
        let map: Map2D<CharMapElement> = Map2D::parse(input_file.as_str());
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<(Symbol, usize)> = vec![];

        let mut it = map.tiles.iter().filter(|c| c.c != '\n').enumerate();
        while let Some(tile) = it.next() {
            match tile.1.c {
                '.' | '\n' => {},
                '0'..='9' => {
                    let row = tile.0 / map.width;
                    let end_of_line = (row + 1) * map.width;
                    let nbr_len = if let Some(s) = map.tiles[tile.0..end_of_line].iter()
                        .position(|c| !c.c.is_numeric())
                    {
                        s
                    } else {
                        end_of_line - tile.0
                    };

                    let num_str = map.tiles[tile.0..tile.0 + nbr_len].iter().map(|c| c.c).collect::<String>();
                    let num_val = num_str.parse::<usize>().unwrap();
                    numbers.push(Number {
                        val: num_val,
                        pos: tile.0,
                        nbr_len
                    });
                    if nbr_len != 1 {
                        it.nth(nbr_len - 2);
                    }
                },
                c => match c {
                    '+' | '-' | '*' | '/' | '@' | '=' | '$' | '#' | '%' | '&' => symbols.push((c, tile.0)),
                    idk => panic!("{}", idk)
                },
            }
        }

        Schematic {
            map,
            numbers,
            symbols }
    }

    pub fn part1_get_part_numbers_sum(&self) -> usize {
        self.numbers.iter().filter_map(| nbr|{
            for (_, sym_pos) in &self.symbols {
                for surround_pos in self.map.get_positions_around(*sym_pos) {
                    match surround_pos {
                        None => {}
                        Some(v) => match nbr.pos <= v && v <= nbr.pos + nbr.nbr_len {
                            true => { dbg!(nbr.val); return Some(nbr.val)},
                            false => {}
                        }
                    }
                }
            }
            None
        })
            .sum()
    }
}

pub fn run() {
    let schematic = Schematic::parse(fs::read_to_string("../inputs/day3").unwrap());

    println!("Day 3: ");
    println!("Part 1: {}", schematic.part1_get_part_numbers_sum());
    // println!("Part 2: {}", schematic.part1_get_part_numbers_sum());
    println!("----------");
}
