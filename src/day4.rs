use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day4.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut total = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // XMAS
            // try going down
            if i < matrix.len() - 3
                && matrix[i][j] == 'X'
                && matrix[i + 1][j] == 'M'
                && matrix[i + 2][j] == 'A'
                && matrix[i + 3][j] == 'S'
            {
                total += 1;
            }
            // try going up
            if i > 2
                && matrix[i][j] == 'X'
                && matrix[i - 1][j] == 'M'
                && matrix[i - 2][j] == 'A'
                && matrix[i - 3][j] == 'S'
            {
                total += 1;
            }
            // try going left
            if j < matrix[0].len() - 3
                && matrix[i][j] == 'X'
                && matrix[i][j + 1] == 'M'
                && matrix[i][j + 2] == 'A'
                && matrix[i][j + 3] == 'S'
            {
                total += 1;
            }
            // try going right
            if j > 2
                && matrix[i][j] == 'X'
                && matrix[i][j - 1] == 'M'
                && matrix[i][j - 2] == 'A'
                && matrix[i][j - 3] == 'S'
            {
                total += 1;
            }
            // try going up and right
            if i > 2
                && j < matrix[0].len() - 3
                && matrix[i][j] == 'X'
                && matrix[i - 1][j + 1] == 'M'
                && matrix[i - 2][j + 2] == 'A'
                && matrix[i - 3][j + 3] == 'S'
            {
                total += 1;
            }
            // try going up and left
            if i > 2
                && j > 2
                && matrix[i][j] == 'X'
                && matrix[i - 1][j - 1] == 'M'
                && matrix[i - 2][j - 2] == 'A'
                && matrix[i - 3][j - 3] == 'S'
            {
                total += 1;
            }
            // try going down and right
            if i < matrix.len() - 3
                && j < matrix[0].len() - 3
                && matrix[i][j] == 'X'
                && matrix[i + 1][j + 1] == 'M'
                && matrix[i + 2][j + 2] == 'A'
                && matrix[i + 3][j + 3] == 'S'
            {
                total += 1;
            }
            // try going down and left
            if i < matrix.len() - 3
                && j > 2
                && matrix[i][j] == 'X'
                && matrix[i + 1][j - 1] == 'M'
                && matrix[i + 2][j - 2] == 'A'
                && matrix[i + 3][j - 3] == 'S'
            {
                total += 1;
            }
        }
    }
    println!("day 4 :: part 1 :: {total}");
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day4.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut total = 0;
    for i in 0..matrix.len() - 2 {
        for j in 0..matrix[i].len() - 2 {
            // search for
            //M.S
            //.A.
            //M.S
            if matrix[i][j] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j + 2] == 'S'
                && matrix[i][j + 2] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'M'
            {
                total += 1;
            }
            //S.S
            //.A.
            //M.M
            if matrix[i][j] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j + 2] == 'M'
                && matrix[i][j + 2] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'M'
            {
                total += 1;
            }
            //S.M
            //.A.
            //S.M
            if matrix[i][j] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j + 2] == 'M'
                && matrix[i][j + 2] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'S'
            {
                total += 1;
            }
            //M.M
            //.A.
            //S.S
            if matrix[i][j] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j + 2] == 'S'
                && matrix[i][j + 2] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'S'
            {
                total += 1;
            }
        }
    }
    println!("day 4 :: part 2 :: {total}");
}
