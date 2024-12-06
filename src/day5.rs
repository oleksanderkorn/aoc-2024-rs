use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day5.txt");
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = Vec::new();
    let mut add_rules = true;
    for line in lines.iter() {
        if line.is_empty() {
            add_rules = false;
            continue;
        }
        if add_rules {
            let nums: Vec<i32> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
            rules
                .entry(nums[1])
                .and_modify(|n| n.push(nums[0]))
                .or_insert(vec![nums[0]]);
        } else {
            let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            updates.push(nums);
        }
    }
    let mut total = 0;
    for update in updates {
        let mut printed = HashSet::<i32>::new();
        let len = update.len();
        let mut is_valid = true;
        for n in update.iter() {
            let mut can_print = true;
            if rules.contains_key(n) {
                for num_rule in rules.get(n).unwrap() {
                    if update.contains(num_rule) && !printed.contains(num_rule) {
                        can_print = false;
                        break;
                    }
                }
            }
            if can_print {
                printed.insert(*n);
            } else {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            total += update[len / 2];
        }
    }
    println!("day 5 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day5.txt");
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = Vec::new();
    let mut add_rules = true;
    for line in lines.iter() {
        if line.is_empty() {
            add_rules = false;
            continue;
        }
        if add_rules {
            let nums: Vec<i32> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
            rules
                .entry(nums[1])
                .and_modify(|n| n.push(nums[0]))
                .or_insert(vec![nums[0]]);
        } else {
            let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            updates.push(nums);
        }
    }
    let mut invalid = Vec::new();
    for (i, update) in updates.iter().enumerate() {
        let mut is_valid = true;
        let mut printed = HashSet::<i32>::new();
        for n in update.iter() {
            let mut can_print = true;
            if rules.contains_key(n) {
                for num_rule in rules.get(n).unwrap() {
                    if update.contains(num_rule) && !printed.contains(num_rule) {
                        can_print = false;
                        break;
                    }
                }
            }
            if can_print {
                printed.insert(*n);
            } else {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            invalid.push(i);
        }
    }
    let mut total = 0;
    for idx in invalid {
        let mut printed = Vec::<i32>::new();
        let update = &updates[idx];
        for n in update.iter() {
            if printed.contains(n) {
                continue;
            }
            process(n, &mut printed, &rules, update);
        }
        total += printed[printed.len() / 2];
    }

    fn process(n: &i32, printed: &mut Vec<i32>, rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) {
        if rules.contains_key(n) {
            for num_rule in rules.get(n).unwrap() {
                if update.contains(num_rule) && !printed.contains(num_rule) {
                    process(num_rule, printed, rules, update);
                }
            }
            if !printed.contains(n) {
                printed.push(*n);
            }
        } else {
            printed.push(*n);
        }
    }
    println!("day 5 :: part 2 :: {total}");
}
