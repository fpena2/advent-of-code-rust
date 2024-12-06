advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in input.lines() {
        grid.push(row.chars().collect());
    }

    let mut instances = 0;
    let sequence = "XMAS";
    for (i_row, row) in grid.iter().enumerate() {
        for (i_col, c) in row.iter().enumerate() {
            if *c == 'X' {
                if right(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if left(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if up(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if down(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_left_up(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_left_down(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_right_up(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_right_down(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
            }
        }
    }

    Some(instances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in input.lines() {
        grid.push(row.chars().collect());
    }

    let mut instances = 0;
    let sequence = "MAS";
    for (i_row, row) in grid.iter().enumerate() {
        for (i_col, c) in row.iter().enumerate() {
            if *c == 'A' {
                let point = (i_row, i_col);
                if cross_search(&grid, point, sequence) {
                    instances += 1;
                }
            }
        }
    }
    Some(instances)
}

pub fn cross_search(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let walk_nw_se = vec![(-1, -1), (0, 0), (1, 1)];
    let walk_ne_sw = vec![(1, -1), (0, 0), (-1, 1)];

    let mut test_sequence = String::new();
    for coor in walk_nw_se {
        let x = point.0 as i32 + coor.0;
        let y = point.1 as i32 + coor.1;

        if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
            return false;
        }

        test_sequence.push(grid[x as usize][y as usize]);
    }

    let check1 =
        test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>();

    let mut test_sequence = String::new();
    for coor in walk_ne_sw {
        let x = point.0 as i32 + coor.0;
        let y = point.1 as i32 + coor.1;

        if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
            return false;
        }

        test_sequence.push(grid[x as usize][y as usize]);
    }

    let check2 =
        test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>();

    if check1 && check2 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

fn down(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.1 + sequence_len > grid[0].len() {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0][point.1 + i]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn up(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.1 < sequence_len - 1 {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0][point.1 - i]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn right(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.0 + sequence_len > grid.len() {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0 + i][point.1]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn left(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.0 < sequence_len - 1 {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0 - i][point.1]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn diagonal_left_up(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.0 < sequence_len - 1 || point.1 < sequence_len - 1 {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0 - i][point.1 - i]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn diagonal_left_down(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.0 < sequence_len - 1 || point.1 + sequence_len > grid[0].len() {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0 - i][point.1 + i]);
    }
    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn diagonal_right_up(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.0 + sequence_len > grid.len() || point.1 < sequence_len - 1 {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0 + i][point.1 - i]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}

fn diagonal_right_down(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
    let sequence_len = sequence.len();
    if point.0 + sequence_len > grid.len() || point.1 + sequence_len > grid[0].len() {
        return false;
    }

    let mut test_sequence = String::new();
    for i in 0..sequence_len {
        test_sequence.push(grid[point.0 + i][point.1 + i]);
    }

    test_sequence == sequence || test_sequence == sequence.chars().rev().collect::<String>()
}
