use crate::utils::read_lines;

pub fn solve() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let lines = read_lines("inputs/day12.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut visit = Vec::new();
    let col_size = matrix[0].len();
    for _ in 0..matrix.len() {
        visit.push(vec![false; col_size]);
    }

    let mut total = 0;

    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if !visit[i][j] {
                let plants = dfs(&matrix, i as isize, j as isize, &mut visit, matrix[i][j]);
                let area = plants.len();
                let plant = matrix[plants[0].0][plants[0].1];
                let perimeter = calculate_perimeter(plant, &plants, &matrix);
                //println!(
                //    "{}: {:?} area: {}, perimeter: {}",
                //    matrix[plants[0].0][plants[0].1], plants, area, perimeter
                //);
                total += area * perimeter;
            }
        });
    });

    println!("day 12 :: part 1 :: {}", total);

    fn calculate_perimeter(plant: char, plants: &[(usize, usize)], matrix: &[Vec<char>]) -> usize {
        let mut perimeter = 0;
        for &(row, col) in plants {
            let mut fences = 4;
            if row < matrix.len() - 1 && matrix[row + 1][col] == plant {
                fences -= 1;
            }
            if row > 0 && matrix[row - 1][col] == plant {
                fences -= 1;
            }
            if col < matrix[0].len() - 1 && matrix[row][col + 1] == plant {
                fences -= 1;
            }
            if col > 0 && matrix[row][col - 1] == plant {
                fences -= 1;
            }
            perimeter += fences;
        }

        perimeter
    }
}

pub fn solve_part_2() {
    let lines = read_lines("inputs/day12.test.txt");
    let mut matrix = Vec::new();
    for line in lines.iter() {
        matrix.push(line.chars().collect::<Vec<char>>());
    }
    let mut visit = Vec::new();
    let col_size = matrix[0].len();
    for _ in 0..matrix.len() {
        visit.push(vec![false; col_size]);
    }

    let mut total = 0;

    (0..matrix.len()).for_each(|i| {
        (0..matrix[i].len()).for_each(|j| {
            if !visit[i][j] {
                let mut plants = dfs(&matrix, i as isize, j as isize, &mut visit, matrix[i][j]);
                let area = plants.len();
                let plant = matrix[plants[0].0][plants[0].1];
                let sides = count_sides(plant, &mut plants, &matrix);
                //println!(
                //    "{}: {:?} area: {}, sides: {}",
                //    matrix[plants[0].0][plants[0].1], plants, area, sides
                //);
                total += area * sides;
            }
        });
    });

    println!("day 12 :: part 2 :: {}", total);

    fn count_sides(plant: char, plants: &mut [(usize, usize)], matrix: &[Vec<char>]) -> usize {
        plants.sort_by_key(|a| a.0);
        let mut sides = 0;
        let mut prev_row = None;
        let mut prev_col = None;
        let mut prev_up = false;
        let mut prev_down = false;
        for p in plants.iter() {
            let row = p.0;
            let col = p.1;

            let diff_plant_up = row == 0 || matrix[row - 1][col] != plant;
            let diff_plant_down = row == matrix.len() - 1 || matrix[row + 1][col] != plant;

            let same_row = prev_row.is_some() && prev_row.unwrap() == row;
            let is_adj_col = prev_col.is_some() && prev_col.unwrap() + 1 == col;

            if diff_plant_up && (!same_row || (!is_adj_col || !prev_up)) {
                //println!("{plant}: [{row},{col}] is new row upper side");
                sides += 1;
            } else {
                //println!("{plant}: [{row},{col}] is as up skip");
            }
            prev_up = diff_plant_up;

            if diff_plant_down && (!same_row || (!is_adj_col || !prev_down)) {
                //println!("{plant}: [{row},{col}] is new row down side");
                sides += 1;
            } else {
                //println!("{plant}: [{row},{col}] is as down skip");
            }
            prev_down = diff_plant_down;
            prev_col = Some(col);
            prev_row = Some(row);
        }
        //println!("{plant}: sides: {sides}");
        plants.sort_by_key(|a| a.1);
        prev_row = None;
        prev_col = None;
        let mut prev_left = false;
        let mut prev_right = false;
        for p in plants.iter() {
            let row = p.0;
            let col = p.1;
            let diff_plant_left = col == 0 || matrix[row][col - 1] != plant;
            let diff_plant_right = col == matrix[0].len() - 1 || matrix[row][col + 1] != plant;
            let same_col = prev_col.is_some() && prev_col.unwrap() == col;

            let is_adj_row = prev_row.is_some() && prev_row.unwrap() + 1 == row;

            if diff_plant_left && (!same_col || (!is_adj_row || !prev_left)) {
                //println!("{plant}: [{row},{col}] is new col left side");
                sides += 1;
            } else {
                //println!("{plant}: [{row},{col}] is as col left skip");
            }

            if diff_plant_right && (!same_col || (!is_adj_row || !prev_right)) {
                //println!("{plant}: [{row},{col}] is new col right side");
                sides += 1;
            } else {
                //println!("{plant}: [{row},{col}] is as col down skip");
            }
            prev_left = diff_plant_left;
            prev_right = diff_plant_right;
            prev_col = Some(col);
            prev_row = Some(row);
        }
        //println!("{plant}: sides: {sides}");
        sides
    }
}

fn dfs(
    matrix: &Vec<Vec<char>>,
    i: isize,
    j: isize,
    visit: &mut Vec<Vec<bool>>,
    plant: char,
) -> Vec<(usize, usize)> {
    if i < 0 && j < 0 || i as usize >= matrix.len() || j as usize >= matrix[0].len() {
        return vec![];
    }
    if visit[i as usize][j as usize] || matrix[i as usize][j as usize] != plant {
        return vec![];
    }
    visit[i as usize][j as usize] = true;
    let mut plants = Vec::new();
    plants.push((i as usize, j as usize));
    plants.append(&mut dfs(matrix, i - 1, j, visit, plant));
    plants.append(&mut dfs(matrix, i + 1, j, visit, plant));
    plants.append(&mut dfs(matrix, i, j - 1, visit, plant));
    plants.append(&mut dfs(matrix, i, j + 1, visit, plant));
    plants
}
