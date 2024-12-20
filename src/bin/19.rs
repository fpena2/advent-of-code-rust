use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Design(String);
struct Towel(String);
struct AvailableTowels(Vec<Towel>);
#[derive(Debug)]
struct Designs(Vec<Design>);

impl Design {
    fn count_ways_to_construct(
        &self,
        towels: &AvailableTowels,
        memory: &mut HashMap<Design, u64>,
    ) -> u64 {
        if let Some(count) = memory.get(&self) {
            return *count;
        }

        if self.0.is_empty() {
            return 1;
        }

        let mut count = 0;
        for towel in towels.0.iter() {
            if self.0.starts_with(&towel.0) {
                // Create a new Design with the remaining substring
                let remaining = &self.0[towel.0.len()..];
                let new_design = Design(remaining.to_string());
                count += new_design.count_ways_to_construct(towels, memory);
            }
        }

        *memory.entry(self.clone()).or_insert(0) += count;
        count
    }
}

impl TryFrom<&str> for Designs {
    type Error = &'static str;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let vec: Vec<String> = input.lines().skip(2).map(|line| line.to_string()).collect();
        if vec.len() > 0 {
            Ok(Designs(vec.iter().map(|a| Design(a.to_string())).collect()))
        } else {
            Err("Invalid input")
        }
    }
}

impl TryFrom<&str> for AvailableTowels {
    type Error = &'static str;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if let Some(block) = input.lines().next() {
            let vec = block
                .split(", ")
                .map(|line| Towel(line.to_string()))
                .collect();
            Ok(AvailableTowels(vec))
        } else {
            Err("Invalid input")
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let available_towels = AvailableTowels::try_from(input).ok()?;
    let designs = Designs::try_from(input).ok()?;

    let mut memory: HashMap<Design, u64> = HashMap::new();
    let valid_design_count = designs
        .0
        .iter()
        .filter(|d| d.count_ways_to_construct(&available_towels, &mut memory) > 0)
        .count();

    Some(valid_design_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let available_towels = AvailableTowels::try_from(input).ok()?;
    let designs = Designs::try_from(input).ok()?;

    let mut memory: HashMap<Design, u64> = HashMap::new();
    let total: u64 = designs
        .0
        .iter()
        .map(|d| d.count_ways_to_construct(&available_towels, &mut memory))
        .sum();

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
