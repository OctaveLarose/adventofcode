use std::fmt::{Debug, Display, Formatter, Write};
use num_integer::Integer;
use crate::map::Direction::*;

pub type Pos = usize;

pub enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

pub struct Map2D<T> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) tiles: Vec<T>,
}

impl<T: MapElement> Debug for Map2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: MapElement> Display for Map2D<T> {
    // There's probably a prettier way of doing this, but good enough for now.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;

        while i < (self.width * self.height) {
            f.write_char(self.tiles.get(i).unwrap().to_char()).expect("write failed");

            if (i + 1) % self.width == 0 {
                write!(f, "\n").expect("write failed");
            }

            i += 1;
        }

        Ok(())
    }
}

pub trait MapElement {
    fn parse_from_char(c: char) -> Self;
    fn to_char(&self) -> char; // Used for printing out the map
}

#[derive(Debug)]
pub struct CharMapElement(pub char);

impl MapElement for CharMapElement {
    fn parse_from_char(c: char) -> CharMapElement {
        CharMapElement(c)
    }

    fn to_char(&self) -> char {
        self.0
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

    pub fn get(&self, pos: Pos) -> &T {
        self.tiles.get(pos).unwrap()
    }

    pub fn get_in_dir(&self, pos: Pos, dir: Direction) -> Option<&T> {
        self.get_pos_in_dir(pos, dir).and_then(|pos_in_dir| Some(self.tiles.get(pos_in_dir).unwrap()))
    }

    pub fn get_pos_in_dir(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        match dir {
            NW => (!self.is_on_top_edge(pos) && !self.is_on_left_edge(pos)).then(|| pos - self.width - 1),
            N => (!self.is_on_top_edge(pos)).then(|| pos - self.width),
            NE => (!self.is_on_top_edge(pos) && !self.is_on_right_edge(pos)).then(|| pos - self.width + 1),
            W => (!self.is_on_left_edge(pos)).then(|| pos - 1),
            E => (!self.is_on_right_edge(pos)).then(|| pos + 1),
            SW => (!self.is_on_bottom_edge(pos) && !self.is_on_left_edge(pos)).then(|| pos + self.width - 1),
            S => (!self.is_on_bottom_edge(pos)).then(|| pos + self.width),
            SE => (!self.is_on_bottom_edge(pos) && !self.is_on_right_edge(pos)).then(|| pos + self.width + 1),
        }
    }

    pub fn get_positions_around(&self, pos: Pos) -> [Option<Pos>; 8] {
        [NW, N, NE, W, E, SW, S, SE]
            .map(|dir| self.get_pos_in_dir(pos, dir))
            .into_iter()
            .collect::<Vec<Option<Pos>>>()
            .try_into()
            .unwrap()
    }

    fn is_on_left_edge(&self, pos: Pos) -> bool {
        pos == 0 || self.width.divides(&pos)
    }

    fn is_on_right_edge(&self, pos: Pos) -> bool {
        (pos + 1) % self.width == 0
    }

    fn is_on_top_edge(&self, pos: Pos) -> bool {
        pos < self.width
    }

    fn is_on_bottom_edge(&self, pos: Pos) -> bool {
        pos > self.width * self.height - self.width
    }
}
