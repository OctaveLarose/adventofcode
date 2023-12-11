use std::fs;

type Pos = usize; // maybe a (usize, usize) is more convenient?
type Symbol = char; // should that be an enum of the possible ones?

#[derive(Debug)]
struct Number {
    val: usize,
    pos: Pos,
    nbr_len: usize // number of digits. to not have to recalculate it constantly
}

#[derive(Debug)]
struct Schematic {
    width: usize,
    height: usize,
    numbers: Vec<Number>,
    symbols: Vec<(Symbol, Pos)>
}

impl Schematic {
    pub fn parse(input_file: String) -> Schematic {
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<(Symbol, Pos)> = vec![];

        let mut it = input_file.chars().enumerate();
        while let Some(tile) = it.next() {
            match tile.1 {
                '.' | '\n' => {},
                '0'..='9' => {
                    let nbr_len = input_file[tile.0..].find(|c: char| !c.is_numeric()).expect("Number not terminated somehow");
                    numbers.push(Number {
                        val: input_file[tile.0..tile.0 + nbr_len].parse::<usize>().unwrap(),
                        pos: tile.0,
                        nbr_len
                    });
                    it.nth(nbr_len);
                },
                c => match c {
                    '+' | '-' | '*' | '/' | '@' | '=' | '$' | '#' | '%' | '&' => symbols.push((c, tile.0)),
                    idk => panic!("{}", idk)
                },
            }
        }

        Schematic {
            width: input_file.find('\n').unwrap(),
            height: input_file.lines().count(),
            numbers,
            symbols }
    }

    pub fn part1_get_part_numbers_sum(&self) -> usize {
        let mut nbrs_sum = 0;

        for nbr in &self.numbers {
            for (_, sym_pos) in &self.symbols {
                // if (*sym_pos >= self.width * 2 && nbr.pos < (sym_pos - self.width * 2)) || // first bit checks for sub
                //     nbr.pos > (sym_pos + self.width * 2) {
                //     continue; // if it's not even close to the symbol. this check can be made more precise, but it's OK as is
                // }

                // nbr to the right of the sym
                if nbr.pos != 0 && *sym_pos == nbr.pos - 1 {
                    nbrs_sum += nbr.val;
                }

                // nbr is above sym
                if nbr.pos + self.width - 1 <= *sym_pos &&
                    *sym_pos <= nbr.pos + self.width + nbr.nbr_len + 1 {
                    nbrs_sum += nbr.val
                }
            }
        }

        nbrs_sum
    }
}

pub fn run() {
    let schematic = Schematic::parse(fs::read_to_string("../inputs/testday3").unwrap());

    println!("Day 3: ");
    println!("Part 1: {}", schematic.part1_get_part_numbers_sum());
    // println!("Part 2: {}", schematic.part1_get_part_numbers_sum());
    println!("----------");
}
