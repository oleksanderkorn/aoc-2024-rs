use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day7.txt");
    let mut total = 0;
    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        let test = split[0].parse::<i64>().unwrap();
        let nums: Vec<i64> = split[1]
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if is_valid(&nums, &test, 1, nums[0]) {
            total += test;
        }
    }

    fn is_valid(nums: &Vec<i64>, test: &i64, i: usize, curr: i64) -> bool {
        if i == nums.len() {
            return curr == *test;
        }
        is_valid(nums, test, i + 1, curr + nums[i]) || is_valid(nums, test, i + 1, curr * nums[i])
    }
    println!("day 7 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day7.txt");
    let mut total = 0;
    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        let test = split[0].parse::<i64>().unwrap();
        let nums: Vec<i64> = split[1]
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if is_valid(&nums, &test, 1, nums[0]) {
            total += test;
        }
    }

    fn concat(lhs: i64, rhs: i64) -> i64 {
        let ls = lhs.to_string();
        let rs = rhs.to_string();
        (ls + &rs).parse::<i64>().unwrap()
    }

    fn is_valid(nums: &Vec<i64>, test: &i64, i: usize, curr: i64) -> bool {
        if i == nums.len() {
            return curr == *test;
        }
        is_valid(nums, test, i + 1, curr + nums[i])
            || is_valid(nums, test, i + 1, curr * nums[i])
            || is_valid(nums, test, i + 1, concat(curr, nums[i]))
    }
    println!("day 7 :: part 2 :: {total}");
}
