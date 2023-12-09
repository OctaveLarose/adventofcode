use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

impl Race {
    pub fn get_nbr_winnable_ways(&self) -> usize {
        (0..self.time)
            .filter(|time_spent_waiting| {
                (self.time - time_spent_waiting) * time_spent_waiting > self.distance
            })
            .count()
    }
}

fn parse(races_file_str: String) -> Vec<Race> {
    let (time_str, distance_str) = races_file_str
        .lines()
        .map(|s| &s[s.find(':').unwrap() + 1..])
        .collect_tuple().unwrap();
    let mut time_distance_zip = time_str
        .split_whitespace()
        .zip(distance_str.split_whitespace());

    time_distance_zip.map(|(a, b)|
        Race {time: a.parse::<usize>().unwrap(), distance: b.parse::<usize>().unwrap()})
        .collect::<Vec<Race>>()
}

fn part1(races: &Vec<Race>) -> usize {
    races.iter().map(|r| r.get_nbr_winnable_ways()).collect::<Vec<usize>>().iter().product()
}

fn main() {
    let races = parse(fs::read_to_string("../inputs/day6").unwrap());

    // dbg!(&races);

    println!("Part 1: {}", part1(&races));
}
