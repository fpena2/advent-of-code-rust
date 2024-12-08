advent_of_code::solution!(7);

use std::iter;

use itertools::Itertools;

fn generate_operation_combinations(k: usize) -> Vec<Vec<char>> {
    // 'c' is the concatenation operator
    let operations = vec!['+', '*', 'c'];
    let combinations = iter::repeat(operations).take(k).multi_cartesian_product();

    combinations
        .into_iter()
        .map(|combination| combination.to_vec())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut calibration_results: u64 = 0;
    for line in input.lines() {
        let function = line.split(": ").collect::<Vec<&str>>();
        let result = function[0].parse::<u64>().unwrap();
        let operations = function[1]
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let operator_combinations = generate_operation_combinations(operations.len() - 1);
        for operator_combination in operator_combinations.iter() {
            let mut operator_combination_view = operator_combination.iter().peekable();
            let mut operands_view = operations.iter().peekable();

            let initial_value = *operands_view.next().unwrap();
            let test = operands_view.fold(initial_value, |acc, x| {
                let operator = *operator_combination_view.next().unwrap();

                if operator == '+' {
                    acc + x
                } else {
                    acc * x
                }
            });

            if test == result {
                calibration_results += test;
                break;
            }
        }
    }

    Some(calibration_results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calibration_results: u64 = 0;
    for line in input.lines() {
        let function = line.split(": ").collect::<Vec<&str>>();
        let result = function[0].parse::<u64>().unwrap();
        let operations = function[1]
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let op_comb = generate_operation_combinations(operations.len() - 1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
