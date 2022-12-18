use std::{fs};
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;
use itertools::Itertools;

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: usize,
    tunnels_to: Vec<Rc<RefCell<Valve<'a>>>>
}

impl ToString for Valve<'_> {
    fn to_string(&self) -> String {
        format!("{}, flow_rate: {}, connected to: {}", self.name, self.flow_rate, self.tunnels_to.iter().map(|l| l.borrow().name).join(", "))
    }
}

fn parse_scan<'a>(input: &'a String) -> HashMap<&'a str, Rc<RefCell<Valve<'a>>>> {
    let mut valves: HashMap<&'a str, Rc<RefCell<Valve>>> = HashMap::new();

    for l in input.lines() {
        valves.insert(&l[6..8], Rc::new(RefCell::from(Valve {
            name: &l[6..8],
            flow_rate: *&l[l.find("=").unwrap() + 1..l.find(";").unwrap()].parse::<usize>().unwrap(),
            tunnels_to: vec![],
        })));
    }

    for l in input.lines() {
        let cur_valve = valves.get_mut(&l[6..8]).unwrap().clone();

        let tunnels_str = match l.find("valves") {
            None => {&l[l.find("valve").unwrap() + 6..]} // sometimes it's only connected to one
            Some(idx) => {&l[idx + 7..]}
        };

        cur_valve.borrow_mut().tunnels_to = tunnels_str.split(", ")
            .map(|name| valves.get(name).unwrap().clone())
            .collect::<Vec<Rc<RefCell<Valve>>>>();
    }

    valves
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/testday16").unwrap();
    let valves = parse_scan(&file_str);

    for v in &valves {
        println!("{}", v.1.borrow().to_string());
    }

    println!("Day 16: ");
    // println!("Part 1: {}", get_nbr_positions_with_no_beacon(&sensor_beacon_pairs, 2000000));
    // println!("Part 2: {}", Map::create_map_from_input(&file_lines).get_nbr_sand_resting_with_ground());
    println!("----------");
}