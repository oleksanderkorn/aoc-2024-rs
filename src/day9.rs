use std::collections::HashSet;

use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let line = &read_lines("inputs/day9.txt")[0][..]
        .chars()
        .collect::<Vec<_>>();
    let mut arr = Vec::new();
    let mut id = 0;
    (0..line.len()).step_by(2).for_each(|i| {
        let block_size = line[i].to_digit(10).unwrap() as usize;
        for _ in 0..block_size {
            arr.push(id.to_string());
        }
        if i + 1 < line.len() {
            let free_space = line[i + 1].to_digit(10).unwrap() as usize;
            for _ in 0..free_space {
                arr.push(".".to_string());
            }
        }
        id += 1;
    });
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        while arr[left] != "." {
            left += 1;
        }
        while arr[right] == "." {
            right -= 1;
        }
        arr[left] = arr[right].clone();
        arr[right] = ".".to_string();
        left += 1;
        right -= 1;
    }
    let mut total: i64 = 0;
    let mut id = 0;
    for s in arr.iter() {
        if *s != "." {
            total += id * s.parse::<i64>().unwrap();
            id += 1;
        }
    }
    println!("day 9 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let line = &read_lines("inputs/day9.txt")[0][..]
        .chars()
        .collect::<Vec<_>>();
    let mut arr = Vec::new();
    let mut id = 0;
    (0..line.len()).step_by(2).for_each(|i| {
        let block_size = line[i].to_digit(10).unwrap() as isize;
        arr.push((block_size, id.to_string()));
        if i + 1 < line.len() {
            let free_space = line[i + 1].to_digit(10).unwrap() as isize;
            arr.push((free_space, ".".to_string()));
        }
        id += 1;
    });

    fn print_arr(arr: &[(isize, String)]) {
        let mut strs = Vec::new();
        for (l, v) in arr.iter() {
            if v != "." {
                for _ in 0..*l as usize {
                    strs.push(v.clone());
                }
            }
        }
        println!("{:?}", strs.concat());
    }

    let mut moved = HashSet::new();
    let mut right = arr.len() - 1;
    while right > 0 {
        while right > 0 && (arr[right].1 == "." || moved.contains(&arr[right].1)) {
            right -= 1;
        }

        if right == 0 {
            break;
        }

        for l in 0..right {
            if arr[l].1 != "." || arr[l].0 < arr[right].0 {
                continue;
            }
            let id = arr[right].1.clone();
            moved.insert(id.clone());
            let remainder = arr[l].0 - arr[right].0;
            arr[l] = arr[right].clone();
            arr[right] = (arr[right].0, ".".to_string());
            if remainder > 0 {
                if l + 1 < arr.len() && arr[l + 1].1 == "." {
                    arr[l + 1] = (arr[l + 1].0 + remainder, ".".to_string());
                } else {
                    arr.insert(l + 1, (remainder, ".".to_string()));
                }
            }
            break;
        }
        right = right.saturating_sub(1);
    }
    let mut total: i64 = 0;
    let mut id = 0;
    let mut strs = Vec::new();
    for (l, v) in arr {
        for _ in 0..l {
            strs.push(v.clone());
        }
    }

    for s in strs.iter() {
        if s != "." {
            total += id * s.parse::<i64>().unwrap();
        }
        id += 1;
    }
    println!("day 9 :: part 2 :: {total}");
}
