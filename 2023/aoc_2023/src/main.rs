use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod map;
mod day11;
mod day12;
mod day13;

fn main() {
    let args: Vec<_> = env::args().collect();

    // default case
    if args.len() == 1 {
        day13::run();
        return;
    }

    match args.get(1).unwrap().parse::<usize>().unwrap() {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        10 => day10::run(),
        11 => day11::run(),
        12 => day12::run(),
        13 => day13::run(),
        _ => panic!("Day not done yet, or invalid input.")
    }
}
