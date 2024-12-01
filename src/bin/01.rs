advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in input.lines() {
        let pair: Vec<u32> = line.split("   ").map(|v| v.parse().unwrap()).collect();
        left.push(pair[0]);
        right.push(pair[1]);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for (l, r) in std::iter::zip(left, right) {
        sum += l.abs_diff(r);
    }

    Some(sum)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
