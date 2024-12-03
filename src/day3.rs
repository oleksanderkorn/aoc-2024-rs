use crate::utils::read_lines;
use regex::Regex;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day3.txt");
    let mut total = 0;
    for line in lines.iter() {
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        for m in matches {
            let mut res = 1;
            for num in m
                .replace("mul(", "")
                .replace(")", "")
                .split(',')
                .collect::<Vec<&str>>()
            {
                res *= num.parse::<i32>().unwrap();
            }
            total += res;
        }
    }
    println!("day 3 :: part 1 :: {total}");
}

#[derive(Clone, Copy, Debug)]
struct Op {
    kind: &'static str,
    start: usize,
    left: Option<i32>,
    right: Option<i32>,
}

pub fn solve_part_2() {
    let mut total = 0;
    let line = read_lines("inputs/day3_2.txt").join("");
    let mut ops: Vec<Op> = Vec::new();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let do_matches: Vec<_> = re_do.find_iter(&line).collect::<Vec<_>>();

    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let dont_matches: Vec<_> = re_dont.find_iter(&line).collect::<Vec<_>>();

    let re_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mul_matches: Vec<_> = re_mul.find_iter(&line).collect::<Vec<_>>();

    for m in mul_matches.iter() {
        let m_str = m.as_str().replace("mul(", "").replace(")", "");
        let nums = m_str.split(',').collect::<Vec<&str>>();

        ops.push(Op {
            kind: "mul",
            start: m.start(),
            left: Some(nums[0].parse::<i32>().unwrap()),
            right: Some(nums[1].parse::<i32>().unwrap()),
        })
    }

    for m in do_matches.iter() {
        ops.push(Op {
            kind: "do",
            start: m.start(),
            left: None,
            right: None,
        })
    }

    for m in dont_matches.iter() {
        ops.push(Op {
            kind: "dont",
            start: m.start(),
            left: None,
            right: None,
        })
    }

    ops.sort_by_key(|a| a.start);
    let mut can_mul = true;
    for op in ops {
        if op.kind == "dont" {
            //println!("don't(): start: {}", op.start);
            can_mul = false;
        } else if op.kind == "do" {
            //println!("do(): start: {}", op.start);
            can_mul = true;
        } else if can_mul && op.kind == "mul" {
            let res = op.left.unwrap() * op.right.unwrap();
            total += res;
            //println!(
            //    "take mul({},{}): start: {} = [{}] new total: {}",
            //    op.start,
            //    op.left.unwrap(),
            //    op.right.unwrap(),
            //    res,
            //    total
            //);
        } else {
            //println!("skip mul({},{})", op.left.unwrap(), op.right.unwrap());
        }
    }
    println!("day 3 :: part 2 :: {total}");
}
