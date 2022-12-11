use std::fs;
use std::process::exit;

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

fn do_n_rounds(mut monkeys: Vec<Monkey>, n: usize) -> usize {
    for round_idx in 1..n + 1 {
        for mut monkey in monkeys.iter_mut() {
            for item in &monkey.items {
                let mut new_item_val = match monkey.operation {
                    Operation::Plus(val) => item + val,
                    Operation::Times(val) => item * val,
                    Operation::TimesItself => item * item
                };
                new_item_val = (new_item_val as f64 / 3.0).round() as usize;

                match new_item_val % monkey.test_divisible_by == 0 {
                    true => monkeys.get_mut(monkey.monkey_idx_if_true as usize).unwrap().items.push(new_item_val),
                    false => monkeys.get_mut(monkey.monkey_idx_if_false as usize).unwrap().items.push(new_item_val),
                }
            };
            monkey.items.clear();
        }

        println!("After round {}, the monkeys are holding items with these worry levels:", round_idx);
        for monkey in &monkeys {
            println!("Monkey {}: {}", monkey.idx, monkey.items.iter().map(|item_nbr| item_nbr.to_string()).collect::<Vec<String>>().join(", "));
        }
        println!();
    }
    42
}

pub fn run() {
    let mut monkeys = fs::read_to_string("inputs/testday11").unwrap()
        .split("\n\n")
        .map(|monkey_str| Monkey::from_string(monkey_str))
        .collect::<Vec<Monkey>>();

    println!("Day 11: ");
    println!("Part 1: {}", do_n_rounds(monkeys, 1));
    println!("----------");
}