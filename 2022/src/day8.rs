use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct TreeMap {
    length: usize,
    trees: Vec<u8>
}

impl TreeMap {
    pub fn get_neighbors_heights(&self, tree_idx: usize) -> [Option<&u8>; 4] {
        let top = if tree_idx >= self.length { self.trees.get(tree_idx - self.length) } else { None };
        let bottom = if tree_idx + self.length < self.length * self.length { self.trees.get(tree_idx + self.length) } else { None };
        let left = if ((tree_idx - 1) % self.length) == 0 { None } else { self.trees.get(tree_idx - 1) };
        let right = if ((tree_idx + 1) % self.length) == 0 { None } else { self.trees.get(tree_idx + 1) };
        [top, right, bottom, left]
    }

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

        let tree_height = self.trees.get(tree_idx).unwrap();

        for neighbor in self.get_neighbors_heights(tree_idx) {
            match neighbor {
                None => return true,
                Some(neighbor_height) => {
                    if neighbor_height < tree_height {
                        return true;
                    }
                }
            }
        }
        dbg!(tree_idx);
        false
    }
}

fn get_number_of_visible_trees(tree_map: &TreeMap) -> usize {
    let mut nbr_visible_trees = 0;

    for idx in 0..(tree_map.length * tree_map.length) {
        if tree_map.is_tree_visible(idx) {
            // dbg!(idx);
            nbr_visible_trees += 1;
        }
    }
    nbr_visible_trees
}

pub fn run() {
    let file = File::open("inputs/testday8").unwrap();
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