use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::rc::Rc;
use crate::day7::NodeType::{DIR, FILE};

#[derive(PartialEq, Debug)]
enum NodeType {
    FILE,
    DIR
}

// #[derive(PartialEq)]
#[derive(Debug)]
struct TreeNode {
    pub node_type: NodeType,
    pub name: String,
    pub value: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new_dir(name: String) -> TreeNode {
        return TreeNode {
            node_type: DIR,
            name,
            value: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn new_file(name: String, size: u32) -> TreeNode {
        return TreeNode {
            node_type: FILE,
            name,
            value: Option::from(size),
            children: vec![],
            parent: None,
        };
    }

    // pub fn get_total_size(self) -> usize {
    //     match self.node_type {
    //         FILE => self.value.unwrap(),
    //         DIR => self.children.iter().map(|c| c.as_ref().get_total_size()).sum();
    //     }
    // }
}

fn build_tree(lines: Vec<String>) -> Rc<RefCell<TreeNode>> {
    let root_node = Rc::new(RefCell::new(TreeNode::new_dir(String::from("/"))));
    let mut current_node_ref = Rc::clone(&root_node);
    let mut lines_iter = lines.iter().skip(1).peekable(); // first line is "/" node, and we create it ahead of time

    while lines_iter.peek().is_some() {
        let l = lines_iter.next().unwrap();

        if l.starts_with("$ ls") {
            while lines_iter.peek().is_some() && !lines_iter.peek().unwrap().starts_with("$") {
                let l = lines_iter.next().unwrap();
                match &l[0..3] {
                    "dir" => {
                        current_node_ref.borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new_dir(String::from(&l[4..])))))
                    },
                    _ => {
                        let mut split = l.split_whitespace();
                        let size = split.next().unwrap().parse::<u32>().unwrap();
                        let name = split.next().unwrap();
                        current_node_ref.borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new_file(String::from(name), size))));
                    }
                }
                dbg!(l);
            }
        }
        if l.starts_with("$ cd") {
            if &l[5..7] == ".." {
                let parent = current_node_ref.borrow().parent.as_ref().unwrap().clone();
                current_node_ref = parent;
            } else {
                let children = &current_node_ref.borrow().children;
                for child_ref in children {
                    let child = child_ref.borrow();
                    if child.node_type == DIR && child.name.eq(&l[4..]) { // may be 3 TODO
                        current_node_ref = Rc::clone(&child_ref);
                    };
                };
            }
        }
        lines_iter.next();
    }

    root_node
}

pub fn run() {
    let file = File::open("inputs/testday7").unwrap();
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let tree = build_tree(lines);

    dbg!(tree);
    println!("Day 5: ");
    // println!("Part 1: {}", (&buffer, 4));
    println!("----------");
}