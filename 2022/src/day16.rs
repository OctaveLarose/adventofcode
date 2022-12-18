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

fn walk<'a>(cur_valve: Rc<RefCell<Valve<'a>>>, open_valves: &mut Vec<Rc<RefCell<Valve<'a>>>>, nbr_minutes: i32, mut best_output: usize) -> usize {
    if nbr_minutes <= 0 {
        return best_output;
    }

    for open_valve in open_valves.into_iter() {
        best_output += open_valve.borrow().flow_rate;
    }

    let mut outputs = vec![best_output];
    // either we move
    for dest in &cur_valve.borrow().tunnels_to {
        outputs.push(walk(dest.clone(), &mut open_valves.clone(), nbr_minutes - 1, best_output))
    }

    // or we open the valve (if it's openable)
    if cur_valve.borrow().flow_rate != 0 {
        if open_valves.iter().find(|&c| c.borrow().name == cur_valve.borrow().name).is_none() {
            open_valves.push(cur_valve.clone());
            outputs.push(walk(cur_valve.clone(), &mut open_valves.clone(), nbr_minutes - 1, best_output))
        }
    }

    *outputs.iter().max().unwrap()
}

fn part1(valves: &HashMap<&str, Rc<RefCell<Valve>>>) -> usize {
    let mut open_valves = vec![];

    let best_output = walk(valves["AA"].clone(), &mut open_valves, 30, 0);
    for v in open_valves {
        println!("{}", v.borrow().to_string())
    }
    best_output
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/testday16").unwrap();
    let valves = parse_scan(&file_str);

    // for v in &valves {
    //     println!("{}", v.1.borrow().to_string());
    // }

    println!("Day 16: ");
    println!("Part 1: {}", part1(&valves));
    // println!("Part 2: {}", Map::create_map_from_input(&file_lines).get_nbr_sand_resting_with_ground());
    println!("----------");
}