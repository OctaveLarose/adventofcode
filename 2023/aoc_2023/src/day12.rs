use std::fs;
use std::process::exit;
use itertools::Itertools;
use crate::day12::Spring::*;

#[derive(Debug, PartialEq)]
enum Spring {
    DOT,
    BROKEN,
    UNKNOWN,
}

impl Spring {
    pub fn from_char(c: char) -> Spring {
        match c {
            '.' => DOT,
            '#' => BROKEN,
            '?' => UNKNOWN,
            _ => panic!("Invalid char for spring")
        }
    }
}

#[derive(Debug)]
struct Springs(Vec<Spring>);

impl Springs {
    fn get_sizes(&self) -> SpringSizes {
        let mut cur_acc = 0;
        let mut sizes = SpringSizes(vec![]);

        for spr in &self.0 {
            match spr {
                BROKEN => cur_acc += 1,
                DOT | UNKNOWN => {
                    if cur_acc != 0 {
                        sizes.0.push(cur_acc);
                    }
                    cur_acc = 0;
                }
            }
        }

        if cur_acc != 0 {
            sizes.0.push(cur_acc);
        }

        sizes
    }
}

#[derive(Debug)]
struct SpringSizes(Vec<usize>);

impl PartialEq<Self> for SpringSizes {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b) | *a == *b)
    }
}

impl Eq for SpringSizes {}

#[derive(Debug)]
struct SpringsAndCondition {
    springs: Springs,
    condition: SpringSizes,
}

impl SpringsAndCondition {
    fn from_str(str: &str) -> SpringsAndCondition {
        let (springs_str, condition_str): (&str, &str) = str.split_whitespace().collect_tuple().unwrap();

        SpringsAndCondition {
            springs: Springs(springs_str.chars().map(Spring::from_char).collect()),
            condition: SpringSizes(condition_str.split(',').map(|c| c.parse::<usize>().unwrap()).collect()),
        }
    }

    fn get_nbr_arrangements(&self) -> usize {
        let indices_unknowns: Vec<usize> = self.springs.0.iter().enumerate()
            .filter_map(|(idx, s)| (*s == BROKEN).then(|| idx))
            .collect();
        let all_possible_combinations = indices_unknowns.iter().cartesian_product(&[BROKEN, DOT]);

        all_possible_combinations.map(|(idx, val)|)
        // for l in all_possible_combinations {
        //     dbg!(l);
        // }



        dbg!(&indices_unknowns);
        exit(1);

        42 //todo.
    }
}

pub fn run() {
    let file_str = fs::read_to_string("../inputs/testday12").unwrap();
    let all_pairs: Vec<SpringsAndCondition> = file_str.lines()
        .map(SpringsAndCondition::from_str)
        .collect();

    println!("Part 1: {}", all_pairs.iter().map(SpringsAndCondition::get_nbr_arrangements).sum::<usize>());
    // println!("Part 2: {}", part2(&galaxies, uni_width));
}
