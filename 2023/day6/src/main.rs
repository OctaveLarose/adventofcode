use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

impl Race {
    // you would think this would be too slow and you'd need to start looking for extra tricks,
    // like stopping evaluation once the curve goes back down. but that's not even needed.
    // part 2 is near instantaneous even with this naive function. so i'll take it, i guess! thanks, rust.
    pub fn get_nbr_winnable_ways(&self) -> usize {
        (0..self.time)
            .filter(|time_spent_waiting| {
                (self.time - time_spent_waiting) * time_spent_waiting > self.distance
            })
            .count()
    }
}

fn parse_part1(races_file_str: &String) -> Vec<Race> {
    let (time_str, distance_str) = races_file_str
        .lines()
        .map(|s| &s[s.find(':').unwrap() + 1..])
        .collect_tuple().unwrap();

    time_str
        .split_whitespace()
        .zip(distance_str.split_whitespace())
        .map(|(a, b)| Race {time: a.parse::<usize>().unwrap(), distance: b.parse::<usize>().unwrap()})
        .collect::<Vec<Race>>()
}

fn parse_part2(races_file_str: &String) -> Race {
    let (time_str, distance_str) = races_file_str
        .lines()
        .map(|s| &s[s.find(':').unwrap() + 1..])
        .collect_tuple().unwrap();

    Race {
        time: time_str.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap(),
        distance: distance_str.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap()
    }
}

fn part1(races: &Vec<Race>) -> usize {
    races.iter().map(|r| r.get_nbr_winnable_ways()).collect::<Vec<usize>>().iter().product()
}

fn main() {
    let input_file = fs::read_to_string("../inputs/day6").unwrap();
    let races = parse_part1(&input_file);
    let race_part2 = parse_part2(&input_file);

    println!("Part 1: {}", part1(&races));
    println!("Part 2: {}", race_part2.get_nbr_winnable_ways());
}
