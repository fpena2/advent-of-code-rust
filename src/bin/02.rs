advent_of_code::solution!(2);

pub fn is_safe(levels: Vec<i32>) -> bool {
    let mut prev_cal: i32 = 0;
    for pos in 0..levels.len() - 1 {
        let cal = levels[pos] - levels[pos + 1];

        if cal.abs() > 3 || cal == 0 {
            return false;
        }

        if (prev_cal.is_negative() && cal.is_positive())
            || (prev_cal.is_positive() && cal.is_negative())
        {
            if pos == 0 {
                continue;
            }

            return false;
        }

        prev_cal = cal;
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_levels = 0;
    for line in input.lines() {
        let level: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(level) {
            safe_levels += 1;
        }
    }

    Some(safe_levels)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_levels = 0;
    for line in input.lines() {
        let level: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(level.clone()) {
            safe_levels += 1;
        } else {
            let mut test = level.clone();
            for i in 0..test.len() {
                test.remove(i);
                if is_safe(test.clone()) {
                    safe_levels += 1;
                    break;
                }
                test = level.clone();
            }
        }
    }

    Some(safe_levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
