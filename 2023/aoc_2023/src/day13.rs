use std::fmt::{Debug, Formatter, Write};
use std::fs;
use crate::day13::Tile::*;
use crate::map::{Map2D, MapElement};

#[derive(PartialEq)]
enum Tile {
    ASH,
    ROCK,
}

enum MirrorDirection {
    HORIZONTAL,
    VERTICAL
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASH => f.write_char('.'),
            ROCK => f.write_char('#')
        }
    }
}

impl MapElement for Tile {
    fn parse_from_char(c: char) -> Self {
        match c {
            '.' => ASH,
            '#' => ROCK,
            _ => panic!("Invalid pattern")
        }
    }

    fn to_char(&self) -> char {
        match self {
            ASH => '.',
            ROCK => '#'
        }
    }
}

impl Map2D<Tile> {
    fn are_columns_identical(&self, idx1: usize, idx2: usize) -> bool {
        (0..self.height).all(|v| *self.get(idx1 + v * self.width) == *self.get(idx2 + v * self.width))
    }

    fn are_rows_identical(&self, idx1: usize, idx2: usize) -> bool {
        (0..self.width).all(|v| *self.get(idx1 * self.width + v) == *self.get(idx2 * self.width + v))
    }

    fn find_reflection(&self, dir: MirrorDirection) -> Option<usize> {
        let (max, are_col_or_rows_equiv) = match dir {
            MirrorDirection::HORIZONTAL => (self.height, Map2D::are_rows_identical as fn(&Map2D<Tile>, usize, usize) -> bool),
            MirrorDirection::VERTICAL => (self.width, Map2D::are_columns_identical as fn(&Map2D<Tile>, usize, usize) -> bool)
        };

        for (chunk_idx, _) in self.tiles[0..max].windows(2).enumerate() {
            if !are_col_or_rows_equiv(self, chunk_idx, chunk_idx + 1) {
                continue;
            }

            let left_col_iter = (0..chunk_idx).rev();
            let right_col_iter = (chunk_idx + 2..).take_while(|&idx| idx < max);

            if left_col_iter
                .zip(right_col_iter)
                .all(|(left_idx, right_idx)| are_col_or_rows_equiv(self, left_idx, right_idx))
            {
                return Some(chunk_idx);
            }
        }
        None
    }

    fn get_part1_val(&self) -> usize {
        if let Some(horizontal_idx) = self.find_reflection(MirrorDirection::HORIZONTAL) {
            (horizontal_idx + 1) * 100
        } else if let Some(vertical_idx) = self.find_reflection(MirrorDirection::VERTICAL) {
            vertical_idx + 1
        } else {
            panic!("No reflection found??");
        }
    }
}

pub fn run() {
    let file_str = fs::read_to_string("../inputs/day13").unwrap();
    let all_maps: Vec<Map2D<Tile>> = file_str.split("\n\n").map(Map2D::parse).collect();

    // dbg!(&all_maps);
    // dbg!(all_maps.first().unwrap().find_vertical_reflection());
    // dbg!(all_maps.last().unwrap().find_horizontal_reflection());

    println!("Part 1: {}", all_maps.iter().map(Map2D::get_part1_val).sum::<usize>());
    // println!("Part 2: {}", all_pairs.iter().map(SpringsAndCondition::unfold).map(|v| v.get_nbr_arrangements()).sum::<usize>());
}
