use std::collections::HashMap;

advent_of_code::solution!(19);

fn get_towels(input: &str) -> Vec<String> {
    let block: &str = input.lines().next().unwrap();
    block.split(", ").map(|line| line.to_string()).collect()
}

fn get_designs(input: &str) -> Vec<String> {
    let designs: Vec<String> = input.lines().skip(2).map(|line| line.to_string()).collect();
    designs
}

fn valid_combinations(
    design: &String,
    towels: &Vec<String>,
    memory: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(count) = memory.get(design) {
        return *count;
    }

    if design.is_empty() {
        return 1;
    }

    let mut count = 0;
    for towel in towels.iter() {
        if design.starts_with(towel) {
            count += valid_combinations(&(design[towel.len()..]).to_string(), towels, memory);
        }
    }

    *memory.entry(design.to_string()).or_insert(0) += count;
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let towels = get_towels(input);
    let designs = get_designs(input);

    let mut total = 0;
    let mut memory: HashMap<String, u64> = HashMap::new();
    for design in designs.iter() {
        if valid_combinations(design, &towels, &mut memory) > 0 {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let towels = get_towels(input);
    let designs = get_designs(input);

    let mut total = 0;
    let mut memory: HashMap<String, u64> = HashMap::new();
    for design in designs.iter() {
        total += valid_combinations(design, &towels, &mut memory);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
