use std::fmt::{Debug, Formatter};
use std::fs;
use itertools::{Itertools};
use crate::day12::Spring::*;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn to_char(&self) -> char {
        match self {
            DOT => '.',
            BROKEN => '#',
            UNKNOWN => '?',
        }
    }
}

struct Springs(Vec<Spring>);

impl Debug for Springs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.0.iter().map(Spring::to_char).join(""))
    }
}

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

    pub fn replace_unknowns(&self, unknowns_map: Vec<(&usize, &Spring)>) -> Springs {
        Springs(
            self.0.iter()
                .enumerate()
                .map(|(idx, spring)| {
                    match spring {
                        DOT => DOT,
                        BROKEN => BROKEN,
                        UNKNOWN => match &unknowns_map.iter().find(|(idx2, v)| idx == **idx2).unwrap().1 {
                            DOT => DOT,
                            BROKEN => BROKEN,
                            UNKNOWN => panic!("Should be impossible")
                        }
                    }
                })
                .collect::<Vec<Spring>>()
        )
    }
}

struct SpringSizes(Vec<usize>);

impl Debug for SpringSizes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.0.iter()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join(" "))
    }
}

impl PartialEq<Self> for SpringSizes {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().zip(other.0.iter()).all(|(a, b)| *a == *b)
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
            .filter_map(|(idx, s)| (*s == UNKNOWN).then(|| idx))
            .collect();
        let all_possible_combinations = (0..indices_unknowns.len()).map(|_| vec![BROKEN, DOT]).multi_cartesian_product();

        all_possible_combinations.filter(|spring_arrangement| {
            let possible_arrangement_desc: Vec<(&usize, &Spring)> = indices_unknowns.iter().zip(spring_arrangement).collect();
            self.springs.replace_unknowns(possible_arrangement_desc).get_sizes().eq(&self.condition)
        }).count()
    }

    fn unfold(&self) -> SpringsAndCondition {
        SpringsAndCondition {
            springs: {
                Springs(std::iter::repeat(&self.springs)
                    .take(4) // four copies with "?" at the end
                    .flat_map(|v| v.0.clone().into_iter().chain(std::iter::once(UNKNOWN)))
                    .chain(self.springs.0.clone()) // ...and one without "?" at the end.
                    .collect())
            },
            condition: SpringSizes(self.condition.0.repeat(5)),
        }
    }
}

pub fn run() {
    let file_str = fs::read_to_string("../inputs/testday12").unwrap();
    let all_pairs: Vec<SpringsAndCondition> = file_str.lines()
        .map(SpringsAndCondition::from_str)
        .collect();

    println!("Part 1: {}", all_pairs.iter().map(SpringsAndCondition::get_nbr_arrangements).sum::<usize>());
    println!("Part 2: {}", all_pairs.iter().map(SpringsAndCondition::unfold).map(|v| v.get_nbr_arrangements()).sum::<usize>());
}
