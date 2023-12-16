use std::fmt::{Debug, Formatter, Write};
use std::fs;
use itertools::Itertools;
use crate::day13::Tile::*;
use crate::map::{Map2D, MapElement, Pos};

#[derive(PartialEq)]
enum Tile {
    ASH,
    ROCK,
}

enum MirrorDirection {
    HORIZONTAL,
    VERTICAL,
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

    fn find_smudge_candidates_horizontal(&self) -> Option<usize> {
        let mut maybe_smudges: Vec<Pos> = vec![];

        for v in (0..self.height).permutations(2) {
            // activate that for perf, it should work
            // if usize::abs_diff(v[0], v[1]) % 2 == 0 {
            //     continue;
            // }

            let zip_two_rows = (0..self.width)
                        .map(|i| v[0] * self.width + i)
                        .zip((0..self.width).map(|i| v[1] * self.width + i));

            if zip_two_rows.filter(|(a, b)| self.get(*a) != self.get(*b)).count() == 1 {
                println!("Candidate: {} and {}", v[0], v[1]);
                maybe_smudges.push(v[0] * self.width) // + the idx of the row
            }
        }
        Some(42)
    }

    // fn find_smudge_candidates_vertical(&self) -> Option<usize> {
    //     for v in (0..self.width).permutations(2) {
    //         // activate that for perf, it should work
    //         // if usize::abs_diff(v[0], v[1]) % 2 == 0 {
    //         //     continue;
    //         // }
    //
    //         let zip_two_cols = (0..self.height)
    //             .map(|i| i * self.width + v[0])
    //             .zip((0..self.width).map(|i| i * self.width + v[1]));
    //
    //         if zip_two_cols.filter(|(a, b)| self.get(*a) != self.get(*b)).count() == 1 {
    //             println!("Candidate: {} and {}", v[0], v[1]);
    //         }
    //     }
    //     Some(42)
    // }

    fn get_val(&self) -> usize {
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
    let file_str = fs::read_to_string("../inputs/testday13").unwrap();
    let all_maps: Vec<Map2D<Tile>> = file_str.split("\n\n").map(Map2D::parse).collect();

    // dbg!(all_maps.first().unwrap().find_smudge_candidate_horizontal());
    dbg!(all_maps.last().unwrap().find_smudge_candidates_horizontal());
    // dbg!(all_maps.last().unwrap().find_smudge_candidates_vertical());

    // println!("Part 1: {}", all_maps.iter().map(Map2D::get_val).sum::<usize>());
    // println!("Part 2: {}", all_pairs.iter().map(SpringsAndCondition::unfold).map(|v| v.get_nbr_arrangements()).sum::<usize>());
}
