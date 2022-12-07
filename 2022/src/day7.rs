use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use crate::day7::NodeType::{DIR, FILE};

#[derive(PartialEq, Debug)]
enum NodeType {
    FILE,
    DIR
}

#[derive(Debug)]
struct TreeNode {
    pub node_type: NodeType,
    pub name: String,
    pub value: Option<usize>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new_dir(name: String, parent: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        let tree_node = TreeNode {
            node_type: DIR,
            name,
            value: None,
            children: vec![],
            parent,
        };
        Rc::new(RefCell::new(tree_node))
    }

    pub fn new_file(name: String, size: usize, parent: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let tree_node = TreeNode {
            node_type: FILE,
            name,
            value: Option::from(size),
            children: vec![],
            parent: Option::from(parent),
        };
        Rc::new(RefCell::new(tree_node))
    }

    pub fn print(&self) -> String {
        if let Some(value) = self.value {
            return format!("({}-{})", self.name, value.to_string());
        } else {
            return String::from(format!("{}[", self.name))
                + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
                + "]";
        }
    }

    pub fn get_size(&self) -> usize {
        match self.node_type {
            FILE => self.value.unwrap(),
            DIR => {
                self.children.iter().map(|c| c.borrow().get_size()).sum()
            }
        }
    }

    pub fn get_sizes_under_threshold(&self, threshold: usize) -> Vec<usize> {
        let mut correct_sizes = vec![];

        let own_size = self.get_size();
        if own_size <= threshold {
            correct_sizes.push(own_size);
        }

        for c in &self.children {
            if c.borrow().node_type == DIR {
                let mut children_sizes = c.borrow().get_sizes_under_threshold(threshold);
                correct_sizes.append(&mut children_sizes);
            }
        }

        correct_sizes
    }

    pub fn get_total_size_under_threshold(&self, threshold: usize) -> usize {
        return self.get_sizes_under_threshold(threshold).iter().sum();
    }
}

fn build_tree(lines: Vec<String>) -> Rc<RefCell<TreeNode>> {
    let root_node = TreeNode::new_dir(String::from("/"), None);
    let mut current_node = Rc::clone(&root_node);
    let mut lines_iter = lines.iter().skip(1).peekable(); // first line is "/" node, and we create it ahead of time

    while lines_iter.peek().is_some() {
        let l = lines_iter.next().unwrap();

        match &l[0..4] {
            "$ ls" => {
                while lines_iter.peek().is_some() && !lines_iter.peek().unwrap().starts_with("$") {
                    let l = lines_iter.next().unwrap();
                    match &l[0..3] {
                        "dir" => {
                            current_node.borrow_mut().children.push(
                                TreeNode::new_dir(String::from(&l[4..]), Some(current_node.clone()))
                            )
                        },
                        _ => {
                            let mut split = l.split_whitespace();
                            let size = split.next().unwrap().parse::<usize>().unwrap();
                            let name = split.next().unwrap();
                            current_node.borrow_mut().children.push(
                                TreeNode::new_file(String::from(name), size, current_node.clone())
                            );
                        }
                    }
                }
            },
            "$ cd" => {
                if l.len() == 7 && &l[5..7] == ".." {
                    let parent = current_node.borrow().parent.as_ref().unwrap().clone();
                    current_node = parent;
                } else {
                    let current_node_clone = Rc::clone(&current_node);
                    for child_ref in &current_node_clone.borrow().children {
                        let child = child_ref.borrow();
                        if child.node_type == DIR && child.name.eq(&l[5..]) {
                            current_node = Rc::clone(&child_ref);
                            break;
                        };
                    };
                }
            }
            _ => panic!("Invalid line: {}", l)
        }
    }

    root_node
}

pub fn run() {
    let file = File::open("inputs/day7").unwrap();
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let tree = build_tree(lines);

    // println!("{}", tree.borrow().print());

    println!("Day 5: ");
    println!("Part 1: {}", tree.borrow().get_total_size_under_threshold(100000));
    println!("----------");
}