use std::collections::HashSet;

use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day10.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut total = 0;
    let mut trailheads = Vec::new();
    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if matrix[i][j] == '0' {
                let mut curr = HashSet::<(isize, isize)>::new();
                dfs(&mut matrix, i as isize, j as isize, -1, &mut curr);
                if !curr.is_empty() {
                    total += curr.len();
                    trailheads.push((i, j, curr.len()));
                }
            }
        });
    });

    fn dfs(
        matrix: &mut Vec<Vec<char>>,
        i: isize,
        j: isize,
        prev: isize,
        curr: &mut HashSet<(isize, isize)>,
    ) {
        if i < 0 && j < 0 || i as usize >= matrix.len() || j as usize >= matrix[0].len() {
            return;
        }
        if matrix[i as usize][j as usize].to_digit(10) != Some((prev + 1) as u32) {
            return;
        }

        if matrix[i as usize][j as usize].to_digit(10) == Some(9) {
            curr.insert((i, j));
            return;
        }
        let v = matrix[i as usize][j as usize];
        matrix[i as usize][j as usize] = 'x';
        dfs(matrix, i - 1, j, prev + 1, curr);
        dfs(matrix, i + 1, j, prev + 1, curr);
        dfs(matrix, i, j - 1, prev + 1, curr);
        dfs(matrix, i, j + 1, prev + 1, curr);
        matrix[i as usize][j as usize] = v;
    }
    println!("day 10 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day10.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut total = 0;
    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if matrix[i][j] == '0' {
                let mut curr = 0;
                dfs(&mut matrix, i as isize, j as isize, -1, &mut curr);
                total += curr;
            }
        });
    });

    fn dfs(matrix: &mut Vec<Vec<char>>, i: isize, j: isize, prev: isize, curr: &mut i32) {
        if i < 0 && j < 0 || i as usize >= matrix.len() || j as usize >= matrix[0].len() {
            return;
        }
        if matrix[i as usize][j as usize].to_digit(10) != Some((prev + 1) as u32) {
            return;
        }

        if matrix[i as usize][j as usize].to_digit(10) == Some(9) {
            *curr += 1;
            return;
        }
        dfs(matrix, i - 1, j, prev + 1, curr);
        dfs(matrix, i + 1, j, prev + 1, curr);
        dfs(matrix, i, j - 1, prev + 1, curr);
        dfs(matrix, i, j + 1, prev + 1, curr);
    }
    println!("day 10 :: part 1 :: {total}");
}
