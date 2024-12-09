use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day8.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut antennas = HashMap::new();
    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if matrix[i][j] != '.' {
                antennas
                    .entry(matrix[i][j])
                    .and_modify(|n: &mut Vec<(isize, isize)>| n.push((i as isize, j as isize)))
                    .or_insert(vec![(i as isize, j as isize)]);
            }
        });
    });

    fn is_valid(r: isize, c: isize, matrix: &[Vec<char>]) -> bool {
        r >= 0 && c >= 0 && (r as usize) < matrix.len() && (c as usize) < matrix[0].len()
    }

    let mut antinodes = HashSet::<(isize, isize)>::new();
    for entry in antennas.iter() {
        for i in 0..entry.1.len() {
            for j in (i + 1)..entry.1.len() {
                let lhs = entry.1[i];
                let rhs = entry.1[j];
                let row_diff = (lhs.0 - rhs.0).abs();
                let col_diff = (lhs.1 - rhs.1).abs();
                if lhs.1 <= rhs.1 {
                    // left to right diagonal
                    let up_row = lhs.0 - row_diff;
                    let up_col = lhs.1 - col_diff;
                    if is_valid(up_row, up_col, &matrix) {
                        antinodes.insert((up_row, up_col));
                    }
                    let down_row = rhs.0 + row_diff;
                    let down_col = rhs.1 + col_diff;
                    if is_valid(down_row, down_col, &matrix) {
                        antinodes.insert((down_row, down_col));
                    }
                } else if lhs.1 > rhs.1 {
                    // right to left diagonal
                    let up_row = lhs.0 - row_diff;
                    let up_col = lhs.1 + col_diff;
                    if is_valid(up_row, up_col, &matrix) {
                        antinodes.insert((up_row, up_col));
                    }
                    let down_row = rhs.0 + row_diff;
                    let down_col = rhs.1 - col_diff;
                    if is_valid(down_row, down_col, &matrix) {
                        antinodes.insert((down_row, down_col));
                    }
                }
            }
        }
    }
    let total = &antinodes.len();
    println!("day 8 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day8.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut antennas = HashMap::new();
    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if matrix[i][j] != '.' {
                antennas
                    .entry(matrix[i][j])
                    .and_modify(|n: &mut Vec<(isize, isize)>| n.push((i as isize, j as isize)))
                    .or_insert(vec![(i as isize, j as isize)]);
            }
        });
    });

    fn is_valid(r: isize, c: isize, matrix: &[Vec<char>]) -> bool {
        r >= 0 && c >= 0 && (r as usize) < matrix.len() && (c as usize) < matrix[0].len()
    }

    let mut antinodes = HashSet::<(isize, isize)>::new();
    for entry in antennas.iter() {
        for i in 0..entry.1.len() {
            for j in (i + 1)..entry.1.len() {
                let lhs = entry.1[i];
                let rhs = entry.1[j];
                antinodes.insert((lhs.0, lhs.1));
                antinodes.insert((rhs.0, rhs.1));
                let row_diff = (lhs.0 - rhs.0).abs();
                let col_diff = (lhs.1 - rhs.1).abs();
                if lhs.1 <= rhs.1 {
                    let mut up_row = lhs.0 - row_diff;
                    let mut up_col = lhs.1 - col_diff;
                    while is_valid(up_row, up_col, &matrix) {
                        antinodes.insert((up_row, up_col));
                        up_row -= row_diff;
                        up_col -= col_diff;
                    }
                    let mut down_row = rhs.0 + row_diff;
                    let mut down_col = rhs.1 + col_diff;
                    while is_valid(down_row, down_col, &matrix) {
                        antinodes.insert((down_row, down_col));
                        down_row += row_diff;
                        down_col += col_diff;
                    }
                } else if lhs.1 > rhs.1 {
                    let mut up_row = lhs.0 - row_diff;
                    let mut up_col = lhs.1 + col_diff;
                    while is_valid(up_row, up_col, &matrix) {
                        antinodes.insert((up_row, up_col));
                        up_row -= row_diff;
                        up_col += col_diff;
                    }
                    let mut down_row = rhs.0 + row_diff;
                    let mut down_col = rhs.1 - col_diff;
                    while is_valid(down_row, down_col, &matrix) {
                        antinodes.insert((down_row, down_col));
                        down_row += row_diff;
                        down_col -= col_diff;
                    }
                }
            }
        }
    }
    let total = &antinodes.len();
    println!("day 8 :: part 2 :: {total}");
}
