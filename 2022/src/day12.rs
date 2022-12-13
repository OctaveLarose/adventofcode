use std::fs;
use pathfinding::astar;

#[derive(Debug)]
struct Map {
    length: usize,
    map: Vec<u8>,
    start_player_idx: usize,
    goal_idx: usize
}

impl Map {
    fn get_valid_destinations_for_player(&self, player_idx: usize) -> Vec<(usize, usize)> {
        let mut valid_destinations = vec![];
        let current_height = self.map[player_idx];
        let valid_height = current_height + 1;

        // left
        if player_idx % self.length != 0 {
            valid_destinations.push(player_idx - 1);
        }

        // right
        if (player_idx + 1) % self.length != 0 {
            valid_destinations.push(player_idx + 1);
        }

        // bottom
        if player_idx + self.length < self.map.len() {
            valid_destinations.push(player_idx + self.length);
        }

        // top
        if player_idx >= self.length {
            valid_destinations.push(player_idx - self.length);
        }

        valid_destinations.retain(|&d| self.map[d] <= valid_height);
        valid_destinations.iter().map(|&d| (d, 1)).collect::<Vec<(usize, usize)>>()
    }

    fn get_heuristic(&self, pos: usize) -> usize {
        let pos_x = pos % self.length;
        let pos_y = pos / self.length;
        let goal_x = self.goal_idx % self.length;
        let goal_y = self.goal_idx / self.length;

        usize::abs_diff(goal_x, pos_x) + usize::abs_diff(goal_y, pos_y)
    }
}

fn part1(map: &Map) -> usize {
    astar(
        &map.start_player_idx,
        |&p| map.get_valid_destinations_for_player(p).into_iter(),
        |&p| map.get_heuristic(p),
        |&p| p == map.goal_idx)
        .unwrap()
        .1
}

fn part2(map: &Map) -> usize {
    let all_starting_points = map.map.iter()
        .enumerate()
        .filter_map(|(idx, &val)| if val == 96 || val == 97 { Some(idx) } else { None })
        .collect::<Vec<usize>>();

    *all_starting_points.iter().filter_map(
        |start| {
            astar(
                start,
                |&p| map.get_valid_destinations_for_player(p).into_iter(),
                |&p| map.get_heuristic(p),
                |&p| p == map.goal_idx)
        })
        .map(|(p1, p2) | p2)
        .collect::<Vec<usize>>().iter()
        .min().unwrap()
}

pub fn run() {
    let mut file_str = fs::read_to_string("inputs/day12").unwrap();
    let len = file_str.find('\n').unwrap();
    file_str.retain(|c| c != '\n');

    let map = Map {
        length: len,
        map: file_str.chars().filter_map(|c|
            if c == 'S' {
                Some('a' as u8 - 1) // Starting position is the lowest
            } else if c == 'E' {
                Some('z' as u8 + 1) // ...and goal the highest
            } else if c != '\n' {
                Some(c as u8)
            } else {
                None
            }).collect::<Vec<u8>>(),
        start_player_idx: file_str.find('S').unwrap(),
        goal_idx: file_str.find('E').unwrap()
    };

    println!("Day 12: ");
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
    println!("----------");
}