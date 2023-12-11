use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct MapFormat {
    dest: usize,
    source: usize,
    range: usize,
}

#[derive(Debug)]
struct Map {
    patterns: Vec<MapFormat>,
}

impl Map {
    // use the map to get an output from an input number
    pub fn map_nbr(&self, input_nbr: usize) -> usize {
        for pattern in &self.patterns {
            if pattern.source <= input_nbr && input_nbr <= pattern.source + pattern.range {
                return pattern.dest + (input_nbr - pattern.source);
            }
        }

        input_nbr // no match means return the number itself
    }

    // given an output of the existing map, return what the input was!
    pub fn reverse_map_nbr(&self, next_map_nbr: usize) -> usize {
        for pattern in &self.patterns {
            if pattern.dest <= next_map_nbr && next_map_nbr <= pattern.dest + pattern.range {
                return pattern.source + (next_map_nbr - pattern.dest);
            }
        }

        next_map_nbr // no match means return the number itself
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn part1_get_lowest_location(&self) -> usize {
        let mut seed_modifs = self.seeds.to_vec();

        for map in &self.maps {
            for idx in 0..seed_modifs.len() {
                seed_modifs[idx] = map.map_nbr(seed_modifs[idx]);
            }
        }

        *seed_modifs.iter().min().unwrap()
    }

    pub fn part2_get_lowest_location(&self) -> usize {
        const ARBITRARY_HIGH_NUMBER: usize = 94293573459;

        for i in 0..ARBITRARY_HIGH_NUMBER {
            let mut v = i;
            for map in self.maps.iter().rev() {
                v = map.reverse_map_nbr(v);
            }

            for seed_range in self.seeds.chunks(2) {
                if seed_range[0] <= v && v <= seed_range[0] + seed_range[1] {
                    return i
                }
            }
        }

        panic!("Uh oh. Should never have gotten this far");
    }
}

impl Almanac {
    pub fn parse(input: String) -> Almanac {
        let input_lines = input.split("\n\n").collect::<Vec<&str>>();
        let seeds_str = input_lines[0].split(':').nth(1).unwrap().split_whitespace();
        let map_lines = &input_lines[1..];

        Almanac {
            seeds: seeds_str
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            maps: {
                map_lines.iter()
                    .map(|map_line| {
                        let map_info = map_line.split('\n').skip(1).collect::<Vec<&str>>();

                        Map {
                            patterns: map_info.iter()
                                .map(|s| {
                                    let map_line_info: (usize, usize, usize) = s.split_whitespace()
                                        .map(|nbr| nbr.parse::<usize>().unwrap())
                                        .collect_tuple().unwrap();

                                    MapFormat { dest: map_line_info.0, source: map_line_info.1, range: map_line_info.2 }
                                })
                                .collect::<Vec<MapFormat>>()
                        }
                    })
                    .collect::<Vec<Map>>()
            },
        }
    }
}

pub fn run() {
    let almanac = Almanac::parse(fs::read_to_string("../inputs/day5").unwrap());

    println!("Day 5: ");
    println!("Part 1: {}", &almanac.part1_get_lowest_location());
    println!("Part 2: {}", &almanac.part2_get_lowest_location());
    println!("----------");
}
