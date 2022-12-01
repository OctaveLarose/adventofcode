use std::fs::File;
use std::usize;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

fn get_elves_calories(lines: Lines<BufReader<File>>) -> Vec<usize> {
    let mut elves: Vec<usize> = Vec::new();
    let mut current_elf_sum = 0;

    for line_res in lines {
        let line = line_res.unwrap();
        match line.is_empty() {
            true => {
                elves.push(current_elf_sum);
                current_elf_sum = 0;
            }
            false => {
                current_elf_sum += usize::from_str(&*line.trim()).unwrap();
            }
        }
    }
    elves
}

fn part1(elves_calories: &Vec<usize>) -> usize {
    *elves_calories.iter().max().unwrap()
}

fn part2(elves_calories: &mut Vec<usize>) -> usize {
    let mut max_calories_sum = 0;
    const NBR_ELVES: i32 = 3;

    for _ in 0..NBR_ELVES {
        let index_of_max: Option<usize> = elves_calories
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index);

        max_calories_sum += elves_calories.get(index_of_max.unwrap()).unwrap();
        elves_calories.remove(index_of_max.unwrap());
    }

    max_calories_sum
}

pub fn run() {
    let file = File::open("inputs/day1").unwrap();
    let lines = BufReader::new(file).lines();
    let mut elves = get_elves_calories(lines);

    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&mut elves));
}