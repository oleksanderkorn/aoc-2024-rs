use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day2.txt");
    let mut total = 0;
    for line in lines.iter() {
        let mut is_safe = true;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let inc = nums[0] < nums[1];
        for i in 0..nums.len() - 1 {
            let curr = nums[i];
            let next = nums[i + 1];
            let diff = (next - curr).abs();
            if diff == 0 || diff > 3 || inc && next < curr || !inc && next > curr {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            total += 1;
        }
    }
    println!("day 2 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    fn scan(nums: &[i32]) -> bool {
        let inc = nums[0] < nums[1];
        let mut i = 0;
        while i < nums.len() - 1 {
            let curr = nums[i];
            let next = nums[i + 1];
            let diff = (next - curr).abs();
            if diff == 0 || diff > 3 || inc && next < curr || !inc && next > curr {
                return false;
            }
            i += 1;
        }
        true
    }

    let lines = read_lines("inputs/day2.txt");
    let mut total = 0;
    for line in lines.iter() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if scan(&nums) {
            total += 1;
        } else {
            // check each index removed
            for i in 0..nums.len() {
                let mut skip = nums.clone();
                skip.remove(i);
                if scan(&skip) {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("day 2 :: part 2 :: {total}");
}
