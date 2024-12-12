use std::collections::HashMap;

use crate::utils::read_lines;

use rayon::prelude::*;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let cache = HashMap::new();
    let stones: usize = read_lines("inputs/day11.txt")[0]
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| calculate(v, 25, &mut cache.clone()))
        .sum();
    println!("day 11 :: part 1 :: {}", stones);
}

pub fn solve_part_2() {
    let cache = HashMap::new();
    let stones: usize = read_lines("inputs/day11.txt")[0]
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| calculate(v, 75, &mut cache.clone()))
        .sum();
    println!("day 11 :: part 2 :: {stones}");
}

fn calculate(v: u64, blinks: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
    if blinks == 0 {
        1
    } else if let Some(count) = cache.get(&(v, blinks)) {
        *count
    } else {
        let count = match v {
            0 => calculate(1, blinks - 1, cache),
            v => {
                let digits = v.ilog10() + 1;
                if digits % 2 == 0 {
                    calculate(v / 10u64.pow(digits / 2), blinks - 1, cache)
                        + calculate(v % 10u64.pow(digits / 2), blinks - 1, cache)
                } else {
                    calculate(v * 2024, blinks - 1, cache)
                }
            }
        };
        cache.insert((v, blinks), count);
        count
    }
}
