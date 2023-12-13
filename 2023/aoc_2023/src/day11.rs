use std::fs;
use std::ops::Not;
use itertools::Itertools;

type GalaxyPos = (usize, usize);
type Galaxies = Vec<GalaxyPos>;

fn expand_universe(galaxies: &Galaxies, uni_width: usize) -> Galaxies {
    let empty_cols: Vec<usize> = (0..uni_width).filter_map(|i| galaxies.iter().any(|g| i == g.0).not().then(|| i)).collect();
    let empty_rows: Vec<usize> = (0..uni_width).filter_map(|i| galaxies.iter().any(|g| i == g.1).not().then(|| i)).collect();

    galaxies.iter()
        .map(|g|
            (g.0 + empty_cols.iter().filter(|r_idx| **r_idx < g.0).count(),
             g.1 + empty_rows.iter().filter(|r_idx| **r_idx < g.1).count()))
        .collect()
}

fn part1(galaxies: &Galaxies, uni_width: usize) -> usize {
    let new_galaxies = expand_universe(galaxies, uni_width);

    new_galaxies.iter().tuple_combinations()
        .map(|(g1, g2)| { usize::abs_diff(g1.0, g2.0) + usize::abs_diff(g1.1, g2.1) })
        .sum::<usize>()
}

pub fn run() {
    let file_str = fs::read_to_string("../inputs/day11").unwrap();
    let uni_width = file_str.find('\n').unwrap();
    let galaxies: Galaxies = file_str.chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter_map(|(idx, c)| (c == '#').then(|| idx)).
        map(|g_idx| (g_idx % uni_width, g_idx / uni_width))
        .collect();

    println!("Part 1: {}", part1(&galaxies, uni_width));
    // println!("Part 2: {}", &histories.iter().map(part2_rec).sum::<isize>());
}
