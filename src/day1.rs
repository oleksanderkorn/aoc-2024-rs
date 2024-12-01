use crate::utils::read_lines;
use std::collections::{BinaryHeap, HashMap};

pub fn solve() {
    solve_part_1();
    solve_part_2();
}
pub fn solve_part_1() {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    let lines = read_lines("inputs/day1.txt");
    for line in lines.iter() {
        let parts = line.split("   ").collect::<Vec<&str>>();
        left.push(parts[0]);
        right.push(parts[1]);
    }
    let mut total = 0;
    while !left.is_empty() {
        let first = left.pop().unwrap().parse::<i32>().unwrap();
        let second = right.pop().unwrap().parse::<i32>().unwrap();
        total += (first - second).abs();
    }
    println!("day 1 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let mut left = Vec::new();
    let mut right = HashMap::new();
    let lines = read_lines("inputs/day1.txt");
    for line in lines.iter() {
        let parts = line.split("   ").collect::<Vec<&str>>();
        left.push(parts[0]);
        let key = parts[1];
        if right.contains_key(key) {
            *right.get_mut(key).unwrap() += 1;
        } else {
            right.insert(key, 1);
        }
    }
    let mut result = 0;
    for s in left {
        let num = s.parse::<i32>().unwrap();
        if right.contains_key(s) {
            result += num * right.get(s).unwrap();
        }
    }
    println!("day 1 :: part 2 :: {result}");
}
