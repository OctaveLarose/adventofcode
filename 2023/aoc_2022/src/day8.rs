use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::rc::Rc;
use crate::day8::Dir::*;
use circular_vec::CircularVec;

#[derive(Debug)]
enum Dir {
    LEFT,
    RIGHT
}

struct Node {
    pub name: [char; 3],
    pub is_end: bool,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn init(name: &str) -> Node {
        Node {
            name: name.chars().collect::<Vec<char>>().try_into().unwrap(),
            is_end: name.eq("ZZZ"),
            left: None,
            right: None,
        }
    }

    pub fn link_with(&mut self, left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
        self.left = Some(left);
        self.right = Some(right);
    }

    // there's definitely an easier way of doing this, and to avoid declaring this function in the first place.
    fn compare_name(&self, str: &str) -> bool {
        str[0..3].chars().zip(&self.name).all(|(a, b) | a == *b)
    }

    pub fn get_left(&self) -> Rc<RefCell<Node>> {
        self.left.as_ref().unwrap().clone()
    }

    pub fn get_right(&self) -> Rc<RefCell<Node>> {
        self.right.as_ref().unwrap().clone()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Node {:?}, linked with {:?} and {:?}",
                                 self.name,
                                 self.get_left().as_ref().borrow().name,
                                 self.get_right().as_ref().borrow().name))
    }
}

fn part1(nodes: &Vec<Rc<RefCell<Node>>>, directions: &Vec<Dir>) {
    let mut nbr_steps = 0;
    let mut cur_node = nodes.iter().find(|n| n.borrow().compare_name("AAA")).unwrap().clone();
    let mut dir_circ: CircularVec<&Dir> = CircularVec::from_iter(directions.iter());

    while !cur_node.borrow().is_end {
        cur_node = match dir_circ.next() {
            LEFT => cur_node.borrow().get_left().clone(),
            RIGHT => cur_node.borrow().get_right().clone(),
        };

        nbr_steps += 1;
    }
    println!("Part 1: {}", nbr_steps);
}

pub fn run() {
    let input_file_str = fs::read_to_string("../inputs/day8").unwrap();
    let directions: Vec<Dir> = input_file_str.lines().nth(0).unwrap().chars().map(|c|
        match c {
            'L' => LEFT,
            'R' => RIGHT,
            _ => panic!("Invalid direction.")
    }).collect();
    let all_nodes_str: Vec<(&str, &str, &str)> = input_file_str.lines().skip(2)
        .collect::<Vec<&str>>().iter()
        .map(|s| (&s[0..3], &s[7..10], &s[12..15]))
        .collect();
    let mut nodes: Vec<Rc<RefCell<Node>>> = all_nodes_str.iter().map(|(a, _, _)| Rc::new(RefCell::new(Node::init(a)))).collect();

    for (name, left_name, right_name) in &all_nodes_str {
        let mut cur_node = nodes.iter().find(|n| n.borrow().compare_name(*name)).unwrap();
        let left_node = Rc::clone(nodes.iter().find(|n| n.borrow().compare_name(left_name)).unwrap());
        let right_node = Rc::clone(nodes.iter().find(|n| n.borrow().compare_name(right_name)).unwrap());
        cur_node.borrow_mut().link_with(left_node, right_node);
    }

    part1(&nodes, &directions);
}
