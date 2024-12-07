use std::collections::HashSet;

use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day6.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    const DIRECTIONS: [(isize, isize); 4] = [
        (-1, 0), //up
        (0, 1),  //right
        (1, 0),  //down
        (0, -1), //left
    ];
    let mut total = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                scan(&mut matrix, i, j);
            }
        }
    }
    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if matrix[i][j] == 'X' {
                total += 1;
            }
        });
    });

    println!("day 6 :: part 1 :: {total}");

    fn scan(matrix: &mut [Vec<char>], i: usize, j: usize) {
        let mut dir = 0;
        matrix[i][j] = 'X';
        let mut next_i = i as isize + DIRECTIONS[dir].0;
        let mut next_j = j as isize + DIRECTIONS[dir].1;
        while next_i >= 0
            && next_j >= 0
            && (next_i as usize) < matrix.len()
            && (next_j as usize) < matrix[0].len()
        {
            if matrix[next_i as usize][next_j as usize] == '#' {
                next_i -= DIRECTIONS[dir].0;
                next_j -= DIRECTIONS[dir].1;
                dir = (dir + 1) % 4;
            } else {
                matrix[next_i as usize][next_j as usize] = 'X';
                next_i += DIRECTIONS[dir].0;
                next_j += DIRECTIONS[dir].1;
            }
        }
    }
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day6.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut total = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                total = scan(&mut matrix, i, j);
            }
        }
    }

    println!("day 6 :: part 2 :: {total}");

    fn scan(matrix: &mut [Vec<char>], i: usize, j: usize) -> i32 {
        fn has_loop(matrix: &[Vec<char>], i: usize, j: usize) -> bool {
            const DIRECTIONS: [(isize, isize); 4] = [
                (-1, 0), //up
                (0, 1),  //right
                (1, 0),  //down
                (0, -1), //left
            ];
            let mut dir = 0;
            let mut next_i = i as isize + DIRECTIONS[dir].0;
            let mut next_j = j as isize + DIRECTIONS[dir].1;
            let mut encountered_obs = HashSet::<(usize, isize, isize)>::new();
            while next_i >= 0
                && next_j >= 0
                && (next_i as usize) < matrix.len()
                && (next_j as usize) < matrix[0].len()
            {
                if matrix[next_i as usize][next_j as usize] == '#' {
                    if encountered_obs.contains(&(dir, next_i, next_j)) {
                        return true;
                    }
                    encountered_obs.insert((dir, next_i, next_j));
                    next_i -= DIRECTIONS[dir].0;
                    next_j -= DIRECTIONS[dir].1;
                    dir = (dir + 1) % 4;
                } else {
                    next_i += DIRECTIONS[dir].0;
                    next_j += DIRECTIONS[dir].1;
                }
            }
            false
        }

        let mut obstructions = 0;
        for ii in 0..matrix.len() {
            for jj in 0..matrix[0].len() {
                if matrix[ii][jj] == '.' {
                    matrix[ii][jj] = '#';
                    if has_loop(matrix, i, j) {
                        obstructions += 1;
                    }
                    matrix[ii][jj] = '.';
                }
            }
        }
        obstructions
    }
}
