use std::fmt::{Debug, Display, Formatter};
use std::fs;
use itertools::Itertools;
use crate::map::{Map2D, CharMapElement};

type Symbol = char; // should that be an enum of the possible ones?

#[derive(Debug, Hash)]
struct Number {
    val: usize,
    pos: usize,
    nbr_len: usize, // number of digits. to not have to recalculate it constantly
}

impl Number {
    pub fn is_pos_part_of_number(&self, surround_pos: usize) -> bool {
        self.pos <= surround_pos && surround_pos <= self.pos + self.nbr_len - 1
    }
}

impl PartialEq<Self> for Number {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.pos == other.pos && self.nbr_len == other.nbr_len
    }
}

impl Eq for Number {}


#[derive(Debug)]
struct Schematic {
    map: Map2D<CharMapElement>,
    numbers: Vec<Number>,
    symbols: Vec<(Symbol, usize)>,
}

// bad, inefficient code made as a sanity check.
impl Display for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;

        while i < (self.map.width * self.map.height) {
            let matching_sym = self.symbols.iter().find(|(_, sym_pos)| { *sym_pos == i });
            let matching_num = self.numbers.iter().find(|n| { n.pos == i });

            if matching_sym.is_some() {
                write!(f, "{}", matching_sym.unwrap().0).expect("write failed");
            } else if matching_num.is_some() {
                write!(f, "{}", matching_num.unwrap().val).expect("write failed");
                i += matching_num.unwrap().nbr_len - 1;
            } else {
                write!(f, ".").expect("write failed");
            }

            if (i + 1) % self.map.width == 0 {
                write!(f, "\n").expect("write failed");
            }

            i += 1;
        }

        Ok(())
    }
}

impl Schematic {
    pub fn parse(input_file: String) -> Schematic {
        let map: Map2D<CharMapElement> = Map2D::parse(input_file.as_str());
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<(Symbol, usize)> = vec![];

        let mut it = map.tiles.iter().filter(|c| c.0 != '\n').enumerate();
        while let Some((pos, char_elem)) = it.next() {
            match char_elem.0 {
                '.' => {}
                '0'..='9' => {
                    let end_of_line = (pos / map.width + 1) * map.width;
                    let nbr_len = map.tiles[pos..end_of_line].iter()
                        .position(|c| !c.0.is_numeric())
                        .unwrap_or(end_of_line - pos);

                    numbers.push(Number {
                        val: map.tiles[pos..pos + nbr_len].iter()
                            .map(|c| c.0)
                            .collect::<String>()
                            .parse::<usize>().unwrap(),
                        pos,
                        nbr_len,
                    });

                    // skip the next digits if they exist, and account for the fact we do a next() at the start of each loop (therefore -2 and not -1)
                    if nbr_len != 1 {
                        it.nth(nbr_len - 2);
                    }
                }
                c => match c {
                    '+' | '-' | '*' | '/' | '@' | '=' | '$' | '#' | '%' | '&' => symbols.push((c, pos)),
                    idk => panic!("{}", idk)
                },
            }
        }

        Schematic {
            map,
            numbers,
            symbols,
        }
    }

    pub fn part1_get_part_numbers_sum(&self) -> usize {
        self.numbers.iter().filter_map(|nbr| {
            for (_, sym_pos) in &self.symbols {
                for surround_pos_opt in self.map.get_positions_around(*sym_pos) {
                    match surround_pos_opt {
                        None => {}
                        Some(surround_pos) => match nbr.is_pos_part_of_number(surround_pos) {
                            true => return Some(nbr.val),
                            false => {}
                        }
                    }
                }
            }
            None
        })
            .sum()
    }

    pub fn part2_get_gear_ratios_sum(&self) -> usize {
        self.symbols.iter()
            .filter(|(sym, _)| *sym == '*')
            .filter_map(|(_, sym_pos)| {
                let numbers_around = self.map.get_positions_around(*sym_pos).iter()
                    .filter_map(|surround_pos_opt| {
                        surround_pos_opt.and_then(|surround_pos| self.numbers.iter().find(|nbr| nbr.is_pos_part_of_number(surround_pos)))
                    })
                    .unique()
                    .collect::<Vec<&Number>>();

                (numbers_around.len() == 2).then(|| numbers_around.iter().map(|n| { n.val }).product::<usize>())
            })
            .sum()
    }
}

pub fn run() {
    let schematic = Schematic::parse(fs::read_to_string("../inputs/day3").unwrap());

    // println!("{}", &schematic);

    println!("Day 3: ");
    println!("Part 1: {}", schematic.part1_get_part_numbers_sum());
    println!("Part 2: {}", schematic.part2_get_gear_ratios_sum());
    println!("----------");
}
