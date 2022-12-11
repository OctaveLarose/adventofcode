use std::fs;

#[derive(Debug)]
enum Operation {
    Plus(usize),
    Times(usize),
    TimesItself
}

#[derive(Debug)]
struct Monkey {
    idx: u8,
    items: Vec<usize>,
    operation: Operation,
    test_divisible_by: usize,
    monkey_idx_if_true: u8,
    monkey_idx_if_false: u8,
    nbr_inspections: usize
}

impl Monkey {
    pub fn from_string(str: &str) -> Monkey {
        let mut line_iter = str.lines();

        let monkey_idx = line_iter.next().unwrap().chars().nth(7).unwrap() as u8 - ('0' as u8);
        let starting_items = line_iter.next().unwrap()[18..]
            .split(", ")
            .map(|item_str| item_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let operations_str = &line_iter.next().unwrap()[23..];
        let operation = match operations_str.chars().nth(0).unwrap() {
            '+' => {Operation::Plus( operations_str[2..].parse::<usize>().unwrap()) },
            '*' => {
                match &operations_str[2..] {
                    "old" => Operation::TimesItself,
                    val => Operation::Times(val.parse::<usize>().unwrap())
                }
            }
            _ => panic!("Invalid operator")
        };

        let test_divisible_by = &line_iter.next().unwrap()[21..].parse::<usize>().unwrap();
        let if_true_monkey_idx = line_iter.next().unwrap()[29..].parse::<u8>().unwrap();
        let if_false_monkey_idx = line_iter.next().unwrap()[30..].parse::<u8>().unwrap();

        Monkey {
            idx: monkey_idx,
            items: starting_items,
            operation,
            test_divisible_by: *test_divisible_by,
            monkey_idx_if_true: if_true_monkey_idx,
            monkey_idx_if_false: if_false_monkey_idx,
            nbr_inspections: 0
        }
        }
}

pub fn run() {
    let monkeys = fs::read_to_string("inputs/day11").unwrap()
        .split("\n\n")
        .map(|monkey_str| Monkey::from_string(monkey_str))
        .collect::<Vec<Monkey>>();

    dbg!(monkeys);
    println!("Day 11: ");
    println!("----------");
}