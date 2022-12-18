use std::{fs};
use std::collections::HashSet;
use std::ops::Range;
use itertools::Itertools;

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: Pos,
    beacon: Pos
}

impl SensorBeacon {
    fn from_string(str: &str) -> SensorBeacon {
        let sensor_coords_str = &str[str.find("x").unwrap()..str.find(":").unwrap()];
        let beacon_coords_str = &str[str.find("is at ").unwrap() + 6..str.len()];
        let str_to_coords = |coords_str: &str | coords_str
            .split(", ")
            .map(|s| { s.split("=").nth(1).unwrap().parse::<isize>().unwrap() })
            .collect_tuple::<(isize, isize)>().unwrap();

        let sensor_coords = str_to_coords(sensor_coords_str);
        let beacon_coords = str_to_coords(beacon_coords_str);

        SensorBeacon{ sensor: Pos { x: sensor_coords.0, y: sensor_coords.1 }, beacon: Pos { x: beacon_coords.0, y: beacon_coords.1 } }
    }

}

fn get_nbr_positions_with_no_beacon(pairs: &Vec<SensorBeacon>, line_nbr: isize) -> usize {
    let mut positions: HashSet<isize> = HashSet::new();

    for pair in pairs {
        let distance = (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs();

        if (pair.sensor.y > line_nbr && pair.sensor.y - distance < line_nbr) ||
            (pair.sensor.y < line_nbr && pair.sensor.y + distance > line_nbr) {
            let lol = distance - (line_nbr - pair.sensor.y).abs();

            let r = Range {start: pair.sensor.x - lol, end: pair.sensor.x + lol + 1};

            for c in r {
                positions.insert(c);
            }
        }
    }

    for pair in pairs {
        if pair.beacon.y == line_nbr {
            positions.remove(&pair.beacon.x);
        }
    }

    positions.len()
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/day15").unwrap();
    let sensor_beacon_pairs = file_str.lines()
        .map(|str| SensorBeacon::from_string(str))
        .collect::<Vec<SensorBeacon>>();

    println!("Day 15: ");
    println!("Part 1: {}", get_nbr_positions_with_no_beacon(&sensor_beacon_pairs, 2000000));
    // println!("Part 2: {}", Map::create_map_from_input(&file_lines).get_nbr_sand_resting_with_ground());
    println!("----------");
}