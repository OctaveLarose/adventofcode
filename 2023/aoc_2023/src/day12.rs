use std::fmt::{Debug, Formatter};
use std::fs;
use std::slice::Iter;
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
    fn replace_unknowns_and_check_condition(&self, mut replacements: Iter<Spring>, mut condition_iter: Iter<usize>) -> bool {
        let mut cur_acc = 0;

        for spr in &self.0 {
            let match_on = match *spr == UNKNOWN {
                true => replacements.next().unwrap(),
                false => spr
            };

            match match_on {
                BROKEN => cur_acc += 1,
                DOT => {
                    if cur_acc != 0 {
                        match condition_iter.next() {
                            None => return false,
                            Some(v) => {
                                if *v != cur_acc {
                                    return false;
                                }
                            }
                        }
                    }
                    cur_acc = 0;
                }
                UNKNOWN => panic!("Replacements should not contain an unknown.")
            }
        }

        if cur_acc != 0 {
            match condition_iter.next() {
                None => return false,
                Some(v) => {
                    if *v != cur_acc {
                        return false;
                    }
                }
            }
        }

        if condition_iter.next().is_some() {
            return false;
        }

        true
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
            self.springs.replace_unknowns_and_check_condition(spring_arrangement.iter(), self.condition.0.iter())
        }).count()
    }

    fn unfold(&self, n: usize) -> SpringsAndCondition {
        SpringsAndCondition {
            springs: {
                Springs(std::iter::repeat(&self.springs)
                    .take(n - 1) // N - 1 copies with "?" at the end
                    .flat_map(|v| v.0.clone().into_iter().chain(std::iter::once(UNKNOWN)))
                    .chain(self.springs.0.clone()) // ...and one without "?" at the end.
                    .collect())
            },
            condition: SpringSizes(self.condition.0.repeat(n)),
        }
    }
}

fn part2(all_pairs: &Vec<SpringsAndCondition>) -> usize {
    let mut sum_arrangements = 0;

    for pair in all_pairs {
        let new_pair_end = SpringsAndCondition {
            springs: {
                let mut new_springs = Springs(pair.springs.0.clone());
                new_springs.0.push(UNKNOWN);
                new_springs
            },
            condition: SpringSizes(pair.condition.0.clone()),
        };

        let new_pair_start = SpringsAndCondition {
            springs: {
                let mut new_springs = Springs(pair.springs.0.clone());
                new_springs.0.insert(0, UNKNOWN);
                new_springs
            },
            condition: SpringSizes(pair.condition.0.clone()),
        };

        let og_val = pair.get_nbr_arrangements();
        let (start_val, end_val) = (new_pair_start.get_nbr_arrangements(), new_pair_end.get_nbr_arrangements());
        // dbg!(og_val);
        // dbg!(start_val);
        // dbg!(end_val);
        // dbg!();
        let val = match start_val > end_val {
            true => start_val.pow(4) * og_val,
            false => end_val.pow(4) * og_val
        };

        dbg!(&val);
        dbg!();

        if pair.unfold(2).get_nbr_arrangements() == 1 {
            sum_arrangements += 1;
        } else {
            sum_arrangements += val;
        }
    }

    sum_arrangements
}

pub fn run() {
    let file_str = fs::read_to_string("../inputs/day12").unwrap();
    let all_pairs: Vec<SpringsAndCondition> = file_str.lines()
        .map(SpringsAndCondition::from_str)
        .collect();

    println!("Part 1: {}", all_pairs.iter().map(SpringsAndCondition::get_nbr_arrangements).sum::<usize>());
    println!("Part 2: {}", part2(&all_pairs));
}
