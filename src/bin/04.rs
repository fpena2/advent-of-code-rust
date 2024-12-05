advent_of_code::solution!(4);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in input.lines() {
        grid.push(row.chars().collect());
    }

    // print!("length: {}", grid.len());

    let mut instances = 0;
    let sequence = "XMAS";
    for (i_row, row) in grid.iter().enumerate() {
        for (i_col, c) in row.iter().enumerate() {
            if *c == 'X' {
                if front(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if back(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if up(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if down(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_up_left(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_up_right(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_down_left(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
                if diagonal_down_right(&grid, (i_row, i_col), sequence) {
                    instances += 1;
                }
            }
        }
    }

    Some(instances)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}

fn front(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn back(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn up(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn down(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn diagonal_up_left(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn diagonal_up_right(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn diagonal_down_left(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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

fn diagonal_down_right(grid: &Vec<Vec<char>>, point: (usize, usize), sequence: &str) -> bool {
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
