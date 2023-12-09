use std::cmp::min;
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
    pub fn map_nbr(&self, input_nbr: usize) -> usize {
        for pattern in &self.patterns {
            if pattern.source <= input_nbr && input_nbr <= pattern.source + pattern.range {
                return pattern.dest + (input_nbr - pattern.source);
            }
        }

        input_nbr // no match means return the number itself
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
        let mut seed_ranges = self.seeds.chunks(2);
        let mut global_min = usize::MAX;

        for seed_range in seed_ranges {
            let mut local_min = usize::MAX;

            for i in 0..seed_range[1] {
                let mut val = seed_range[0] + i;
                for map in &self.maps {
                    val = map.map_nbr(val);
                }
                local_min = min(local_min, val);
            }

            global_min = min(global_min, local_min);
        }

        global_min
        // *seed_modifs.iter().min().unwrap()
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

fn main() {
    let almanac = Almanac::parse(fs::read_to_string("../inputs/day5").unwrap());

    println!("Part 1: {}", &almanac.part1_get_lowest_location());
    println!("Part 2: {}", &almanac.part2_get_lowest_location());
}
