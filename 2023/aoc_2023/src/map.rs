use crate::map::Direction::*;

pub enum Direction {
    NW, N, NE,
    W, E,
    SW, S, SE
}

#[derive(Debug)]
pub struct Map2D<T> {
    pub(crate) width: usize,
    height: usize,
    tiles: Vec<T>,
}

pub trait MapElement {
    fn parse_from_char(c: char) -> Self;
}

#[derive(Debug)]
pub struct CharMapElement {
    c: char
}

impl MapElement for CharMapElement {
    fn parse_from_char(c: char) -> CharMapElement {
        CharMapElement { c }
    }
}

impl<T: MapElement> Map2D<T> {
    pub fn parse(input_str: &str) -> Map2D<T> {
        Map2D {
            width: input_str.find('\n').unwrap(),
            height: input_str.lines().count(),
            tiles: input_str.chars().filter_map(|c|
                match c {
                    '\n' => None,
                    any => Some(T::parse_from_char(any))
                })
                .collect(),
        }
    }

    pub fn get_pos_in_dir(&self, pos: usize, dir: Direction) -> Option<usize> {
        match dir {
            NW => self.get_pos_diag(pos, NW),
            N => self.get_pos_above(pos),
            NE => self.get_pos_diag(pos, NE),
            W => self.get_pos_left(pos),
            E => self.get_pos_right(pos),
            SW => self.get_pos_diag(pos, SW),
            S => self.get_pos_below(pos),
            SE => self.get_pos_diag(pos, SE),
        }
    }

    pub fn get_pos_left(&self, pos: usize) -> Option<usize> {
        match pos / self.width == 0 {
            true => None,
            false => Some(pos - 1)
        }
    }

    pub fn get_pos_right(&self, pos: usize) -> Option<usize> {
        match pos % (self.width - 1) == 0 {
            true => None,
            false => Some(pos + 1)
        }
    }

    pub fn get_pos_above(&self, pos: usize) -> Option<usize> {
        match pos < self.width {
            true => None,
            false => Some(pos - self.width)
        }
    }

    pub fn get_pos_below(&self, pos: usize) -> Option<usize> {
        match pos < self.width * self.height - self.width {
            true => None,
            false => Some(pos + self.width)
        }
    }

    pub fn get_pos_diag(&self, pos: usize, dir: Direction) -> Option<usize> {
        match dir {
            NW => match pos < self.width || pos / self.width == 0 {
                true => None,
                false => Some(pos - self.width - 1)
            },
            NE => todo!(),
            SW => todo!(),
            SE => todo!(),
            _ => panic!("Not a valid diagonal")
        }
    }

    pub fn get_positions_around(&self, pos: usize) -> [Option<usize>; 8] {
        // [None, self.get_pos_above(pos), None,
        // self.get_pos_left(pos), self.get_pos_right(pos),
        // None, self.get_pos_below(pos), None]

        [NW, N, NE, W, E, SW, S, SE]
            .map(|dir| self.get_pos_in_dir(pos, dir))
            .into_iter()
            .collect::<Vec<Option<usize>>>()
            .try_into()
            .unwrap()
    }
}
