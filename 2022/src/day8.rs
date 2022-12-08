use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

#[derive(Debug)]
struct TreeMap {
    length: usize,
    trees: Vec<u8>
}

impl TreeMap {
    pub fn is_on_edge(&self, tree_idx: usize) -> bool {
        if tree_idx < self.length {
            return true;
        } else if tree_idx >= self.length * (self.length - 1) {
            return true;
        } else if (tree_idx + 1) % self.length == 0 {
            return true;
        } else if tree_idx % self.length == 0 {
            return true;
        }
        false
    }

    pub fn is_tree_visible(&self, tree_idx: usize) -> bool {
        if self.is_on_edge(tree_idx) {
            return true
        }

        let our_tree_height = self.trees.get(tree_idx).unwrap();

        let trees_top: Vec<usize> = (0..self.trees.len() - 1)
            .filter_map(|idx| if (tree_idx % self.length == idx % self.length) && (idx < tree_idx) { Some(idx) } else { None })
            .collect();
        let trees_bottom: Vec<usize> = (0..self.trees.len() - 1)
            .filter_map(|idx| if (tree_idx % self.length == idx % self.length) && (idx > tree_idx) { Some(idx) } else { None })
            .collect();
        let trees_left: Vec<usize> = (0..self.trees.len() - 1)
            .filter_map(|idx| if (tree_idx / self.length == idx / self.length) && (idx < tree_idx) { Some(idx) } else { None })
            .collect();
        let trees_right: Vec<usize> = (0..self.trees.len() - 1)
            .filter_map(|idx| if (tree_idx / self.length == idx / self.length) && (idx > tree_idx) { Some(idx) } else { None })
            .collect();

        for dir in [trees_top, trees_bottom, trees_right, trees_left] {
            if !dir.iter().any(|idx| self.trees.get(*idx).unwrap() >= our_tree_height) {
                return true
            }
        }
        false
    }
}

fn get_number_of_visible_trees(tree_map: &TreeMap) -> usize {
    (0..(tree_map.length * tree_map.length))
        .map(|idx| if tree_map.is_tree_visible(idx) { 1 } else { 0 })
        .sum()
}

pub fn run() {
    let file = File::open("inputs/day8").unwrap();
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let tree_map = TreeMap {
        length: lines.iter().next().unwrap().len(),
        trees: lines.join("").chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>()
    };

    println!("Day 8: ");
    println!("Part 1: {}", get_number_of_visible_trees(&tree_map));
    // println!("Part 2: {}", tree.borrow().find_smallest_dir_to_free(70000000, 30000000));
    println!("----------");
}