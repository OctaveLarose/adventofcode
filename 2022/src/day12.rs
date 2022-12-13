use std::fs;
use std::process::exit;

// #[derive(Debug)]
// struct Pos {
//     x: usize,
//     y: usize
// }

#[derive(Debug)]
struct Map {
    length: usize,
    height: usize,
    map: Vec<u8>,
    player_idx: usize,

    paths_found: Vec<usize>,
    cur_nbr_steps: usize
}

impl Map {
    // fn idx_to_pos(&self, idx: usize) -> Pos {
    //     Pos { x: idx % self.len, y: idx % self.height }
    // }
    fn get_valid_destinations_for_player(&self, current_height: u8, prev_pos: usize) -> Vec<usize> {
        let mut valid_destinations = vec![];
        let valid_height = current_height + 1;

        if self.player_idx == 23 {
            print!("bp");
        }

        // left
        if self.player_idx % self.length != 0 {
            valid_destinations.push(self.player_idx - 1);
        }

        // right
        if (self.player_idx + 1) % self.length != 0 {
            valid_destinations.push(self.player_idx + 1);
        }

        // top
        if self.player_idx + self.length < self.map.len() {
            valid_destinations.push(self.player_idx + self.length);
        }

        // bottom
        if self.player_idx > self.length {
            valid_destinations.push(self.player_idx - self.length);
        }

        valid_destinations.retain(|&d| {
            self.map[d] <= valid_height || (self.map[d] == 'E' as u8 && valid_height > 'z' as u8)}
        );
        valid_destinations.retain(|&x| x != prev_pos);


        let x = self.player_idx % self.length;
        let y = self.player_idx / self.length;
        dbg!(self.player_idx, self.map[self.player_idx] as char);
        dbg!(x, y);
        let valid_dest_chars = valid_destinations.iter().map(|&d| self.map[d] as char).collect::<Vec<char>>();
        dbg!(valid_dest_chars);
        println!();

        valid_destinations
    }

    fn walk(&mut self, prev_pos: usize) {
        if self.map[self.player_idx] == 'E' as u8 {
            self.paths_found.push(self.cur_nbr_steps);
            return;
        }

        if self.player_idx == 0 && prev_pos == 0 {
            return;
        }

        let current_height = self.map[self.player_idx];
        let mut valid_destinations = self.get_valid_destinations_for_player(current_height, prev_pos);

        let cur_nbr_steps_before_walk = self.cur_nbr_steps;
        let player_pos_before_walk = self.player_idx;

        // dbg!(&prev_pos);
        // dbg!(self.player_idx);
        // dbg!(&valid_destinations);

        for valid_dest in &valid_destinations {
            self.player_idx = *valid_dest;
            self.cur_nbr_steps += 1;
            self.walk(player_pos_before_walk);
            self.player_idx = player_pos_before_walk;
            self.cur_nbr_steps = cur_nbr_steps_before_walk;
        }
    }

    fn get_fewest_steps_to_goal(&mut self) -> usize {
        self.walk(usize::MAX);

        dbg!(&self.paths_found);
        *self.paths_found.iter().min().unwrap()
    }
}

pub fn run() {
    let file_str = fs::read_to_string("inputs/testday12").unwrap();
    let player_idx = file_str.find('S').unwrap();
    let (len, height) = (file_str.find('\n').unwrap(), file_str.matches('\n').count());
    let mut map = Map {
        length: len,
        height,
        map: file_str.chars().filter_map(|c|
            if c == 'S' {
                Some('a' as u8 - 1) // Starting position is the lowest
            } else if c != '\n' {
                Some(c as u8)
            } else {
                None
            }).collect::<Vec<u8>>(),
        player_idx,
        paths_found: vec![],
        cur_nbr_steps: 0
    };

    // dbg!(&map);

    println!("Day 12: ");
    println!("Part 1: {}", map.get_fewest_steps_to_goal());
    println!("----------");
}