use std::fs;
use std::ops::Not;
use itertools::Itertools;

type GalaxyPos = (usize, usize);
type Galaxies = Vec<GalaxyPos>;

fn expand_universe(galaxies: &Galaxies, uni_width: usize, n_times_bigger: usize) -> Galaxies {
    let empty_cols: Vec<usize> = (0..uni_width).filter_map(|i| galaxies.iter().any(|g| i == g.0).not().then(|| i)).collect();
    let empty_rows: Vec<usize> = (0..uni_width).filter_map(|i| galaxies.iter().any(|g| i == g.1).not().then(|| i)).collect();

    galaxies.iter()
        .map(|g|
            (g.0 + empty_cols.iter().filter(|r_idx| **r_idx < g.0).count() * (n_times_bigger - 1), // minus 1 because e.g. 10 times bigger means 9 (10 - 1) more lines
             g.1 + empty_rows.iter().filter(|r_idx| **r_idx < g.1).count() * (n_times_bigger - 1)))
        .collect()
}

fn part1(galaxies: &Galaxies, uni_width: usize) -> usize {
    let new_galaxies = expand_universe(galaxies, uni_width, 1);

    new_galaxies.iter().tuple_combinations()
        .map(|(g1, g2)| { usize::abs_diff(g1.0, g2.0) + usize::abs_diff(g1.1, g2.1) })
        .sum::<usize>()
}

fn part2(galaxies: &Galaxies, uni_width: usize) -> usize {
    let new_galaxies = expand_universe(galaxies, uni_width, 1000000);

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
    println!("Part 2: {}", part2(&galaxies, uni_width));
}
